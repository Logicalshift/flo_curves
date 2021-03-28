use super::fit::*;
use super::walk::*;
use super::curve::*;
use super::normal::*;
use crate::geo::*;

use std::iter;

///
/// Distorts a curve using an arbitrary function
///
pub fn distort_curve<CurveIn, DistortFn, CurveOut>(curve: &CurveIn, distort_fn: DistortFn, step_len: f64, max_error: f64) -> Option<Vec<CurveOut>>
where
CurveIn:        BezierCurve,
CurveIn::Point: Normalize+Coordinate2D,
CurveOut:       BezierCurveFactory<Point=CurveIn::Point>,
DistortFn:      Fn(CurveIn::Point, f64) -> CurveIn::Point {
    // Walk the curve at roughly step_len increments
    let sections    = walk_curve_evenly(curve, step_len, step_len / 4.0);

    // Generate the points to fit to using the distortion function
    let fit_points  = sections.map(|section| {
            let (t, _)  = section.original_curve_t_values();
            let pos     = curve.point_at_pos(t);

            (pos, t)
        })
        .chain(iter::once((curve.point_at_pos(1.0), 1.0)))
        .map(|(point, t)| distort_fn(point, t))
        .collect::<Vec<_>>();

    // Fit the points to generate the final curve
    fit_curve(&fit_points, max_error)
}
