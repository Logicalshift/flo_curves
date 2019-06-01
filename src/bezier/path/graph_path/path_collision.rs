use super::{GraphPath, GraphEdge, GraphEdgeRef, GraphPathPoint, GraphPathEdge};
use super::super::super::curve::*;
use super::super::super::intersection::*;
use super::super::super::super::geo::*;
use super::super::super::super::consts::*;

use smallvec::*;

use std::mem;
use std::ops::Range;
use std::collections::HashMap;

///
/// Struct representing a collision in the graph path
///
struct Collision {
    idx:    usize,
    edge:   usize,
    t:      f64
}

///
/// Struct representing a set of collisions in the graph path
///
struct CollisionList {
    /// List of collisions on the source and target side
    collisions: Vec<(Collision, Collision)>
}

impl CollisionList {
    ///
    /// Creates a new list of collisions
    ///
    fn new() -> CollisionList {
        CollisionList { 
            collisions: vec![]
        }
    }

    ///
    /// Adds a collision to this list
    ///
    fn push(&mut self, collision: (Collision, Collision)) {
        self.collisions.push(collision);
    }

    ///
    /// Removes the last collision from this list
    ///
    fn pop(&mut self) -> Option<(Collision, Collision)> {
        self.collisions.pop()
    }

    ///
    /// For all remaining collisions, finds any that use the specified edge and change them so they are subdivided at 
    /// the specified t value
    ///
    fn move_after_midpoint<Point, Label>(&mut self, graph: &mut GraphPath<Point, Label>, midpoint: usize, point_idx: usize, edge_idx: usize, t: f64, new_edge_idx: usize) {
        // Usually new_mid_point is a new point, but it can be an existing point in the event the collision was at an existing point on the path
        debug_assert!(midpoint < graph.points.len());
        debug_assert!(new_edge_idx < graph.points[midpoint].forward_edges.len());

        // TODO(?): this just iterates through the collisions, not clear if this will always be fast enough
        for (ref mut collision_src, ref mut collision_tgt) in self.collisions.iter_mut() {
            // If the src edge was divided...
            if collision_src.idx == point_idx && collision_src.edge == edge_idx {
                if collision_src.t < t {
                    // Before the midpoint. Edge is the same, just needs to be modified.
                    collision_src.t /= t;
                } else {
                    debug_assert!(graph.points[midpoint].forward_edges.len() > 0);

                    // After the midpoint. Edge needs to be adjusted.
                    collision_src.t     = (collision_src.t - t) / (1.0-t);
                    collision_src.idx   = midpoint;
                    collision_src.edge  = new_edge_idx;
                }
            }

            // If the target edge was divided...
            if collision_tgt.idx == point_idx && collision_tgt.edge == edge_idx {
                if collision_tgt.t < t {
                    // Before the midpoint. Edge is the same, just needs to be modified.
                    collision_tgt.t /= t;
                } else {
                    debug_assert!(graph.points[midpoint].forward_edges.len() > 1);

                    // After the midpoint. Edge needs to be adjusted.
                    collision_tgt.t     = (collision_tgt.t - t) / (1.0-t);
                    collision_tgt.idx   = midpoint;
                    collision_tgt.edge  = new_edge_idx;
                }
            }
        }
    }

    ///
    /// Takes all the collisions that were originally on `original_point_idx` and changes them to `new_point_idx`.
    /// The edges should still be in sequence, starting at `edge_idx_offset` in the new point
    ///
    fn move_all_edges(&mut self, original_point_idx: usize, new_point_idx: usize, edge_idx_offset: usize) {
        if original_point_idx == new_point_idx {
            // Edges will be unchanged
            return;
        }

        for (ref mut collision_src, ref mut collision_tgt) in self.collisions.iter_mut() {
            if collision_src.idx == original_point_idx {
                collision_src.idx   = new_point_idx;
                collision_src.edge  += edge_idx_offset;
            }
            if collision_tgt.idx == original_point_idx {
                collision_tgt.idx   = new_point_idx;
                collision_tgt.edge  += edge_idx_offset;
            }
        }
    }

