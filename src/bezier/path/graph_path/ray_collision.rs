use super::{GraphEdge, GraphEdgeRef, GraphPath};
use crate::bezier::path::ray::*;
use crate::geo::*;
use crate::line::*;

use smallvec::*;

///
/// Represents a collision between a ray and a GraphPath
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraphRayCollision {
    /// Collision against a single edge
    SingleEdge(GraphEdgeRef),

    /// Collision against an intersection point
    Intersection(GraphEdgeRef),
}

impl<Point: Coordinate + Coordinate2D, Label: Copy> GraphPath<Point, Label> {
    ///
    /// Finds all collisions between a ray and this path
    ///
    /// The return value is a tuple of (collision, curve_t, line_t, position)
    ///
    pub fn ray_collisions<L: Line<Point = Point>>(
        &self,
        ray: &L,
    ) -> Vec<(GraphRayCollision, f64, f64, Point)> {
        ray_collisions(&self, ray)
    }
}

impl GraphRayCollision {
    ///
    /// Returns true if this collision is at an intersection
    ///
    #[inline]
    pub fn is_intersection(&self) -> bool {
        match self {
            GraphRayCollision::SingleEdge(_) => false,
            GraphRayCollision::Intersection(_edges) => true,
        }
    }

    ///
    /// Returns the edge this collision is for
    ///
    #[inline]
    pub fn edge(&self) -> GraphEdgeRef {
        match self {
            GraphRayCollision::SingleEdge(edge) => *edge,
            GraphRayCollision::Intersection(edge) => *edge,
        }
    }
}

impl<'a, Point, Label> RayPath for &'a GraphPath<Point, Label>
where
    Point: Coordinate + Coordinate2D,
    Label: Copy,
{
    type Point = Point;
    type Curve = GraphEdge<'a, Point, Label>;

    #[inline]
    fn num_points(&self) -> usize {
        self.points.len()
    }

    #[inline]
    fn num_edges(&self, point_idx: usize) -> usize {
        self.points[point_idx].forward_edges.len()
    }

    #[inline]
    fn edges_for_point(&self, point_idx: usize) -> SmallVec<[GraphEdgeRef; 8]> {
        let num_edges = self.points[point_idx].forward_edges.len();
        (0..num_edges)
            .into_iter()
            .map(move |edge_idx| GraphEdgeRef {
                start_idx: point_idx,
                edge_idx: edge_idx,
                reverse: false,
            })
            .collect()
    }

    #[inline]
    fn reverse_edges_for_point(&self, point_idx: usize) -> SmallVec<[GraphEdgeRef; 8]> {
        self.points[point_idx]
            .connected_from
            .iter()
            .flat_map(|connected_point_idx| {
                let num_edges = self.points[*connected_point_idx].forward_edges.len();

                (0..num_edges)
                    .into_iter()
                    .filter(move |edge_idx| {
                        self.points[*connected_point_idx].forward_edges[*edge_idx].end_idx
                            == point_idx
                    })
                    .map(move |edge_idx| GraphEdgeRef {
                        start_idx: *connected_point_idx,
                        edge_idx: edge_idx,
                        reverse: true,
                    })
            })
            .collect()
    }

    #[inline]
    fn get_edge(&self, edge: GraphEdgeRef) -> Self::Curve {
        GraphEdge {
            graph: *self,
            edge: edge,
        }
    }

    #[inline]
    fn get_next_edge(&self, edge: GraphEdgeRef) -> (GraphEdgeRef, Self::Curve) {
        let next_point_idx = self.edge_end_point_idx(edge);
        let next_edge_idx = self.edge_following_edge_idx(edge);

        let next_edge_ref = GraphEdgeRef {
            start_idx: next_point_idx,
            edge_idx: next_edge_idx,
            reverse: edge.reverse,
        };

        (next_edge_ref, self.get_edge(next_edge_ref))
    }

    #[inline]
    fn point_position(&self, point: usize) -> Self::Point {
        self.points[point].position
    }

    #[inline]
    fn edge_start_point_idx(&self, edge: GraphEdgeRef) -> usize {
        if edge.reverse {
            self.points[edge.start_idx].forward_edges[edge.edge_idx].end_idx
        } else {
            edge.start_idx
        }
    }

    #[inline]
    fn edge_end_point_idx(&self, edge: GraphEdgeRef) -> usize {
        if edge.reverse {
            edge.start_idx
        } else {
            self.points[edge.start_idx].forward_edges[edge.edge_idx].end_idx
        }
    }

    #[inline]
    fn edge_following_edge_idx(&self, edge: GraphEdgeRef) -> usize {
        if edge.reverse {
            unimplemented!(
                "Finding the following edge for a reversed reference not implemented yet"
            )
        } else {
            self.points[edge.start_idx].forward_edges[edge.edge_idx].following_edge_idx
        }
    }
}
