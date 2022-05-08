use super::fit::*;
use super::walk::*;
use super::path::*;
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
    DistortFn:      Fn(CurveIn::Point, f64) -> CurveOut::Point,
{
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

///
/// Distorts a path using an arbitrary function
///
pub fn distort_path<PathIn, DistortFn, PathOut>(path: &PathIn, distort_fn: DistortFn, step_len: f64, max_error: f64) -> Option<PathOut> 
where
    PathIn:     BezierPath,
    PathOut:    BezierPathFactory<Point=PathIn::Point>,
    DistortFn:  Fn(PathIn::Point, &Curve<PathIn::Point>, f64) -> PathOut::Point,
{
    // The initial point is derived from the first curve
    let start_point         = path.start_point();
    let mut path_points     = path.points();
    let mut current_point   = path_points.next()?;
    let mut current_curve   = Curve::from_points(start_point, (current_point.0, current_point.1), current_point.2);
    let start_point         = distort_fn(start_point, &current_curve, 0.0);

    // Process the remaining points to generate the new path
    let mut new_points      = vec![];

    loop {
        // Distort the current curve
        let sections    = walk_curve_evenly(&current_curve, step_len, step_len / 4.0);

        let fit_points  = sections.map(|section| {
                let (t, _)  = section.original_curve_t_values();
                let pos     = current_curve.point_at_pos(t);

                (pos, t)
            })
            .chain(iter::once((current_curve.point_at_pos(1.0), 1.0)))
            .map(|(point, t)| distort_fn(point, &current_curve, t))
            .collect::<Vec<_>>();

        // Fit the points to generate the new curves
        let new_curves      = fit_curve::<Curve<_>>(&fit_points, max_error)?;
        new_points.extend(new_curves.into_iter().map(|curve| {
            let (cp1, cp2) = curve.control_points();
            (cp1, cp2, curve.end_point())
        }));

        // Move to the next curve (stopping once we reach the end of the list of the points)
        let next_start_point    = current_curve.end_point();
        current_point           = if let Some(point) = path_points.next() { point } else { break; };
        current_curve           = Curve::from_points(next_start_point, (current_point.0, current_point.1), current_point.2);
    }

    // Create the new path from the result
    Some(PathOut::from_points(start_point, new_points))
}
