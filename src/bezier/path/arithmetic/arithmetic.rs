use super::super::path::*;
use super::super::graph_path::*;
use super::super::is_clockwise::*;
use super::super::super::curve::*;
use super::super::super::normal::*;
use super::super::super::super::geo::*;

use smallvec::*;

///
/// Winding direction of a particular path
///  
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PathDirection {
    Clockwise,
    Anticlockwise
}

impl<'a, P: BezierPath> From<&'a P> for PathDirection
where P::Point: Coordinate2D {
    #[inline]
    fn from(path: &'a P) -> PathDirection {
        if path.is_clockwise() {
            PathDirection::Clockwise
        } else {
            PathDirection::Anticlockwise
        }
    }
}

///
/// Label attached to a path used for arithmetic
/// 
/// The parameters are the path number (counting from 0) and the winding direction of the path
/// 
#[derive(Clone, Copy, Debug)]
pub struct PathLabel(pub u32, pub PathDirection);

impl<Point: Coordinate+Coordinate2D> GraphPath<Point, PathLabel> {
    ///
    /// Sets the edge kinds by performing ray casting
    /// 
    /// The function passed in to this method takes two parameters: these are the number of times edges have been crossed in
    /// path 1 and path 2. It should return true if this number of crossings represents a point inside the final shape, or false
    /// if it represents a point outside of the shape.
    ///
    pub fn set_edge_kinds_by_ray_casting<FnIsInside: Fn(&SmallVec<[i32; 8]>) -> bool>(&mut self, is_inside: FnIsInside) {
        for point_idx in 0..self.num_points() {
            for next_edge in self.edge_refs_for_point(point_idx) {
                // Only process edges that have not yet been categorised
                if self.edge_kind(next_edge) != GraphPathEdgeKind::Uncategorised {
                    continue;
                }

                // Cast a ray at this edge
                let real_edge   = self.get_edge(next_edge);
                let next_point  = real_edge.point_at_pos(0.5);
                let next_normal = real_edge.normal_at_pos(0.5);

                // Mark the next edge as visited (this prevents an infinite loop in the event the edge we're aiming at has a length of 0 and thus will always be an intersection)
                self.set_edge_kind(next_edge, GraphPathEdgeKind::Visited);

                // The 'total direction' indicates how often we've crossed an edge moving in a particular direction
                // We're inside the path when it's non-zero
                let mut path_crossings: SmallVec<[i32; 8]> = smallvec![0, 0];

                // Cast a ray at the target edge
                let ray             = (next_point - next_normal, next_point);
                let ray_direction   = ray.1 - ray.0;
                let collisions      = self.ray_collisions(&ray);

                // There should always be an even number of collisions on a particular ray cast through a closed shape
                debug_assert!((collisions.len()&1) == 0);

                for (collision, curve_t, _line_t, _pos) in collisions {
                    let is_intersection = collision.is_intersection();
                    let edge            = collision.edge();

                    let PathLabel(path_number, direction) = self.edge_label(edge);

                    // The relative direction of the tangent to the ray indicates the direction we're crossing in
                    let normal  = self.get_edge(edge).normal_at_pos(curve_t);

                    let side    = ray_direction.dot(&normal).signum() as i32;
                    let side    = match direction {
                        PathDirection::Clockwise        => { side },
                        PathDirection::Anticlockwise    => { -side }
                    };

                    // Extend the path_crossings vector
                    while path_crossings.len() <= path_number as usize { path_crossings.push(0); }

                    let was_inside = is_inside(&path_crossings);
                    if side < 0 {
                        path_crossings[path_number as usize] -= 1;
                    } else if side > 0 {
                        path_crossings[path_number as usize] += 1;
                    }
                    let is_inside = is_inside(&path_crossings);

                    // If this isn't an intersection, set whether or not the edge is exterior
                    let edge_kind = self.edge_kind(edge);
                    if !is_intersection && (edge_kind == GraphPathEdgeKind::Uncategorised || edge_kind == GraphPathEdgeKind::Visited) {
                        // Exterior edges move from inside to outside or vice-versa
                        if was_inside ^ is_inside {
                            // Exterior edge
                            self.set_edge_kind_connected(edge, GraphPathEdgeKind::Exterior);
                        } else {
                            // Interior edge
                            self.set_edge_kind_connected(edge, GraphPathEdgeKind::Interior);
                        }
                    } else if !is_intersection {
                        if was_inside ^ is_inside {
                            if edge_kind != GraphPathEdgeKind::Exterior {
                                // We've likely got a missing collision in the graph so an edge is both inside and outside
                                // Set the edge to be an 'exterior' one so that we increase the chances of finding a path
                                self.set_edge_kind_connected(edge, GraphPathEdgeKind::Exterior);
                            }

                            // This is a bug so fail in debug builds
                            debug_assert!(edge_kind == GraphPathEdgeKind::Exterior);
                        } else {
                            debug_assert!(edge_kind == GraphPathEdgeKind::Interior);
                        }
                    }
                }

                // The ray should exit and enter the path an even number of times
                debug_assert!(path_crossings.into_iter().all(|crossing_count| crossing_count == 0));
            }
        }
    }
}
