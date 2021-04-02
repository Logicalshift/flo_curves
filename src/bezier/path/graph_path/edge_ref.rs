use super::{GraphPath, GraphEdge, GraphEdgeRef};
use crate::geo::*;

impl GraphEdgeRef {
    ///
    /// Creates a reversed version of this edge ref
    ///
    pub fn reversed(mut self) -> GraphEdgeRef {
        self.reverse = !self.reverse;
        self
    }
}

///
/// A GraphEdgeRef can be created from a GraphEdge in order to release the borrow
///
impl<'a, Point: 'a+Coordinate, Label: 'a+Copy> From<GraphEdge<'a, Point, Label>> for GraphEdgeRef {
    fn from(edge: GraphEdge<'a, Point, Label>) -> GraphEdgeRef {
        edge.edge
    }
}

///
/// A GraphEdgeRef can be created from a GraphEdge in order to release the borrow
///
impl<'a, 'b, Point: 'a+Coordinate, Label: 'a+Copy> From<&'b GraphEdge<'a, Point, Label>> for GraphEdgeRef {
    fn from(edge: &'b GraphEdge<'a, Point, Label>) -> GraphEdgeRef {
        edge.edge
    }
}

impl<Point: Coordinate+Coordinate2D, Label> GraphPath<Point, Label> {
    ///
    /// Given an edge ref, returns the edge ref that follows it
    ///
    #[inline]
    pub fn following_edge_ref(&self, edge_ref: GraphEdgeRef) -> GraphEdgeRef {
        if edge_ref.reverse {
            // Need to search in reverse for the edge
            for connected_from in self.points[edge_ref.start_idx].connected_from.iter() {
                for (edge_idx, edge) in self.points[*connected_from].forward_edges.iter().enumerate() {
                    if edge.end_idx == edge_ref.start_idx {
                        return GraphEdgeRef {
                            start_idx:  *connected_from,
                            edge_idx:   edge_idx,
                            reverse:    true
                        }
                    }
                }
            }

            panic!("Reverse edge could not be found")
        } else {
            // Can just use the following edge
            let edge = &self.points[edge_ref.start_idx].forward_edges[edge_ref.edge_idx];

            GraphEdgeRef {
                start_idx:  edge.end_idx,
                edge_idx:   edge.following_edge_idx,
                reverse:    false
            }
        }
    }
}

