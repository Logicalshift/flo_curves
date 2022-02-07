use super::{GraphEdge, GraphEdgeRef, GraphPath, GraphPathEdge, GraphPathPoint};
use crate::bezier::curve::{BezierCurve, BezierCurveFactory, Curve};
use crate::bezier::intersection::{curve_intersects_curve_clip, find_self_intersection_point};
use crate::consts::{CLOSE_DISTANCE, SMALL_T_DISTANCE};
use crate::geo::{
    sweep_against, sweep_self, BoundingBox, Bounds, Coordinate, Coordinate2D, Geo, HasBoundingBox,
};

use smallvec::{smallvec, SmallVec};

use std::cmp::Ordering;
use std::mem;
use std::ops::Range;

///
/// Struct describing a collision between two edges
///
#[derive(Clone, Copy, Debug)]
struct Collision {
    /// The first edge in the collision
    edge_1: GraphEdgeRef,

    /// The second edge in the collision
    edge_2: GraphEdgeRef,

    /// The location on edge1 of the collision
    edge_1_t: f64,

    /// The location on edge2 of the collision
    edge_2_t: f64,
}

impl<Point: Coordinate + Coordinate2D, Label: Copy> GraphPath<Point, Label> {
    ///
    /// True if the t value is effectively at the start of the curve
    ///
    #[inline]
    fn t_is_zero(t: f64) -> bool {
        t <= 0.0
    }

    ///
    /// True if the t value is effective at the end of the curve
    ///
    #[inline]
    fn t_is_one(t: f64) -> bool {
        t >= 1.0
    }

    ///
    /// Retrieves the ordered graph edges for a range of points
    ///
    fn get_ordered_edges(&self, points: Range<usize>) -> Vec<GraphEdge<Point, Label>> {
        let mut ordered_edges = points
            .into_iter()
            .flat_map(|point_idx| {
                (0..self.points[point_idx].forward_edges.len())
                    .into_iter()
                    .map(move |edge_idx| (point_idx, edge_idx))
            })
            .map(|(point_idx, edge_idx)| GraphEdgeRef {
                start_idx: point_idx,
                edge_idx,
                reverse: false,
            })
            .map(|edge_ref| GraphEdge::new(self, edge_ref))
            .collect::<Vec<_>>();

        ordered_edges.sort_by(|edge1, edge2| {
            let bb1 = edge1.get_bounding_box::<Bounds<_>>();
            let bb2 = edge2.get_bounding_box::<Bounds<_>>();

            bb1.min()
                .x()
                .partial_cmp(&bb2.min().x())
                .unwrap_or(Ordering::Equal)
        });

        ordered_edges
    }

    ///
    /// Returns the 'snapped' version of two points when they're close enough
    ///
    #[inline]
    fn snap_points(p1: &Point, p2: &Point) -> Point {
        Point::from_components(&[(p1.x() + p2.x()) / 2.0, (p1.y() + p2.y()) / 2.0])
    }

    ///
    /// True if points p1 and p2 are near to each other
    ///
    #[inline]
    fn point_is_near(p1: &Point, p2: &Point, max_distance_squared: f64) -> bool {
        let offset = *p1 - *p2;
        let squared_distance = offset.dot(&offset);

        squared_distance <= max_distance_squared
    }

