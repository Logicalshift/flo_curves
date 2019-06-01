use super::{GraphPath, GraphEdge, GraphEdgeRef, GraphPathPoint};
use super::super::super::curve::*;
use super::super::super::intersection::*;
use super::super::super::super::geo::*;
use super::super::super::super::consts::*;

use smallvec::*;

use std::mem;
use std::ops::Range;
use std::collections::HashMap;

///
/// Struct describing a collision between two edges
///
#[derive(Clone, Copy)]
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
                            // Avoid colliding the same edge against itself
                            if src_idx == tgt_idx && src_edge_idx == tgt_edge_idx { continue; }

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

                            // Turn into collisions
                            let edge_collisions = edge_collisions.into_iter()
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
    pub (crate) fn detect_collisions(&mut self, collide_from: Range<usize>, collide_to: Range<usize>, accuracy: f64) {
        // Find all of the collision points
        let all_collisions      = self.find_collisions(collide_from, collide_to, accuracy);

        // Add in any extra points that are required by the collisions we found
        let all_collisions      = self.create_collision_points(all_collisions);

        // Organize the collisions by edge
        let collisions_by_edge  = self.organize_collisions_by_edge(all_collisions);

        // 

        // Finish up by checking that we haven't broken consistency
        self.recalculate_reverse_connections();
        self.remove_all_very_short_edges();
        self.combine_overlapping_points(accuracy);
        self.check_following_edge_consistency();
        unimplemented!()
    }

    ///
    /// Finds any points that have approximately the same coordinates and combines them
    /// 
    /// Accuracy indicates the maximum difference in the x or y coordinate for two points to be considered the same.
    ///
    #[inline(never)]
    pub fn combine_overlapping_points(&mut self, accuracy: f64) {
        // Find collisions using a hashmap
        let multiplier      = 1.0 / accuracy;
        let mut collisions  = HashMap::new();

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
                for connected_from in point.connected_from.iter_mut() {
                    if let Some((remapped_point, _following_edge_idx_offset)) = remapped_points.get(connected_from) {
                        *connected_from = *remapped_point;
                    }
                }
            }
        }
    }

    ///
    /// Checks that the following edges are consistent
    ///
    #[cfg(debug_assertions)]
    pub (crate) fn check_following_edge_consistency(&self) {
        let mut used_edges = vec![vec![]; self.points.len()];

        for point_idx in 0..(self.points.len()) {
            let point = &self.points[point_idx];

            for edge_idx in 0..(point.forward_edges.len()) {
                let edge = &point.forward_edges[edge_idx];

                debug_assert!(edge.end_idx < self.points.len());
                debug_assert!(edge.following_edge_idx < self.points[edge.end_idx].forward_edges.len());
                debug_assert!(!used_edges[edge.end_idx].contains(&edge.following_edge_idx));

                used_edges[edge.end_idx].push(edge.following_edge_idx);
            }
        }
    }

    #[cfg(not(debug_assertions))]
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
            collisions.remove(collision_idx); positions.remove(collision_idx);
            collisions.remove(collision_idx); positions.remove(collision_idx);
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
                if src_start.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) {
                    collisions[collision_idx].0 = 0.0;
                }

                if src_end.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) {
                    collisions[collision_idx].0 = 1.0;
                }
            }

            // Snap the target side
            if collisions[collision_idx].1 > 0.0 && collisions[collision_idx].1 < 1.0 {
                if tgt_start.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) {
                    collisions[collision_idx].1 = 0.0;
                }

                if tgt_end.is_near_to(&positions[collision_idx], CLOSE_DISTANCE) {
                    collisions[collision_idx].1 = 1.0;
                }
            }
        }
    }
}
