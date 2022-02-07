use super::super::super::geo::{BoundingBox, Coordinate};
use super::super::curve::{BezierCurve, Curve};
use super::path::BezierPath;
use super::to_curves::path_to_curves;

///
/// Finds the bounds of a path
///
pub fn path_bounding_box<P: BezierPath, Bounds: BoundingBox<Point = P::Point>>(path: &P) -> Bounds {
    path_to_curves(path)
        .map(|curve: Curve<P::Point>| curve.bounding_box())
        .reduce(|first: Bounds, second| first.union_bounds(second))
        .unwrap_or_else(|| Bounds::from_min_max(P::Point::origin(), P::Point::origin()))
}

///
/// Finds the bounds of a path using the looser 'fast' algorithm
///
pub fn path_fast_bounding_box<P: BezierPath, Bounds: BoundingBox<Point = P::Point>>(
    path: &P,
) -> Bounds {
    path_to_curves(path)
        .map(|curve: Curve<P::Point>| curve.fast_bounding_box())
        .reduce(|first: Bounds, second| first.union_bounds(second))
        .unwrap_or_else(|| Bounds::from_min_max(P::Point::origin(), P::Point::origin()))
}