    ///
    /// Checks consistency of the points and edges against a graph path
    ///
    #[cfg(debug_assertions)]
    fn check_consistency<Point, Label>(&self, graph: &GraphPath<Point, Label>) {
        for (src, tgt) in self.collisions.iter() {
            debug_assert!(src.idx < graph.points.len());
            debug_assert!(src.edge < graph.points[src.idx].forward_edges.len());

            debug_assert!(tgt.idx < graph.points.len());
            debug_assert!(tgt.edge < graph.points[tgt.idx].forward_edges.len());
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn check_consistency<Point, Label>(&self, _graph: &GraphPath<Point, Label>) {
    }
}

impl<Point: Coordinate+Coordinate2D, Label: Copy> GraphPath<Point, Label> {
    ///
    /// Changes every edge that ends at old_point_idx to end at new_point_idx instead
    /// 
    /// Also moves every edge leaving old_point_idx so that it leaves new_point_idx instead
    ///
    fn change_all_edges_ending_at_point(&mut self, old_point_idx: usize, new_point_idx: usize, collisions: &mut CollisionList) {
        // Nothing to do if the indexes are the same
        if old_point_idx == new_point_idx { return; }

        self.check_following_edge_consistency();

        // Edges pointing at old_point_idx will be added to new_point_idx starting at this point
        let edge_idx_offset = self.points[new_point_idx].forward_edges.len();

        // Search all of the points to find edges ending at old_point_idx and update them
        for point_idx in 0..(self.points.len()) {
            for edge_idx in 0..(self.points[point_idx].forward_edges.len()) {
                let edge = &mut self.points[point_idx].forward_edges[edge_idx];
                if edge.end_idx == old_point_idx {
                    // Move the edge to end at the new point and have a suitable following edge index
                    edge.end_idx            = new_point_idx;
                    edge.following_edge_idx += edge_idx_offset;
                }
            }
        }

        // Take the old edges and append them to the new point
        let mut old_point_edges = smallvec![];
        mem::swap(&mut self.points[old_point_idx].forward_edges, &mut old_point_edges);

        self.points[new_point_idx].forward_edges.extend(old_point_edges);

        // Move all of the collisions
        collisions.move_all_edges(old_point_idx, new_point_idx, edge_idx_offset);

        collisions.check_consistency(self);
        self.check_following_edge_consistency();
    }

    ///
    /// Joins two edges at an intersection, returning the index of the intersection point
    /// 
    /// For t=0 or 1 the intersection point may be one of the ends of the edges, otherwise
    /// this will divide the existing edges so that they both meet at the specified mid-point.
    /// 
    /// Note that the case where t=1 is the same as the case where t=0 on a following edge.
    /// The split algorithm is simpler if only the t=0 case is considered.
    /// 
    #[inline]
    fn join_edges_at_intersection(&mut self, edge1: (usize, usize), edge2: (usize, usize), t1: f64, t2: f64, collisions: &mut CollisionList) -> Option<usize> {
        // Do nothing if the edges are the same (they're effectively already joined)
        if edge1 == edge2 { return None; }

        // Get the edge indexes
        let (edge1_idx, edge1_edge_idx) = edge1;
        let (edge2_idx, edge2_edge_idx) = edge2;

        // Create representations of the two edges
        let edge1 = Curve::from_curve(&GraphEdge::new(self, GraphEdgeRef { start_idx: edge1_idx, edge_idx: edge1_edge_idx, reverse: false }));
        let edge2 = Curve::from_curve(&GraphEdge::new(self, GraphEdgeRef { start_idx: edge2_idx, edge_idx: edge2_edge_idx, reverse: false }));

        // If we're very close to the start or end, round to the start/end
        let p1  = edge1.point_at_pos(t1);
        let p2  = edge2.point_at_pos(t2);

        let t1  = if p1.is_near_to(&edge1.start_point(), SMALL_DISTANCE) {
            0.0
        } else if p1.is_near_to(&edge1.end_point(), SMALL_DISTANCE) {
            1.0
        } else {
            t1
        };

        let t2  = if p2.is_near_to(&edge2.start_point(), SMALL_DISTANCE) {
            0.0
        } else if p2.is_near_to(&edge2.end_point(), SMALL_DISTANCE) {
            1.0
        } else {
            t2
        };

        // Create or choose a point to collide at
        // (If t1 or t2 is 0 or 1 we collide on the edge1 or edge2 points, otherwise we create a new point to collide at)
        let collision_point = if Self::t_is_zero(t1) {
            edge1_idx
        } else if Self::t_is_one(t1) {
            self.points[edge1_idx].forward_edges[edge1_edge_idx].end_idx
        } else if Self::t_is_zero(t2) {
            edge2_idx
        } else if Self::t_is_one(t2) {
            self.points[edge2_idx].forward_edges[edge2_edge_idx].end_idx
        } else {
            // Point is a mid-point of both lines

            // Work out where the mid-point is (use edge1 for this always: as this is supposed to be an intersection this shouldn't matter)
            // Note that if we use de Casteljau's algorithm here we get a subdivision for 'free' but organizing the code around it is painful
            let mid_point = edge1.point_at_pos(t1);

            // Add to this list of points
            let mid_point_idx = self.points.len();
            self.points.push(GraphPathPoint::new(mid_point, smallvec![], smallvec![]));

            // New point is the mid-point
            mid_point_idx
        };

        // Subdivide the edges
        let (edge1a, edge1b) = edge1.subdivide::<Curve<_>>(t1);
        let (edge2a, edge2b) = edge2.subdivide::<Curve<_>>(t2);

        // The new edges have the same kinds as their ancestors
        let edge1_kind          = self.points[edge1_idx].forward_edges[edge1_edge_idx].kind;
        let edge2_kind          = self.points[edge2_idx].forward_edges[edge2_edge_idx].kind;
        let edge1_label         = self.points[edge1_idx].forward_edges[edge1_edge_idx].label;
        let edge2_label         = self.points[edge2_idx].forward_edges[edge2_edge_idx].label;
        let edge1_end_idx       = self.points[edge1_idx].forward_edges[edge1_edge_idx].end_idx;
        let edge2_end_idx       = self.points[edge2_idx].forward_edges[edge2_edge_idx].end_idx;
        let edge1_following_idx = self.points[edge1_idx].forward_edges[edge1_edge_idx].following_edge_idx;
        let edge2_following_idx = self.points[edge2_idx].forward_edges[edge2_edge_idx].following_edge_idx;

        // Invalidate the cached bounding boxes
        self.points[edge1_idx].forward_edges[edge1_edge_idx].invalidate_cache();
        self.points[edge2_idx].forward_edges[edge2_edge_idx].invalidate_cache();

        // List of edges we've added to the collision point (in the form of the edge that's divided, the position it was divided at and the index on the collision point)
        let mut new_edges       = vec![];

        // The 'b' edges both extend from our mid-point to the existing end point (provided t < 1.0)
        if !Self::t_is_one(t1) && !Self::t_is_zero(t1) {
            // If t1 is zero or one, we're not subdividing edge1
            // If zero, we're just adding the existing edge again to the collision point (so we do nothing)
            let new_following_idx = self.points[collision_point].forward_edges.len();

            new_edges.push((edge1_idx, edge1_edge_idx, t1, new_following_idx));
            self.points[collision_point].forward_edges.push(GraphPathEdge::new(edge1_kind, edge1b.control_points(), edge1_end_idx, edge1_label, edge1_following_idx));

            // Update edge1
            self.points[edge1_idx].forward_edges[edge1_edge_idx].set_control_points(edge1a.control_points(), collision_point, new_following_idx);

            // If t1 is zero, we're not subdividing edge1
            // If t1 is one this should leave the edge alone
            // If t1 is not one, then the previous step will have added the remaining part of
            // edge1 to the collision point
        }

        self.check_following_edge_consistency();
        collisions.check_consistency(self);

        if !Self::t_is_one(t2) && !Self::t_is_zero(t2) {
            // If t2 is zero or one, we're not subdividing edge2
            // If zero, we're just adding the existing edge again to the collision point (so we do nothing)
            let new_following_idx = self.points[collision_point].forward_edges.len();

            new_edges.push((edge2_idx, edge2_edge_idx, t2, new_following_idx));
            self.points[collision_point].forward_edges.push(GraphPathEdge::new(edge2_kind, edge2b.control_points(), edge2_end_idx, edge2_label, edge2_following_idx));

            // Update edge2
            self.points[edge2_idx].forward_edges[edge2_edge_idx].set_control_points(edge2a.control_points(), collision_point, new_following_idx);
        }

        // The source and target edges will be divided at the midpoint: update any future collisions to take account of that
        for (point_idx, edge_idx, t, new_edge_idx) in new_edges {
            collisions.move_after_midpoint(self, collision_point, point_idx, edge_idx, t, new_edge_idx);
        }

        collisions.check_consistency(self);
        self.check_following_edge_consistency();

        if Self::t_is_one(t2) {
            // If t2 is one, then all edges ending at edge2_end_idx should be redirected to the collision point
            self.change_all_edges_ending_at_point(edge2_end_idx, collision_point, collisions);
        } else if Self::t_is_zero(t2) {
            // If t2 is zero, then all edges ending at edge2_idx should be redirected to the collision point
            self.change_all_edges_ending_at_point(edge2_idx, collision_point, collisions);
        }

        Some(collision_point)
    }

    ///
    /// Searches two ranges of points in this object and detects collisions between them, subdividing the edges
    /// and creating branch points at the appropriate places.
    /// 
    /// collide_from must indicate indices lower than collide_to
    /// 
    pub (crate) fn detect_collisions(&mut self, collide_from: Range<usize>, collide_to: Range<usize>, accuracy: f64) {
        // Vector of all of the collisions found in the graph
        let mut collisions = CollisionList::new();

        // TODO: for complicated paths, maybe some pre-processing for bounding boxes to eliminate trivial cases would be beneficial for performance

        // Iterate through the edges in the 'from' range
        for src_idx in collide_from {
            for src_edge_idx in 0..self.points[src_idx].forward_edges.len() {
                // Only visit target points that have not already been visited as a source point (assume that collide_to is always a higher range than collide_from)
                let tgt_start       = collide_to.start.max(src_idx+1);
                let tgt_end         = collide_to.end.max(src_idx+1);
                let collide_to      = tgt_start..tgt_end;

                let src_curve       = GraphEdge::new(self, GraphEdgeRef { start_idx: src_idx, edge_idx: src_edge_idx, reverse: false });
                let src_edge_bounds = src_curve.fast_bounding_box::<Bounds<_>>();

                // Compare to each point in the collide_to range
                for tgt_idx in collide_to.into_iter() {
                    for tgt_edge_idx in 0..self.points[tgt_idx].forward_edges.len() {
                        // Don't collide edges against themselves
                        if src_idx == tgt_idx && src_edge_idx == tgt_edge_idx { continue; }

                        // Create edge objects for each side
                        let tgt_curve               = GraphEdge::new(self, GraphEdgeRef { start_idx: tgt_idx, edge_idx: tgt_edge_idx, reverse: false });

                        // Quickly reject edges with non-overlapping bounding boxes
                        let tgt_edge_bounds         = tgt_curve.fast_bounding_box::<Bounds<_>>();
                        if !src_edge_bounds.overlaps(&tgt_edge_bounds) { continue; }

                        // Find the collisions between these two edges
                        let mut curve_collisions    = curve_intersects_curve_clip(&src_curve, &tgt_curve, accuracy);

                        // Remove any pairs of collisions that are too close together
                        remove_and_round_close_collisions(&mut curve_collisions, &src_curve, &tgt_curve);

                        // The are the points we need to divide the existing edges at and add branches
                        let tgt_idx = tgt_idx;
                        for (src_t, tgt_t) in curve_collisions {
                            // A collision at t=1 is the same as a collision on t=0 on a following edge
                            // Edge doesn't actually matter for these (as the ray will collide with all of the following edges)
                            let (src_idx, src_edge_idx, src_t) = if Self::t_is_one(src_t) {
                                (self.points[src_idx].forward_edges[src_edge_idx].end_idx, 0, 0.0)
                            } else {
                                (src_idx, src_edge_idx, src_t)
                            };

                            let (tgt_idx, tgt_edge_idx, tgt_t) = if Self::t_is_one(tgt_t) {
                                (self.points[tgt_idx].forward_edges[tgt_edge_idx].end_idx, 0, 0.0)
                            } else {
                                (tgt_idx, tgt_edge_idx, tgt_t)
                            };

                            debug_assert!(src_idx < self.points.len());
                            debug_assert!(tgt_idx < self.points.len());
                            debug_assert!(src_edge_idx < self.points[src_idx].forward_edges.len());
                            debug_assert!(tgt_edge_idx < self.points[tgt_idx].forward_edges.len());

                            // Add this as a collision
                            let src = Collision { idx: src_idx, edge: src_edge_idx, t: src_t };
                            let tgt = Collision { idx: tgt_idx, edge: tgt_edge_idx, t: tgt_t };
                            collisions.push((src, tgt));
                        }
                    }
                }
            }
        }

        collisions.check_consistency(self);
        self.check_following_edge_consistency();

        // Apply the divisions to the edges
        while let Some((src, tgt)) = collisions.pop() {
            // Join the edges
            let _new_mid_point = self.join_edges_at_intersection((src.idx, src.edge), (tgt.idx, tgt.edge), src.t, tgt.t, &mut collisions);
            collisions.check_consistency(self);

            self.check_following_edge_consistency();
        }

        self.check_following_edge_consistency();

        // Recompute the reverse connections
        self.recalculate_reverse_connections();

        // Remove any very short edges that might have been generated during the collision detection
        self.remove_all_very_short_edges();
        self.combine_overlapping_points(accuracy);
        self.check_following_edge_consistency();
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
