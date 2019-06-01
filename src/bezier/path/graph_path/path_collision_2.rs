use super::{GraphPath};
use super::super::super::super::geo::*;

use std::ops::Range;

impl<Point: Coordinate+Coordinate2D, Label: Copy> GraphPath<Point, Label> {
    ///
    /// Searches two ranges of points in this object and detects collisions between them, subdividing the edges
    /// and creating branch points at the appropriate places.
    /// 
    /// collide_from must indicate indices lower than collide_to
    /// 
    pub (crate) fn detect_collisions(&mut self, collide_from: Range<usize>, collide_to: Range<usize>, accuracy: f64) {
        unimplemented!()
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
