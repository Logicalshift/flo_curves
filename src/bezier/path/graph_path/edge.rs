use super::{GraphPath, GraphEdgeRef, GraphEdge, GraphPathEdge, GraphPathEdgeKind};
use crate::bezier::curve::*;
use crate::bezier::bounds::*;
use crate::geo::*;

use std::fmt;
use std::cell::*;

impl<Point: Coordinate, Label> GraphPathEdge<Point, Label> {
    ///
    /// Creates a new graph path edge
    /// 
    #[inline]
    pub (crate) fn new(kind: GraphPathEdgeKind, (cp1, cp2): (Point, Point), end_idx: usize, label: Label, following_edge_idx: usize) -> GraphPathEdge<Point, Label> {
        GraphPathEdge {
            label, kind, cp1, cp2, end_idx, following_edge_idx, bbox: RefCell::new(None)
        }
    }

    ///
    /// Invalidates the cache for this edge
    ///
    #[inline]
    pub (crate) fn invalidate_cache(&self) {
        (*self.bbox.borrow_mut()) = None;
    }
}

impl<'a, Point: 'a, Label: 'a+Copy> GraphEdge<'a, Point, Label> {
    ///
    /// Creates a new graph edge (with an edge kind of 'exterior')
    /// 
    #[inline]
    pub (crate) fn new(graph: &'a GraphPath<Point, Label>, edge: GraphEdgeRef) -> GraphEdge<'a, Point, Label> {
        test_assert!(edge.start_idx < graph.points.len());
        test_assert!(edge.edge_idx < graph.points[edge.start_idx].forward_edges.len());

        GraphEdge {
            graph:  graph,
            edge:   edge
        }
    }

    ///
    /// Returns true if this edge is going backwards around the path
    ///
    #[inline]
    pub fn is_reversed(&self) -> bool {
        self.edge.reverse
    }

    ///
    /// Retrieves a reference to the edge in the graph
    ///
    #[inline]
    fn edge<'b>(&'b self) -> &'b GraphPathEdge<Point, Label> {
        &self.graph.points[self.edge.start_idx].forward_edges[self.edge.edge_idx]
    }

    ///
    /// Returns if this is an interior or an exterior edge in the path
    /// 
    pub fn kind(&self) -> GraphPathEdgeKind {
        self.edge().kind
    }

    ///
    /// Returns the index of the start point of this edge
    /// 
    #[inline]
    pub fn start_point_index(&self) -> usize {
        if self.edge.reverse {
            self.edge().end_idx
        } else {
            self.edge.start_idx
        }
    }

    ///
    /// Returns the index of the end point of this edge
    /// 
    #[inline]
    pub fn end_point_index(&self) -> usize {
        if self.edge.reverse {
            self.edge.start_idx
        } else {
            self.edge().end_idx
        }
    }

    ///
    /// The label attached to this edge
    ///
    #[inline]
    pub fn label(&self) -> Label {
        self.edge().label
    }
}

impl<'a, Point: 'a+Coordinate, Label: 'a> Geo for GraphEdge<'a, Point, Label> {
    type Point = Point;
}

impl<'a, Point: 'a+Coordinate, Label: 'a+Copy> BezierCurve for GraphEdge<'a, Point, Label> {
    ///
    /// The start point of this curve
    /// 
    #[inline]
    fn start_point(&self) -> Self::Point {
        self.graph.points[self.start_point_index()].position
    }

    ///
    /// The end point of this curve
    /// 
    #[inline]
    fn end_point(&self) -> Self::Point {
        self.graph.points[self.end_point_index()].position
    }

    ///
    /// The control points in this curve
    /// 
    #[inline]
    fn control_points(&self) -> (Self::Point, Self::Point) {
        let edge = self.edge();

        if self.edge.reverse {
            (edge.cp2, edge.cp1)
        } else {
            (edge.cp1, edge.cp2)
        }
    }
    
    ///
    /// Faster but less accurate bounding box for a curve
    /// 
    /// This will produce a bounding box that contains the curve but which may be larger than necessary
    /// 
    #[inline]
    fn fast_bounding_box<Bounds: BoundingBox<Point=Self::Point>>(&self) -> Bounds {
        let edge                = self.edge();

        let mut bbox            = edge.bbox.borrow_mut();

        if let Some((ref min, ref max)) = *bbox {
            Bounds::from_min_max(*min, *max)
        } else {
            let start           = self.graph.points[self.edge.start_idx].position;
            let end             = self.graph.points[edge.end_idx].position;
            let control_points  = (edge.cp1, edge.cp2);

            let min             = Self::Point::from_smallest_components(start, end);
            let min             = Self::Point::from_smallest_components(min, control_points.0);
            let min             = Self::Point::from_smallest_components(min, control_points.1);

            let max             = Self::Point::from_biggest_components(start, end);
            let max             = Self::Point::from_biggest_components(max, control_points.0);
            let max             = Self::Point::from_biggest_components(max, control_points.1);

            let bounds          = Bounds::from_min_max(min, max);

            *bbox = Some((bounds.min(), bounds.max()));
            bounds
        }
    }

    ///
    /// Computes the bounds of this bezier curve
    /// 
    #[inline]
    fn bounding_box<Bounds: BoundingBox<Point=Self::Point>>(&self) -> Bounds {
        let edge        = self.edge();

        let start       = self.graph.points[self.edge.start_idx].position;
        let end         = self.graph.points[edge.end_idx].position;
        let (cp1, cp2)  = (edge.cp1, edge.cp2);

        let bounds: Bounds = bounding_box4(start, cp1, cp2, end);

        bounds
    }
}

impl<'a, Point: 'a+Coordinate, Label: 'a+Copy> HasBoundingBox for GraphEdge<'a, Point, Label> {
    #[inline]
    fn get_bounding_box<Bounds: BoundingBox<Point=Self::Point>>(&self) -> Bounds {
        self.fast_bounding_box()
    }
}

impl<'a, Point: fmt::Debug, Label: 'a+Copy> fmt::Debug for GraphEdge<'a, Point, Label> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?} -> {:?} ({:?} -> {:?} ({:?}, {:?}))", self.kind(), self.edge.start_idx, self.edge().end_idx, self.graph.points[self.edge.start_idx].position, self.graph.points[self.edge().end_idx].position, self.edge().cp1, self.edge().cp2)
    }
}
