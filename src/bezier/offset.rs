use super::super::geo::Coordinate2D;
use super::curve::BezierCurveFactory;
use super::normal::{NormalCurve, Normalize};
use super::offset_scaling::offset_scaling;

///
/// Computes a series of curves that approximate an offset curve from the specified origin curve.
///
pub fn offset<Curve>(curve: &Curve, initial_offset: f64, final_offset: f64) -> Vec<Curve>
where
    Curve: BezierCurveFactory + NormalCurve,
    Curve::Point: Normalize + Coordinate2D,
{
    offset_scaling(curve, initial_offset, final_offset)
}
