use super::curve::*;
use super::normal::*;
use super::offset_lms::*;
use super::super::geo::*;

///
/// Computes a series of curves that approximate an offset curve from the specified origin curve.
///
pub fn offset<Curve>(curve: &Curve, initial_offset: f64, final_offset: f64) -> Vec<Curve>
where
    Curve:          BezierCurveFactory+NormalCurve,
    Curve::Point:   Normalize+Coordinate2D,
{
    offset_lms_sampling(curve, move |t| (final_offset - initial_offset) * t + initial_offset, |_| 0.0, 32, 0.1)
        .unwrap_or_else(|| vec![])
}
