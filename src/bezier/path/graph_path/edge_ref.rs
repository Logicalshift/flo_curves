use super::{GraphEdge,GraphEdgeRef};
use super::super::super::super::geo::*;

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
