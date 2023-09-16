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
fn section_length<Curve>(section: CurveSection<'_, Curve>, max_error: f64) -> f64
where
    Curve: BezierCurve,
{
    // This algorithm is described in Graphics Gems V IV.7

    // The MIN_ERROR guards against cases where the length of a section fails to converge for some reason
    const MIN_ERROR: f64 = 1e-12;

    // Algorithm is recursive, but we use a vec as a stack to avoid overflowing (and to make the number of iterations easy to count)
    let mut waiting         = vec![(section, max_error)];
    let mut total_length    = 0.0;

    while let Some((section, max_error)) = waiting.pop() {
        // Estimate the error for the length of the curve
        let polygon_length  = control_polygon_length(&section);
        let chord_length    = chord_length(&section);

        let error           = (polygon_length - chord_length) * (polygon_length - chord_length);

        // If the error is low enough, return the estimated length
        if error < max_error || max_error <= MIN_ERROR {
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

///
/// Returns true if a curve is so small it could be considered to represent an individual point
///
/// 'Tiny' in this instance is a curve with a maximum arc length of 1e-6
///
#[inline]
pub fn curve_is_tiny(curve: &impl BezierCurve) -> bool {
    // Distance that we consider 'tiny'
    const MAX_DISTANCE: f64 = 1e-6;

    let (sp, (cp1, cp2), ep) = curve.all_points();

    if sp.is_near_to(&ep, MAX_DISTANCE) {
        // Start point and end point are close together (curve could be a long loop or a short section)

        // Calculate the length of the control polygon
        let total_length = sp.distance_to(&cp1) + cp1.distance_to(&cp2) + cp2.distance_to(&ep);

        // If the control polygon is shorter than MAX_DISTANCE then the whole curve
        total_length <= MAX_DISTANCE
    } else {
        // Start and end point are far apart
        false
    }
}