    ///
    /// Finds the self collisions in a range
    ///
    fn find_self_collisions(&self, points: Range<usize>, accuracy: f64) -> Vec<Collision> {
        // Sort the edges into min_x order
        let ordered_edges = self.get_ordered_edges(points);

        // Find the collisions
        let mut collisions = vec![];

        for (src_curve, tgt_curve) in sweep_self(ordered_edges.iter()) {
            // Find any collisions between the two edges (to the required accuracy)
            let mut edge_collisions = curve_intersects_curve_clip(src_curve, tgt_curve, accuracy);
            if edge_collisions.is_empty() {
                continue;
            }

            // Remove any pairs of collisions that are too close together
            remove_and_round_close_collisions(&mut edge_collisions, src_curve, tgt_curve);

            // Turn into collisions, filtering out the collisions that occur at the ends (where one edge joins another).
            // For cases where we get a collision at the end of an edge, wait for the one at the beginning of the next one
            let edge_collisions = edge_collisions
                .into_iter()
                .filter(|(src_t, tgt_t)| {
                    !(Self::t_is_one(*src_t)
                        || Self::t_is_one(*tgt_t)
                        || (Self::t_is_zero(*src_t) && Self::t_is_zero(*tgt_t)))
                })
                .map(|(src_t, tgt_t)| Collision {
                    edge_1: src_curve.edge,
                    edge_2: tgt_curve.edge,
                    edge_1_t: src_t,
                    edge_2_t: tgt_t,
                })
                .map(|mut collision| {
                    // If the collision is at the end of the edge, move it to the start of the following edge
                    if Self::t_is_one(collision.edge_1_t) {
                        collision.edge_1 = self.following_edge_ref(collision.edge_1);
                        collision.edge_1_t = 0.0;
                    }

                    if Self::t_is_one(collision.edge_2_t) {
                        collision.edge_2 = self.following_edge_ref(collision.edge_2);
                        collision.edge_2_t = 0.0;
                    }

                    collision
                });

            // Add to the results
            collisions.extend(edge_collisions);
        }

        // Check all edges for self-collisions
        for edge in ordered_edges {
            // Colliding edge against itself
            if let Some((t1, t2)) = find_self_intersection_point(&edge, accuracy) {
                if !(t1 <= 0.0 && t2 >= 1.0 || t1 >= 1.0 && t2 <= 0.0) {
                    collisions.push(Collision {
                        edge_1: edge.edge,
                        edge_2: edge.edge,
                        edge_1_t: t1,
                        edge_2_t: t2,
                    });
                }
            }
        }

        collisions
    }

    ///
    /// Finds any collisions that might exist between two ranges of points
    ///
    fn find_collisions(
        &self,
        collide_from: Range<usize>,
        collide_to: Range<usize>,
        accuracy: f64,
    ) -> Vec<Collision> {
        if collide_from == collide_to {
            return self.find_self_collisions(collide_from, accuracy);
        }

        // Order the edges for the two sides that are going to be collided
        let collide_src = self.get_ordered_edges(collide_from);
        let collide_tgt = self.get_ordered_edges(collide_to);

        // Perform a sweep to find any collisions
        let mut collisions = vec![];

        for (src_curve, tgt_curve) in sweep_against(collide_src.iter(), collide_tgt.iter()) {
            // Find any collisions between the two edges (to the required accuracy)
            let mut edge_collisions = curve_intersects_curve_clip(src_curve, tgt_curve, accuracy);
            if edge_collisions.is_empty() {
                continue;
            }

            // Remove any pairs of collisions that are too close together
            remove_and_round_close_collisions(&mut edge_collisions, src_curve, tgt_curve);

            // Turn into collisions, filtering out the collisions that occur at the ends (where one edge joins another).
            // For cases where we get a collision at the end of an edge, wait for the one at the beginning of the next one
            let edge_collisions = edge_collisions
                .into_iter()
                .filter(|(src_t, tgt_t)| {
                    !(Self::t_is_one(*src_t)
                        || Self::t_is_one(*tgt_t)
                        || (Self::t_is_zero(*src_t) && Self::t_is_zero(*tgt_t)))
                })
                .map(|(src_t, tgt_t)| Collision {
                    edge_1: src_curve.edge,
                    edge_2: tgt_curve.edge,
                    edge_1_t: src_t,
                    edge_2_t: tgt_t,
                })
                .map(|mut collision| {
                    // If the collision is at the end of the edge, move it to the start of the following edge
                    if Self::t_is_one(collision.edge_1_t) {
                        collision.edge_1 = self.following_edge_ref(collision.edge_1);
                        collision.edge_1_t = 0.0;
                    }

                    if Self::t_is_one(collision.edge_2_t) {
                        collision.edge_2 = self.following_edge_ref(collision.edge_2);
                        collision.edge_2_t = 0.0;
                    }

                    collision
                });

            // Add to the results
            collisions.extend(edge_collisions);
        }

        collisions
    }

