use super::{GraphPath, GraphEdge, GraphEdgeRef, GraphPathPoint, GraphPathEdge};
use super::super::super::curve::*;
use super::super::super::intersection::*;
use super::super::super::super::geo::*;
use super::super::super::super::consts::*;

use smallvec::*;

use std::mem;
use std::ops::Range;
use std::cmp::Ordering;
use std::collections::HashMap;

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
    edge_2_t: f64
}

impl<Point: Coordinate+Coordinate2D, Label: Copy> GraphPath<Point, Label> {
    /// 
    /// True if the t value is effectively at the start of the curve
    /// 
    #[inline]
    fn t_is_zero(t: f64) -> bool { t <= 0.0 }

    ///
    /// True if the t value is effective at the end of the curve
    /// 
    #[inline]
    fn t_is_one(t: f64) -> bool { t >= 1.0 }

    ///
    /// Finds any collisions that might exist between two ranges of points
    ///
    fn find_collisions(&self, collide_from: Range<usize>, collide_to: Range<usize>, accuracy: f64) -> Vec<Collision> {
        if collide_to.start < collide_from.start {
            // collide_from must always start at a lower index
            self.find_collisions(collide_to, collide_from, accuracy)
        } else {
            // Start creating the list of collisions
            let mut collisions = vec![];

            // Iterate through all of the collide_from points
            for src_idx in collide_from {
                // Do not re-check edges that we've already visited
                let collide_to      = (collide_to.start.max(src_idx))..collide_to.end;

                // Search all of the edges at this index
                for src_edge_idx in 0..self.points[src_idx].forward_edges.len() {
                    // We can quickly eliminate edges that are outside the bounds
                    let src_curve_ref   = GraphEdgeRef { start_idx: src_idx, edge_idx: src_edge_idx, reverse: false };
                    let src_curve       = GraphEdge::new(self, src_curve_ref);
                    let src_edge_bounds = src_curve.fast_bounding_box::<Bounds<_>>();

                    // Collide against the target edges
                    for tgt_idx in collide_to.clone() {
                        for tgt_edge_idx in 0..self.points[tgt_idx].forward_edges.len() {
                            // Search for loops when colliding an edge against itself
                            if src_idx == tgt_idx {
                                if src_edge_idx == tgt_edge_idx { 
                                    // Colliding edge against itself
                                    if let Some((t1, t2)) = find_self_intersection_point(&src_curve, accuracy) {
                                        if !(t1 <= 0.0 && t2 >= 1.0) && !(t1 >= 1.0 && t2 <= 0.0) {
                                            collisions.push(Collision {
                                                edge_1:     src_curve_ref,
                                                edge_2:     src_curve_ref,
                                                edge_1_t:   t1,
                                                edge_2_t:   t2
                                            });
                                        }
                                    }

                                    continue;
                                } else if src_edge_idx > tgt_edge_idx {
                                    // Will already have collided this edge elsewhere
                                    continue; 
                                }
                            }

                            // Avoid trying to collide two curves whose bounding boxes do not overlap
                            let tgt_curve_ref   = GraphEdgeRef { start_idx: tgt_idx, edge_idx: tgt_edge_idx, reverse: false };
                            let tgt_curve       = GraphEdge::new(self, tgt_curve_ref);
                            
                            let tgt_edge_bounds = tgt_curve.fast_bounding_box::<Bounds<_>>();
                            if !src_edge_bounds.overlaps(&tgt_edge_bounds) { continue; }

                            // Find any collisions between the two edges (to the required accuracy)
                            let mut edge_collisions = curve_intersects_curve_clip(&src_curve, &tgt_curve, accuracy);
                            if edge_collisions.len() == 0 { continue; }

                            // Remove any pairs of collisions that are too close together
                            remove_and_round_close_collisions(&mut edge_collisions, &src_curve, &tgt_curve);

                            // Turn into collisions, filtering out the collisions that occur at the ends (where one edge joins another).
                            // For cases where we get a collision at the end of an edge, wait for the one at the beginning of the next one
                            let edge_collisions = edge_collisions.into_iter()
                                .filter(|(src_t, tgt_t)| !(Self::t_is_one(*src_t) || Self::t_is_one(*tgt_t) || (Self::t_is_zero(*src_t) && Self::t_is_zero(*tgt_t))))
                                .map(|(src_t, tgt_t)| {
                                    Collision {
                                        edge_1:     src_curve_ref,
                                        edge_2:     tgt_curve_ref,
                                        edge_1_t:   src_t,
                                        edge_2_t:   tgt_t
                                    }
                                })
                                .map(|mut collision| {
                                    // If the collision is at the end of the edge, move it to the start of the following edge
                                    if Self::t_is_one(collision.edge_1_t) {
                                        collision.edge_1    = self.following_edge_ref(collision.edge_1);
                                        collision.edge_1_t  = 0.0;
                                    }

                                    if Self::t_is_one(collision.edge_2_t) {
                                        collision.edge_2    = self.following_edge_ref(collision.edge_2);
                                        collision.edge_2_t  = 0.0;
                                    }

                                    collision
                                });

                            // Add to the results
                            collisions.extend(edge_collisions);
                        }
                    }
                }
            }

            collisions
        }
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
                let edge            = self.get_edge(collision.edge_1);
                let new_point_pos   = edge.point_at_pos(collision.edge_1_t);
                let new_point_idx   = self.points.len();

                self.points.push(GraphPathPoint {
                    position:       new_point_pos,
                    forward_edges:  smallvec![],
                    connected_from: smallvec![]
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
    fn organize_collisions_by_edge(&self, collisions: Vec<(Collision, usize)>) -> Vec<Option<SmallVec<[SmallVec<[(f64, usize); 2]>; 2]>>> {
        // Initially there are no collisions for any point
        let mut points: Vec<Option<SmallVec<[SmallVec<[(f64, usize); 2]>; 2]>>> = vec![None; self.num_points()];

        // Iterate through the collisions and store them per edge. Every collision affects two edges
        for (collision, end_point_idx) in collisions.iter() {
            // First edge
            let point   = points[collision.edge_1.start_idx].get_or_insert_with(|| smallvec![smallvec![]; self.points[collision.edge_1.start_idx].forward_edges.len()]);
            let edge    = &mut point[collision.edge_1.edge_idx];

            edge.push((collision.edge_1_t, *end_point_idx));

            // Second edge
            let point   = points[collision.edge_2.start_idx].get_or_insert_with(|| smallvec![smallvec![]; self.points[collision.edge_2.start_idx].forward_edges.len()]);
            let edge    = &mut point[collision.edge_2.edge_idx];

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
    pub (crate) fn detect_collisions(&mut self, collide_from: Range<usize>, collide_to: Range<usize>, accuracy: f64) -> bool {
        // Find all of the collision points
        let all_collisions      = self.find_collisions(collide_from, collide_to, accuracy);
        if all_collisions.len() == 0 {
            let collided_at_point = self.combine_overlapping_points(accuracy);
            self.remove_all_very_short_edges();
            return collided_at_point;
        }

        // Add in any extra points that are required by the collisions we found
        let all_collisions      = self.create_collision_points(all_collisions);

        // Organize the collisions by edge
        let collisions_by_edge  = self.organize_collisions_by_edge(all_collisions);

        // Limit to just points with collisions
        let collisions_by_point = collisions_by_edge.into_iter()
            .enumerate()
            .filter_map(|(point_idx, collisions)| collisions.map(|collisions| (point_idx, collisions)));

        // Actually divide the edges by collision
        for (point_idx, edge_collisions) in collisions_by_point {
            for (edge_idx, mut collisions) in edge_collisions.into_iter().enumerate() {
                // Skip edges with no collisions
                if collisions.len() == 0 { continue; }

                self.check_following_edge_consistency();

                // Create a copy of the edge. Our future edges will all have the same kind and label as the edge that's being divided
                let edge    = self.get_edge(GraphEdgeRef { start_idx: point_idx, edge_idx: edge_idx, reverse: false });
                let kind    = edge.kind();
                let label   = edge.label();
                let edge    = Curve::from_curve(&edge);

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
                let mut remaining_edge          = edge;
                let mut remaining_t             = 1.0;
                let final_point_idx             = self.points[point_idx].forward_edges[edge_idx].end_idx;
                let final_following_edge_idx    = self.points[point_idx].forward_edges[edge_idx].following_edge_idx;
                let mut last_point_idx          = point_idx;
                let mut previous_edge           = None;
                let mut found_collisions        = false;

                // Iterate through the collisions (skipping any at t=0)
                let mut collisions      = collisions.into_iter()
                    .filter(|(t, _)| !Self::t_is_zero(*t));

                // First collision is special as we need to edit the existing edge instead of adding a new one
                if let Some((t, end_point_idx)) = collisions.next() {
                    // Subdivide the edge
                    let (next_edge, new_remaining_edge) = remaining_edge.subdivide::<Curve<_>>(t);
                    let following_edge_idx      = self.points[end_point_idx].forward_edges.len();
                    let (cp1, cp2)              = next_edge.control_points();

                    test_assert!(next_edge.start_point().is_near_to(&self.points[point_idx].position, 0.1));
                    test_assert!(next_edge.end_point().is_near_to(&self.points[end_point_idx].position, 0.1));

                    // Update the control points and end point index
                    let old_edge                = &mut self.points[point_idx].forward_edges[edge_idx];

                    old_edge.cp1                = cp1;
                    old_edge.cp2                = cp2;
                    old_edge.end_idx            = end_point_idx;
                    old_edge.following_edge_idx = following_edge_idx;
                    old_edge.invalidate_cache();

                    // Move on to the next edge
                    previous_edge               = Some((point_idx, edge_idx));
                    remaining_t                 = 1.0-t;
                    remaining_edge              = new_remaining_edge;
                    last_point_idx              = end_point_idx;
                    found_collisions            = true;
                }

                // Deal with the rest of the collisions
                for (t, end_point_idx) in collisions {
                    // Point the previous edge at the new edge we're adding
                    let new_edge_idx = self.points[last_point_idx].forward_edges.len();
                    previous_edge.map(|(point_idx, edge_idx)| self.points[point_idx].forward_edges[edge_idx].following_edge_idx = new_edge_idx);

                    // Subdivide the remaining edge
                    let t2                      = (t - (1.0-remaining_t))/remaining_t;
                    let (next_edge, new_remaining_edge) = remaining_edge.subdivide::<Curve<_>>(t2);
                    let (cp1, cp2)              = next_edge.control_points();

                    test_assert!(next_edge.start_point().is_near_to(&self.points[last_point_idx].position, 0.1));
                    test_assert!(next_edge.end_point().is_near_to(&self.points[end_point_idx].position, 0.1));

                    // Add the new edge to the previous point
                    let new_edge                = GraphPathEdge::new(kind, (cp1, cp2), end_point_idx, label, 0);
                    self.points[last_point_idx].forward_edges.push(new_edge);

                    // Move on to the next edge
                    previous_edge               = Some((last_point_idx, new_edge_idx));
                    remaining_t                 = 1.0-t;
                    remaining_edge              = new_remaining_edge;
                    last_point_idx              = end_point_idx;
                    found_collisions            = true;
                }

                // Provided there was at least one collision (ie, not just one at t=0), add the final edge
                if found_collisions {
                    // Point the previous edge at the new edge we're adding
                    let new_edge_idx = self.points[last_point_idx].forward_edges.len();
                    previous_edge.map(|(point_idx, edge_idx)| self.points[point_idx].forward_edges[edge_idx].following_edge_idx = new_edge_idx);

                    // This edge ends where the original edge ended
                    let end_point_idx       = final_point_idx;
                    let following_edge_idx  = final_following_edge_idx;
                    let (cp1, cp2)          = remaining_edge.control_points();

                    test_assert!(remaining_edge.start_point().is_near_to(&self.points[last_point_idx].position, 0.1));
                    test_assert!(remaining_edge.end_point().is_near_to(&self.points[end_point_idx].position, 0.1));

                    // Add to the final point
                    let final_edge          =  GraphPathEdge::new(kind, (cp1, cp2), end_point_idx, label, following_edge_idx);
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
    /// Finds any points that have approximately the same coordinates and combines them
    /// 
    /// Accuracy indicates the maximum difference in the x or y coordinate for two points to be considered the same.
    ///
    #[inline(never)]
    pub fn combine_overlapping_points(&mut self, accuracy: f64) -> bool {
        // Find collisions using a hashmap
        let multiplier      = 1.0 / accuracy;
        let mut collisions  = HashMap::new();

        // Move any points that are connected to an edge and very close to each other on top of each other
        for point_idx in 0..self.points.len() {
            for edge_idx in 0..(self.points[point_idx].forward_edges.len()) {
                let end_point_idx   = self.points[point_idx].forward_edges[edge_idx].end_idx;
                if end_point_idx == point_idx {
                    // A point is always close to itself, so we don't want to try to move it in this case
                    continue;
                }

                let start_point     = &self.points[point_idx].position;
                let end_point       = &self.points[end_point_idx].position;

                if start_point.is_near_to(end_point, accuracy) {
                    self.points[end_point_idx].position = self.points[point_idx].position.clone();
                }
            }
        }

        // Build up a hash set of the possible collisions
        for (point_idx, point) in self.points.iter().enumerate() {
            // Convert the position to an integer using the accuracy value
            let (pos_x, pos_y) = (point.position.x(), point.position.y());
            let (pos_x, pos_y) = (pos_x * multiplier, pos_y * multiplier);
            let (pos_x, pos_y) = (pos_x.round(), pos_y.round());
            let (pos_x, pos_y) = (pos_x as i64, pos_y as i64);

            // Store in the collision hash map
            collisions.entry((pos_x, pos_y))
                .or_insert_with(|| SmallVec::<[_; 2]>::new())
                .push(point_idx);
        }

        // Find the collided points
        let collided_points = collisions.into_iter()
            .filter_map(|(_point, collisions)| {
                if collisions.len() > 1 {
                    Some(collisions)
                } else {
                    None
                }
            });

        // Combine any collided points into a single point
        let mut remapped_points = HashMap::new();
        for collision in collided_points {
            // The first point is the new point (ordering doesn't matter, we just consider these points the same)
            let mut collision   = collision.into_iter();
            let new_point_id    = collision.next().unwrap();

            // Move the first point to the center
            let mut coords: SmallVec<[_; 4]> = smallvec![];
            for idx in 0..(Point::len()) {
                let x = self.points[new_point_id].position.get(idx);
                coords.push((x * multiplier).round() / multiplier);
            }
            self.points[new_point_id].position = Point::from_components(&coords);

            // Move the forward edges and 'connected' from list from the other points
            for collided_with in collision {
                // Add to the remapped hashmap
                remapped_points.insert(collided_with, (new_point_id, self.points[new_point_id].forward_edges.len()));

                // Move the forward edges and connected from values into the original point
                let mut forward_edges   = smallvec![];
                let mut connected_from  = smallvec![];

                mem::swap(&mut self.points[collided_with].forward_edges, &mut forward_edges);
                mem::swap(&mut self.points[collided_with].connected_from, &mut connected_from);

                self.points[new_point_id].forward_edges.extend(forward_edges.into_iter());
                self.points[new_point_id].connected_from.extend(connected_from.into_iter());
            }
        }

        // If any points were remapped, then also remap edges and connected_from values in other points
        if remapped_points.len() > 0 {
            // Need to update all of the points
            for point in self.points.iter_mut() {
                // Remap the edges
                for edge in point.forward_edges.iter_mut() {
                    if let Some((remapped_point, following_edge_idx_offset)) = remapped_points.get(&edge.end_idx) {
                        edge.end_idx            = *remapped_point;
                        edge.following_edge_idx += *following_edge_idx_offset;
                    }
                }

                // Remap the 'connected from' points
                let mut remapped = false;
                for connected_from in point.connected_from.iter_mut() {
                    if let Some((remapped_point, _following_edge_idx_offset)) = remapped_points.get(connected_from) {
                        *connected_from = *remapped_point;
                        remapped = true;
                    }
                }

                // If we introduced duplicates, remove them
                if remapped {
                    point.connected_from.sort();
                    point.connected_from.dedup();
                }
            }

            true
        } else {
            false
        }
    }

    ///
    /// Checks that the following edges are consistent
    ///
    #[cfg(any(test, extra_checks))]
    pub (crate) fn check_following_edge_consistency(&self) {
        let mut used_edges = vec![vec![]; self.points.len()];

        for point_idx in 0..(self.points.len()) {
            let point = &self.points[point_idx];

            for edge_idx in 0..(point.forward_edges.len()) {
                let edge = &point.forward_edges[edge_idx];

                test_assert!(edge.end_idx < self.points.len());
                test_assert!(edge.following_edge_idx < self.points[edge.end_idx].forward_edges.len());
                test_assert!(!used_edges[edge.end_idx].contains(&edge.following_edge_idx));

                used_edges[edge.end_idx].push(edge.following_edge_idx);
            }
        }
    }

    #[cfg(not(any(test, extra_checks)))]
    pub (crate) fn check_following_edge_consistency(&self) {

    }
}

///
/// Removes any pairs of collisions that are closer than `CLOSE_DISTANCE` apart, and also rounds the 
/// first and last collisions to 0.0 and 1.0
/// 
/// When colliding two bezier curves we want to avoid subdividing excessively to produce very small 
/// sections as they have a tendency to produce extra collisions due to floating point or root finding
/// errors.
///
fn remove_and_round_close_collisions<C: BezierCurve>(collisions: &mut SmallVec<[(f64, f64); 8]>, src: &C, tgt: &C)
where C::Point: Coordinate+Coordinate2D {
    // Nothing to do if there are no collisions
    if collisions.len() == 0 {
        return;
    }

    // Work out the positions of each point
    let mut positions = collisions.iter().map(|(t1, _t2)| src.point_at_pos(*t1)).collect::<Vec<_>>();

    // Find any pairs of points that are too close together
    let mut collision_idx = 0;
    while collision_idx+1 < collisions.len() {
        // Just remove both of these if they are too close together (as each collision crosses the curve once, removing collisions in pairs means that there'll still be at least one collision left if the curves actually end up crossing over)
        if positions[collision_idx].is_near_to(&positions[collision_idx+1], CLOSE_DISTANCE) {
            if (collisions[collision_idx].0 - collisions[collision_idx+1].0).abs() < SMALL_T_DISTANCE
                && (collisions[collision_idx].1 - collisions[collision_idx+1].1).abs() < SMALL_T_DISTANCE {
                collisions.remove(collision_idx); positions.remove(collision_idx);
                collisions.remove(collision_idx); positions.remove(collision_idx);
            } else {
                collision_idx += 1;
            }
        } else {
            collision_idx += 1;
        }
    }
    
    // If the first point or the last point is close to the end of the source or target curve, clip to 0 or 1
    if collisions.len() > 0 {
        // Get the start/end points of the source and target
        let src_start   = src.start_point();
        let src_end     = src.end_point();
        let tgt_start   = tgt.start_point();
        let tgt_end     = tgt.end_point();

        // Snap collisions to 0.0 or 1.0 if they're very close to the start or end of either curve
        for collision_idx in 0..collisions.len() {
            // Snap the source side
            if collisions[collision_idx].0 > 0.0 && collisions[collision_idx].0 < 1.0 {
                if src_start.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) && collisions[collision_idx].0 < SMALL_T_DISTANCE {
                    collisions[collision_idx].0 = 0.0;
                }

                if src_end.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) && collisions[collision_idx].0 > 1.0-SMALL_T_DISTANCE {
                    collisions[collision_idx].0 = 1.0;
                }
            }

            // Snap the target side
            if collisions[collision_idx].1 > 0.0 && collisions[collision_idx].1 < 1.0 && collisions[collision_idx].1 < SMALL_T_DISTANCE {
                if tgt_start.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) {
                    collisions[collision_idx].1 = 0.0;
                }

                if tgt_end.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) && collisions[collision_idx].1 > 1.0-SMALL_T_DISTANCE {
                    collisions[collision_idx].1 = 1.0;
                }
            }
        }
    }
}
