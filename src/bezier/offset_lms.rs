use super::fit::*;
use super::curve::*;
use super::normal::*;
use super::characteristics::*;
use crate::geo::*;

use smallvec::*;
use std::iter;

///
/// Produces an offset curve by performing a least-mean-square curve fit against the output of a function
///
/// This is about 5x slower than the scaling algorithm with 10 subdivisions (which is a number that seems to
/// produce good results). Too few subdivisions can result in flat sections in the curve, and too many can
/// result in artifacts caused by overfitting.
///
pub fn offset_lms_sampling<Curve, NormalOffsetFn, TangentOffsetFn>(curve: &Curve, normal_offset_for_t: NormalOffsetFn, tangent_offset_for_t: TangentOffsetFn, subdivisions: u32, max_error: f64) -> Option<Vec<Curve>>
where   Curve:              BezierCurveFactory+NormalCurve,
        Curve::Point:       Normalize+Coordinate2D,
        NormalOffsetFn:     Fn(f64) -> f64,
        TangentOffsetFn:    Fn(f64) -> f64 {
    if subdivisions < 2 { return None; }

    // Subdivide the curve by its major features
    let sections: SmallVec<[_; 4]>  = match features_for_curve(curve, 0.01) {
        CurveFeatures::DoubleInflectionPoint(t1, t2)  => {
            let t1 = if t1 > 0.9999 { 1.0 } else if t1 < 0.0001 { 0.0 } else { t1 };
            let t2 = if t2 > 0.9999 { 1.0 } else if t2 < 0.0001 { 0.0 } else { t2 };

            if t2 > t1 {
                smallvec![(0.0, t1), (t1, t2), (t2, 1.0)]
            } else {
                smallvec![(0.0, t2), (t2, t1), (t1, 1.0)]
            }
        }

        CurveFeatures::Loop(t1, t3) => {
            let t1 = if t1 > 0.9999 { 1.0 } else if t1 < 0.0001 { 0.0 } else { t1 };
            let t3 = if t3 > 0.9999 { 1.0 } else if t3 < 0.0001 { 0.0 } else { t3 };
            let t2 = (t1+t3)/2.0;

            if t3 > t1 {
                smallvec![(0.0, t1), (t1, t2), (t2, t3), (t3, 1.0)]
            } else {
                smallvec![(0.0, t3), (t3, t2), (t2, t1), (t1, 1.0)]
            }
        }

        CurveFeatures::SingleInflectionPoint(t) => {
            if t > 0.0001 && t < 0.9999 {
                smallvec![(0.0, t), (t, 1.0)]
            } else {
                smallvec![(0.0, 1.0)]
            }
        }

        _ => { smallvec![(0.0, 1.0)] }
    };

    // Each section is subdivided in turn subdivisions times to produce a set of sample points to fit against
    let sections            = sections.into_iter()
        .filter(|(t1, t2)| t1 != t2)
        .flat_map(|(t1, t2)| {
            let step = (t2-t1)/(subdivisions as f64);
            (0..subdivisions).into_iter().map(move |x| t1 + step * (x as f64))
        })
        .chain(iter::once(1.0));

    // Take a sample at each point
    let sample_points       = sections
        .map(|t| {
            let original_point  = curve.point_at_pos(t);
            let unit_tangent    = curve.tangent_at_pos(t).to_unit_vector();
            let unit_normal     = Curve::Point::to_normal(&original_point, &unit_tangent);
            let unit_normal     = Curve::Point::from_components(&unit_normal);
            let normal_offset   = normal_offset_for_t(t);
            let tangent_offset  = tangent_offset_for_t(t);

            original_point + (unit_normal * normal_offset) + (unit_tangent * tangent_offset)
        })
        .collect::<Vec<_>>();

    // Generate a curve using the sample points
    let start_tangent   = curve.tangent_at_pos(0.0).to_unit_vector();
    let end_tangent     = curve.tangent_at_pos(1.0).to_unit_vector() * -1.0;
    Some(fit_curve_cubic(&sample_points, &start_tangent, &end_tangent, max_error))
}
