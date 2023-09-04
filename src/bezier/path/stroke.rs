use super::path::*;

use crate::geo::*;

///
/// How two segments of a line should be joined together
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

///
/// How the end of a line should be drawn
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square
}

///
/// Settings for a line stroke operation
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct StrokeOptions {
    /// How accurately to match the curves,
    accuracy: f64,

    /// The minimum difference in tangent between two samples used for fitting the resulting path
    min_tangent_difference: f64,

    /// The minimum distance between samples, when the minimum tangent is not reached
    min_sample_distance: f64,

    /// How two lines should be joined together
    join: LineJoin,

    /// How to start the line
    start_cap: LineCap,

    /// How to finish the line
    end_cap: LineCap,
}

///
/// Generates a thickened line along a path
///
/// The width describes how wide to make the resulting line. 
///
pub fn stroke_path<TPathFactory, TCoord>(path: impl BezierPath<Point=TCoord>, width: f64, join: LineJoin, start_cap: LineCap, end_cap: LineCap) -> TPathFactory
where
    TPathFactory:   BezierPathFactory<Point=TCoord>,
    TCoord:         Coordinate + Coordinate2D,
{
    // TODO: subdivision algorithm: want to put points at the start end, the x and y extremes and all the midpoints
    // TOOD: offset these points, subdivide between any two that are far enough apart

    todo!()
}