    ///
    /// Adds any new points that will be required to divide the edges with the specified set of collisions
    ///
    fn create_collision_points(&mut self, collisions: Vec<Collision>) -> Vec<(Collision, usize)> {
        // Create new points for each collision
        let mut collision_points = vec![];
        collision_points.reserve(collisions.len());

        for collision in collisions.into_iter() {
            // Determine the index of the point where this collision occurs is
            let point_idx = if Self::t_is_zero(collision.edge_1_t) {
                // Re-use the existing start point for edge1
                collision.edge_1.start_idx
            } else if Self::t_is_zero(collision.edge_2_t) {
                // Re-use the existing start point for edge2
                collision.edge_2.start_idx
            } else {
                // Create a new point
                let edge = self.get_edge(collision.edge_1);
                let new_point_pos = edge.point_at_pos(collision.edge_1_t);
                let new_point_idx = self.points.len();

                self.points.push(GraphPathPoint {
                    position: new_point_pos,
                    forward_edges: smallvec![],
                    connected_from: smallvec![],
                });

                new_point_idx
            };

            // Store in the list of collision points
            collision_points.push((collision, point_idx));
        }

        collision_points
    }

    ///
    /// Given a list of collisions and the point where they end, organizes them by edge
    ///
    /// Return type is a vector of edges for each point, where each edge is a list of collisions, as 't' value on the edge and the
    /// index of the end point
    ///
    fn organize_collisions_by_edge(
        &self,
        collisions: Vec<(Collision, usize)>,
    ) -> Vec<Option<SmallVec<[SmallVec<[(f64, usize); 2]>; 2]>>> {
        // Initially there are no collisions for any point
        let mut points: Vec<Option<SmallVec<[SmallVec<[(f64, usize); 2]>; 2]>>> =
            vec![None; self.num_points()];

        // Iterate through the collisions and store them per edge. Every collision affects two edges
        for (collision, end_point_idx) in collisions.iter() {
            // First edge
            let point   = points[collision.edge_1.start_idx].get_or_insert_with(|| smallvec![smallvec![]; self.points[collision.edge_1.start_idx].forward_edges.len()]);
            let edge = &mut point[collision.edge_1.edge_idx];

            edge.push((collision.edge_1_t, *end_point_idx));

            // Second edge
            let point   = points[collision.edge_2.start_idx].get_or_insert_with(|| smallvec![smallvec![]; self.points[collision.edge_2.start_idx].forward_edges.len()]);
            let edge = &mut point[collision.edge_2.edge_idx];

            edge.push((collision.edge_2_t, *end_point_idx));
        }

        points
    }

