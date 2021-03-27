use super::curve::*;
use super::section::*;
use crate::geo::*;

///
/// Returns the length of the control polygon for a bezier curve
///
pub fn control_polygon_length<Curve: BezierCurve>(curve: &Curve) -> f64 {
    let p1          = curve.start_point();
    let (p2, p3)    = curve.control_points();
    let p4          = curve.end_point();

    p1.distance_to(&p2)
        + p2.distance_to(&p3)
        + p3.distance_to(&p4)
}

///
/// Returns the length of the chord of a bezier curve
///
pub fn chord_length<Curve: BezierCurve>(curve: &Curve) -> f64 {
    let p1  = curve.start_point();
    let p2  = curve.end_point();

    p1.distance_to(&p2)
}

///
/// Estimates the length of a bezier curve within a particular error tolerance
///
pub fn curve_length<Curve: BezierCurve>(curve: &Curve, max_error: f64) -> f64 {
    section_length(curve.section(0.0, 1.0), max_error)
}

///
/// Computes the length of a section of a bezier curve
///
fn section_length<'a, Curve>(section: CurveSection<'a, Curve>, max_error: f64) -> f64
where
Curve: BezierCurve {
    // This algorithm is described in Graphics Gems V IV.7

    // Algorithm is recursive, but we use a vec as a stack to avoid overflowing (and to make the number of iterations easy to count)
    let mut waiting         = vec![(section, max_error)];
    let mut total_length    = 0.0;

    while let Some((section, max_error)) = waiting.pop() {
        // Estimate the error for the length of the curve
        let polygon_length  = control_polygon_length(&section);
        let chord_length    = chord_length(&section);

        let error           = (polygon_length - chord_length) * (polygon_length - chord_length);

        // If the error is low enough, return the estimated length
        if error < max_error {
            total_length += (2.0*chord_length + 2.0*polygon_length)/4.0;
        } else {
            // Subdivide the curve (each half has half the error tolerance)
            let left                = section.subsection(0.0, 0.5);
            let right               = section.subsection(0.5, 1.0);
            let subsection_error    = max_error / 2.0;

            waiting.push((left, subsection_error));
            waiting.push((right, subsection_error));
        }
    }

    total_length
}
