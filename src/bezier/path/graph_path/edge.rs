use super::{GraphEdge, GraphPathEdge, GraphPathEdgeKind};
use super::super::super::curve::*;
use super::super::super::bounds::*;
use super::super::super::super::geo::*;

use std::fmt;
use std::cell::*;

impl<Point: Coordinate, Label> GraphPathEdge<Point, Label> {
    ///
    /// Creates a new graph path edge
    /// 
    #[inline]
    fn new(kind: GraphPathEdgeKind, (cp1, cp2): (Point, Point), end_idx: usize, label: Label, following_edge_idx: usize) -> GraphPathEdge<Point, Label> {
        GraphPathEdge {
            label, kind, cp1, cp2, end_idx, following_edge_idx, bbox: RefCell::new(None)
        }
    }

    ///
    /// Updates the control points of this edge
    /// 
    #[inline]
    fn set_control_points(&mut self, (cp1, cp2): (Point, Point), end_idx: usize, next_edge_idx: usize) {
        self.cp1                = cp1;
        self.cp2                = cp2;
        self.end_idx            = end_idx;
        self.following_edge_idx = next_edge_idx;
    }

    ///
    /// Invalidates the cache for this edge
    ///
    #[inline]
    fn invalidate_cache(&self) {
        (*self.bbox.borrow_mut()) = None;
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
        self.graph.points[self.start_point_index()].position.clone()
    }

    ///
    /// The end point of this curve
    /// 
    #[inline]
    fn end_point(&self) -> Self::Point {
        self.graph.points[self.end_point_index()].position.clone()
    }

    ///
    /// The control points in this curve
    /// 
    #[inline]
    fn control_points(&self) -> (Self::Point, Self::Point) {
        let edge = self.edge();

        if self.edge.reverse {
            (edge.cp2.clone(), edge.cp1.clone())
        } else {
            (edge.cp1.clone(), edge.cp2.clone())
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

impl<'a, Point: fmt::Debug, Label: 'a+Copy> fmt::Debug for GraphEdge<'a, Point, Label> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?} -> {:?} ({:?} -> {:?} ({:?}, {:?}))", self.kind(), self.edge.start_idx, self.edge().end_idx, self.graph.points[self.edge.start_idx].position, self.graph.points[self.edge().end_idx].position, self.edge().cp1, self.edge().cp2)
    }
}