    ///
    /// Searches two ranges of points in this object and detects collisions between them, subdividing the edges
    /// and creating branch points at the appropriate places.
    ///
    /// collide_from must indicate indices lower than collide_to
    ///
    /// Returns true if any collisions were found
    ///
    pub(crate) fn detect_collisions(
        &mut self,
        collide_from: Range<usize>,
        collide_to: Range<usize>,
        accuracy: f64,
    ) -> bool {
        // Find all of the collision points
        let all_collisions = self.find_collisions(collide_from, collide_to, accuracy);
        if all_collisions.is_empty() {
            let collided_at_point = self.combine_overlapping_points(accuracy);
            self.remove_all_very_short_edges();
            return collided_at_point;
        }

        // Add in any extra points that are required by the collisions we found
        let all_collisions = self.create_collision_points(all_collisions);

        // Organize the collisions by edge
        let collisions_by_edge = self.organize_collisions_by_edge(all_collisions);

        // Limit to just points with collisions
        let collisions_by_point =
            collisions_by_edge
                .into_iter()
                .enumerate()
                .filter_map(|(point_idx, collisions)| {
                    collisions.map(|collisions| (point_idx, collisions))
                });

        // Actually divide the edges by collision
        for (point_idx, edge_collisions) in collisions_by_point {
            for (edge_idx, mut collisions) in edge_collisions.into_iter().enumerate() {
                // Skip edges with no collisions
                if collisions.is_empty() {
                    continue;
                }

                self.check_following_edge_consistency();

                // Create a copy of the edge. Our future edges will all have the same kind and label as the edge that's being divided
                let edge = self.get_edge(GraphEdgeRef {
                    start_idx: point_idx,
                    edge_idx,
                    reverse: false,
                });
                let kind = edge.kind();
                let label = edge.label();
                let edge = Curve::from_curve(&edge);

                // Sort collisions by t value
                collisions.sort_by(|(t1, _end_point_idx1), (t2, _end_point_idx2)| {
                    if t1 < t2 {
                        Ordering::Less
                    } else if t1 > t2 {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                // We'll progressively split bits from the edge
                let mut remaining_edge = edge;
                let mut remaining_t = 1.0;
                let final_point_idx = self.points[point_idx].forward_edges[edge_idx].end_idx;
                let final_following_edge_idx =
                    self.points[point_idx].forward_edges[edge_idx].following_edge_idx;
                let mut last_point_idx = point_idx;
                let mut previous_edge = None;
                let mut found_collisions = false;

                // Iterate through the collisions (skipping any at t=0)
                let mut collisions = collisions.into_iter().filter(|(t, _)| !Self::t_is_zero(*t));

                // First collision is special as we need to edit the existing edge instead of adding a new one
                if let Some((t, end_point_idx)) = collisions.next() {
                    // Subdivide the edge
                    let (next_edge, new_remaining_edge) = remaining_edge.subdivide::<Curve<_>>(t);
                    let following_edge_idx = self.points[end_point_idx].forward_edges.len();
                    let (cp1, cp2) = next_edge.control_points();

                    test_assert!(next_edge
                        .start_point()
                        .is_near_to(&self.points[point_idx].position, 0.1));
                    test_assert!(next_edge
                        .end_point()
                        .is_near_to(&self.points[end_point_idx].position, 0.1));

                    // Update the control points and end point index
                    let old_edge = &mut self.points[point_idx].forward_edges[edge_idx];

                    old_edge.cp1 = cp1;
                    old_edge.cp2 = cp2;
                    old_edge.end_idx = end_point_idx;
                    old_edge.following_edge_idx = following_edge_idx;
                    old_edge.invalidate_cache();

                    // Move on to the next edge
                    previous_edge = Some((point_idx, edge_idx));
                    remaining_t = 1.0 - t;
                    remaining_edge = new_remaining_edge;
                    last_point_idx = end_point_idx;
                    found_collisions = true;
                }

                // Deal with the rest of the collisions
                for (t, end_point_idx) in collisions {
                    // Point the previous edge at the new edge we're adding
                    let new_edge_idx = self.points[last_point_idx].forward_edges.len();
                    if let Some((point_idx, edge_idx)) = previous_edge {
                        self.points[point_idx].forward_edges[edge_idx].following_edge_idx =
                            new_edge_idx
                    }

                    // Subdivide the remaining edge
                    let t2 = (t - (1.0 - remaining_t)) / remaining_t;
                    let (next_edge, new_remaining_edge) = remaining_edge.subdivide::<Curve<_>>(t2);
                    let (cp1, cp2) = next_edge.control_points();

                    test_assert!(next_edge
                        .start_point()
                        .is_near_to(&self.points[last_point_idx].position, 0.1));
                    test_assert!(next_edge
                        .end_point()
                        .is_near_to(&self.points[end_point_idx].position, 0.1));

                    // Add the new edge to the previous point
                    let new_edge = GraphPathEdge::new(kind, (cp1, cp2), end_point_idx, label, 0);
                    self.points[last_point_idx].forward_edges.push(new_edge);

                    // Move on to the next edge
                    previous_edge = Some((last_point_idx, new_edge_idx));
                    remaining_t = 1.0 - t;
                    remaining_edge = new_remaining_edge;
                    last_point_idx = end_point_idx;
                    found_collisions = true;
                }

                // Provided there was at least one collision (ie, not just one at t=0), add the final edge
                if found_collisions {
                    // Point the previous edge at the new edge we're adding
                    let new_edge_idx = self.points[last_point_idx].forward_edges.len();
                    if let Some((point_idx, edge_idx)) = previous_edge {
                        self.points[point_idx].forward_edges[edge_idx].following_edge_idx =
                            new_edge_idx
                    }

                    // This edge ends where the original edge ended
                    let end_point_idx = final_point_idx;
                    let following_edge_idx = final_following_edge_idx;
                    let (cp1, cp2) = remaining_edge.control_points();

                    test_assert!(remaining_edge
                        .start_point()
                        .is_near_to(&self.points[last_point_idx].position, 0.1));
                    test_assert!(remaining_edge
                        .end_point()
                        .is_near_to(&self.points[end_point_idx].position, 0.1));

                    // Add to the final point
                    let final_edge = GraphPathEdge::new(
                        kind,
                        (cp1, cp2),
                        end_point_idx,
                        label,
                        following_edge_idx,
                    );
                    self.points[last_point_idx].forward_edges.push(final_edge);
                }
            }
        }

        // Finish up by checking that we haven't broken consistency
        self.check_following_edge_consistency();

        self.recalculate_reverse_connections();
        self.combine_overlapping_points(accuracy);
        self.remove_all_very_short_edges();

        self.check_following_edge_consistency();

        true
    }

    ///
    /// Finds points that are within accuracy distance of each other (for accuracy < 1.0)
    ///
    /// Return value is a list of nearby points
    ///
    fn sweep_for_nearby_points(&mut self, accuracy: f64) -> impl Iterator<Item = (usize, usize)> {
        // Structure to attach a bounding box to a point within this graph: this limits us as to the maximum distance we can use as it's used for sweeping
        struct PointArea<'a, Point, Label>(&'a GraphPath<Point, Label>, usize);

        impl<'a, Point: Coordinate + Coordinate2D, Label> PointArea<'a, Point, Label> {
            #[inline]
            fn pos(&self) -> &Point {
                let PointArea(graph, point_idx) = self;

                &graph.points[*point_idx].position
            }

            #[inline]
            fn idx(&self) -> usize {
                let PointArea(_graph, point_idx) = self;

                *point_idx
            }
        }

        impl<'a, Point: Coordinate + Coordinate2D, Label> Geo for PointArea<'a, Point, Label> {
            type Point = Point;
        }

        impl<'a, Point: Coordinate + Coordinate2D, Label> HasBoundingBox for PointArea<'a, Point, Label> {
            fn get_bounding_box<Bounds: BoundingBox<Point = Self::Point>>(&self) -> Bounds {
                let PointArea(graph, point_idx) = self;

                let point = &graph.points[*point_idx];
                let lower =
                    Point::from_components(&[point.position.x() - 1.0, point.position.y() - 1.0]);
                let upper =
                    Point::from_components(&[point.position.x() + 1.0, point.position.y() + 1.0]);

                Bounds::from_min_max(lower, upper)
            }
        }

        // Collect all of the points in the graph, and order them by min_x
        let mut all_points = (0..self.points.len())
            .into_iter()
            .map(|idx| PointArea(self, idx))
            .collect::<Vec<_>>();
        all_points.sort_by(|point1, point2| {
            let x1 = point1.pos().x();
            let x2 = point2.pos().x();

            x1.partial_cmp(&x2).unwrap_or(Ordering::Equal)
        });

        // Sweep to find the points that might be colliding
        let min_distance_squared = accuracy * accuracy;
        let colliding_points = sweep_self(all_points.iter()).filter(|(point1, point2)| {
            if point1.idx() == point2.idx() {
                // A point cannot overlap itself
                false
            } else {
                // Work out the distances between the points and
                let p1 = point1.pos();
                let p2 = point2.pos();

                let (x1, y1) = (p1.x(), p1.y());
                let (x2, y2) = (p2.x(), p2.y());
                let (dx, dy) = (x2 - x1, y2 - y1);

                let distance_squared = dx * dx + dy * dy;

                distance_squared < min_distance_squared
            }
        });

        // Result is the indexes of the points that are 'close enough' to collide
        colliding_points
            .map(|(point1, point2)| (point1.idx(), point2.idx()))
            .collect::<Vec<_>>()
            .into_iter()
    }

    ///
    /// Finds any points that have approximately the same coordinates and combines them
    ///
    /// Accuracy indicates the maximum difference in the x or y coordinate for two points to be considered the same.
    ///
    #[inline(never)]
    pub fn combine_overlapping_points(&mut self, accuracy: f64) -> bool {
        // Move any points that are connected by an edge and very close to each other on top of each other
        for point_idx in 0..self.points.len() {
            for edge_idx in 0..(self.points[point_idx].forward_edges.len()) {
                let end_point_idx = self.points[point_idx].forward_edges[edge_idx].end_idx;
                if end_point_idx == point_idx {
                    // A point is always close to itself, so we don't want to try to move it in this case
                    continue;
                }

                let start_point = &self.points[point_idx].position;
                let end_point = &self.points[end_point_idx].position;

                if start_point.is_near_to(end_point, accuracy) {
                    self.points[end_point_idx].position = self.points[point_idx].position;
                }
            }
        }

        // Find the points that are close enough to collide
        let mut nearby_points = self.sweep_for_nearby_points(accuracy);

        if let Some(nearby_point) = nearby_points.next() {
            // Remap points according to whatever is nearest
            let min_distance_squared = accuracy * accuracy;
            let mut remapped_points = (0..self.points.len())
                .into_iter()
                .map(|idx| (idx, None)) // Target index (= point index if not remapped and new position, or None if unmoved)
                .collect::<Vec<(_, Option<Point>)>>();

            let mut nearby_point = nearby_point;
            loop {
                // Index is of two points that are close enough to overlap
                let (p1_orig_idx, p2_orig_idx) = nearby_point;
                debug_assert!(p1_orig_idx != p2_orig_idx); // Guaranteed by the implementation of sweep_for_nearby_points()

                // Point may be remapped
                let (p1_idx, p1_pos) = &remapped_points[p1_orig_idx];
                let (p2_idx, p2_pos) = &remapped_points[p2_orig_idx];

                // To prevent averaging a whole bunch of points down to the same point because they're all close together, we re-check the distance if the one of the two close points has already been remapped
                let moved = p1_pos.is_some() || p2_pos.is_some();

                let p1_pos = if let Some(pos) = p1_pos {
                    *pos
                } else {
                    self.points[*p1_idx].position
                };
                let p2_pos = if let Some(pos) = p2_pos {
                    *pos
                } else {
                    self.points[*p2_idx].position
                };

                if !moved || Self::point_is_near(&p1_pos, &p2_pos, min_distance_squared) {
                    // Remap both points to a common target position
                    let pos = Self::snap_points(&p1_pos, &p2_pos);
                    let remap_idx = usize::min(*p1_idx, *p2_idx);

                    remapped_points[p1_orig_idx] = (remap_idx, Some(pos));
                    remapped_points[p2_orig_idx] = (remap_idx, Some(pos));
                }

                // Fetch the next point or stop
                if let Some(next_point) = nearby_points.next() {
                    nearby_point = next_point;
                } else {
                    break;
                }
            }

            // Remap every point and the edges (we can tell remapped points by looking at the new position)
            let mut following_edge_idx_offset = vec![0; self.points.len()];

            for original_idx in 0..self.points.len() {
                if let (new_idx, Some(new_pos)) = &remapped_points[original_idx] {
                    // This point has been moved
                    self.points[original_idx].position = *new_pos;

                    // If this is the target point, then don't move any edges
                    if *new_idx == original_idx {
                        continue;
                    }

                    // Trace the new index to its final point (which is the point still mapped to itself: this should always exist because we always prefer the lowest point)
                    let mut new_idx = *new_idx;
                    loop {
                        let (next_idx, _) = &remapped_points[new_idx];
                        let next_idx = *next_idx;

                        if next_idx == new_idx {
                            break;
                        } else {
                            remapped_points[original_idx].0 = next_idx;
                            new_idx = next_idx;
                        }
                    }

                    // Move the edges into the new index
                    let forward_edges = mem::take(&mut self.points[original_idx].forward_edges);
                    let connected_from = mem::take(&mut self.points[original_idx].connected_from);

                    following_edge_idx_offset[original_idx] =
                        self.points[new_idx].forward_edges.len();

                    self.points[new_idx]
                        .forward_edges
                        .extend(forward_edges.into_iter());
                    self.points[new_idx]
                        .connected_from
                        .extend(connected_from.into_iter());
                }
            }

            // Remap the target points (we should no longer need to follow points to the end as )
            for point in self.points.iter_mut() {
                // Remap the edges
                for edge in point.forward_edges.iter_mut() {
                    let new_end_idx = remapped_points[edge.end_idx].0;

                    if new_end_idx != edge.end_idx {
                        let following_edge_idx_offset = following_edge_idx_offset[edge.end_idx];

                        edge.end_idx = new_end_idx;
                        edge.following_edge_idx += following_edge_idx_offset;
                    }
                }

                // Remap the 'connected from' points
                let mut remapped = false;
                for connected_from_idx in point.connected_from.iter_mut() {
                    let new_connected_from_idx = remapped_points[*connected_from_idx].0;

                    if new_connected_from_idx != *connected_from_idx {
                        *connected_from_idx = new_connected_from_idx;
                        remapped = true;
                    }
                }

                // If we introduced duplicates, remove them
                if remapped {
                    point.connected_from.sort_unstable();
                    point.connected_from.dedup();
                }
            }

            true
        } else {
            // No overlap
            false
        }
    }

    ///
    /// Checks that the following edges are consistent
    ///
    #[cfg(any(test, extra_checks))]
    pub(crate) fn check_following_edge_consistency(&self) {
        let mut used_edges = vec![vec![]; self.points.len()];

        for point_idx in 0..(self.points.len()) {
            let point = &self.points[point_idx];

            for edge_idx in 0..(point.forward_edges.len()) {
                let edge = &point.forward_edges[edge_idx];

                test_assert!(edge.end_idx < self.points.len());
                test_assert!(
                    edge.following_edge_idx < self.points[edge.end_idx].forward_edges.len()
                );
                test_assert!(!used_edges[edge.end_idx].contains(&edge.following_edge_idx));

                used_edges[edge.end_idx].push(edge.following_edge_idx);
            }
        }
    }

    #[cfg(not(any(test, extra_checks)))]
    pub(crate) fn check_following_edge_consistency(&self) {}
}

///
/// Removes any pairs of collisions that are closer than `CLOSE_DISTANCE` apart, and also rounds the
/// first and last collisions to 0.0 and 1.0
///
/// When colliding two bezier curves we want to avoid subdividing excessively to produce very small
/// sections as they have a tendency to produce extra collisions due to floating point or root finding
/// errors.
///
fn remove_and_round_close_collisions<C: BezierCurve>(
    collisions: &mut SmallVec<[(f64, f64); 8]>,
    src: &C,
    tgt: &C,
) where
    C::Point: Coordinate + Coordinate2D,
{
    // Nothing to do if there are no collisions
    if collisions.is_empty() {
        return;
    }

    // Work out the positions of each point
    let mut positions = collisions
        .iter()
        .map(|(t1, _t2)| src.point_at_pos(*t1))
        .collect::<Vec<_>>();

    // Find any pairs of points that are too close together
    let mut collision_idx = 0;
    while collision_idx + 1 < collisions.len() {
        // Just remove both of these if they are too close together (as each collision crosses the curve once, removing collisions in pairs means that there'll still be at least one collision left if the curves actually end up crossing over)
        if positions[collision_idx].is_near_to(&positions[collision_idx + 1], CLOSE_DISTANCE) {
            if (collisions[collision_idx].0 - collisions[collision_idx + 1].0).abs()
                < SMALL_T_DISTANCE
                && (collisions[collision_idx].1 - collisions[collision_idx + 1].1).abs()
                    < SMALL_T_DISTANCE
            {
                collisions.remove(collision_idx);
                positions.remove(collision_idx);
                collisions.remove(collision_idx);
                positions.remove(collision_idx);
            } else {
                collision_idx += 1;
            }
        } else {
            collision_idx += 1;
        }
    }

    // If the first point or the last point is close to the end of the source or target curve, clip to 0 or 1
    if !collisions.is_empty() {
        // Get the start/end points of the source and target
        let src_start = src.start_point();
        let src_end = src.end_point();
        let tgt_start = tgt.start_point();
        let tgt_end = tgt.end_point();

        // Snap collisions to 0.0 or 1.0 if they're very close to the start or end of either curve
        for collision_idx in 0..collisions.len() {
            // Snap the source side
            if collisions[collision_idx].0 > 0.0 && collisions[collision_idx].0 < 1.0 {
                if src_start.is_near_to(&positions[collision_idx], CLOSE_DISTANCE)
                    && collisions[collision_idx].0 < SMALL_T_DISTANCE
                {
                    collisions[collision_idx].0 = 0.0;
                }

                if src_end.is_near_to(&positions[collision_idx], CLOSE_DISTANCE)
                    && collisions[collision_idx].0 > 1.0 - SMALL_T_DISTANCE
                {
                    collisions[collision_idx].0 = 1.0;
                }
            }

            // Snap the target side
            if collisions[collision_idx].1 > 0.0
                && collisions[collision_idx].1 < 1.0
                && collisions[collision_idx].1 < SMALL_T_DISTANCE
            {
                if tgt_start.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) {
                    collisions[collision_idx].1 = 0.0;
                }

                if tgt_end.is_near_to(&positions[collision_idx], CLOSE_DISTANCE)
                    && collisions[collision_idx].1 > 1.0 - SMALL_T_DISTANCE
                {
                    collisions[collision_idx].1 = 1.0;
                }
            }
        }
    }
}
