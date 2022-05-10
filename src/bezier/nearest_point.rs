use super::curve::*;
use super::basis::*;
use super::section::*;
use super::characteristics::*;
use crate::geo::*;
use crate::line::*;

///
/// Optimises an estimate of a nearest point on a bezier curve using the newton-raphson method
///
pub fn nearest_point_on_curve_newton_raphson<C>(curve: &C, point: &C::Point) -> f64
where
    C: BezierCurve + BezierCurve2D
{
    use CurveFeatures::*;

    // Choose the initial test points based on the curve features
    let test_positions = match curve.features(0.01) {
        Point                           => vec![0.5],
        Linear                          => vec![0.5],
        Arch                            => vec![0.5],
        Parabolic                       => vec![0.5],
        Cusp                            => vec![0.5],
        SingleInflectionPoint(t)        => vec![t/2.0, (1.0-t)/2.0 + t],
        DoubleInflectionPoint(t1, t2)   => vec![t1/2.0, (t2-t1)/2.0 + t1, (1.0-t2)/2.0 + t2],
        Loop(t1, t2)                    => vec![t1/2.0, (t2-t1)/2.0 + t1, (1.0-t2)/2.0 + t2],
    };

    // Find the test point nearest to the point we're trying to get the nearest point for
    let mut estimated_t     = 0.5;
    let mut min_distance    = f64::MAX;

    for t in test_positions {
        let curve_pos   = curve.point_at_pos(t);
        let offset      = *point - curve_pos;
        let distance_sq = offset.dot(&offset);

        if distance_sq < min_distance {
            estimated_t = t;
            min_distance = distance_sq;
        }
    }

    // Optimise the guess
    nearest_point_on_curve_newton_raphson_with_estimate(curve, point, estimated_t)    
}

///
/// Optimises an estimate of a nearest point on a bezier curve using the newton-raphson method
///
pub fn nearest_point_on_curve_newton_raphson_with_estimate<C>(curve: &C, point: &C::Point, estimated_t: f64) -> f64
where
    C: BezierCurve
{
    // This uses the fact that the nearest point must be perpendicular to the curve, so it optimises for the point where
    // the tangent to the curve is at 90 degrees to the vector to the point
    const EPSILON: f64 = 1e-8;

    // Get the control vertices for the curves
    let q1          = curve.start_point();
    let q4          = curve.end_point();
    let (q2, q3)    = curve.control_points();
    
    // Generate control vertices for the derivatives
    let qn1         = (q2-q1)*3.0;
    let qn2         = (q3-q2)*3.0;
    let qn3         = (q4-q3)*3.0;

    let qnn1        = (qn2-qn1)*2.0;
    let qnn2        = (qn3-qn2)*2.0;

    let mut estimated_t = estimated_t;

    // Attempt to optimise the solution with up to 12 rounds of newton-raphson
    for _ in 0..12 {
        // Determine the quality of the guess
        if estimated_t < -0.01 { return 0.0; }
        if estimated_t > 1.01 { return 1.0; }

        // Compute Q(t) (where Q is our curve)
        let qt          = de_casteljau4(estimated_t, q1, q2, q3, q4);

        // Compute Q'(t) and Q''(t)
        let qnt         = de_casteljau3(estimated_t, qn1, qn2, qn3);
        let qnnt        = de_casteljau2(estimated_t, qnn1, qnn2);

        // Compute f(u)/f'(u)
        let numerator   = (qt-*point).dot(&qnt);
        let denominator = qnt.dot(&qnt) + (qt-*point).dot(&qnnt);

        // The numerator will converge to 0 as the guess improves
        if numerator.abs() < EPSILON { 
            return estimated_t;
        }

        // u = u - f(u)/f'(u)
        let next_t = if denominator == 0.0 {
            // Found a singularity
            return estimated_t;
        } else {
            estimated_t - (numerator/denominator)
        };

        // Update the guess for the next iteration
        estimated_t = next_t;
    }

    estimated_t
}

///
/// Computes the t position of the nearest point on a curve using a subdivision algorithm
///
pub fn nearest_point_on_curve_subdivision<C, P>(curve: &C, point: &P, precision: f64) -> f64
where
    C:          BezierCurve<Point=P>,
    C::Point:   Coordinate2D,
    P:          Coordinate + Coordinate2D,
{
    nearest_point_on_curve_subsection(curve.section(0.0, 1.0), point, precision)
}

///
/// Returns the minimum and maximum distances between a point and a curve section
///
fn distances_for_section<C, P>(section: CurveSection<'_, C>, point: &P) -> (f64, f64)
where
    C:          BezierCurve<Point=P>,
    C::Point:   Coordinate2D,
    P:          Coordinate + Coordinate2D,
{
    let start_point     = section.start_point();
    let end_point       = section.end_point();
    let (cp1, cp2)      = section.control_points();

    // Decide on the top and bottom lines of the section
    let baseline        = (start_point, end_point);
    let (top, bottom)   = if baseline.which_side(&cp1) == baseline.which_side(&cp2) {
        // Both control points on the same side as the baseline
        (baseline, (cp1, cp2))
    } else {
        // Control points on opposite sides
        ((baseline.0, cp2), (cp1, baseline.1))
    };

    // Distance to the two parts of the section
    let distance_top    = top.point_at_pos(0.5).distance_to(point);
    let distance_bottom = bottom.point_at_pos(0.5).distance_to(point);

    (f64::min(distance_top, distance_bottom), f64::max(distance_top, distance_bottom))
}

///
/// `nearest_point_on_curve_subdivision` but operating on curve sections 
///
fn nearest_point_on_curve_subsection<C, P>(curve: CurveSection<'_, C>, point: &P, precision: f64) -> f64
where
    C:          BezierCurve<Point=P>,
    C::Point:   Coordinate2D,
    P:          Coordinate + Coordinate2D,
{
    // This is a fairly straightforward algorithm: we know the curve must lie between the lines formed by the start and end points and the control points,
    // so we search for the closest point by taking the distance to those two lines as a minimum and maximum value for the distance of the point and discard
    // sections when the two values are outside of that range (or assume we've found a point when the range is within the precision value)
    //
    // This algorithm is not especially fast, but it should be reasonably reliable.
    let (min, max)          = distances_for_section(curve.clone(), point);
    let mut section_stack   = vec![(curve, min, max)];

    let mut candidates      = vec![];

    while let Some((section, _min, max)) = section_stack.pop() {
        // Remove any section from the stack that is definitely further away than this section
        section_stack.retain(|(_section, section_min, _section_max)| *section_min < max);

        // Divide the section into 2
        let section_1       = section.subsection(0.0, 0.5);
        let section_2       = section.subsection(0.5, 1.0);

        // Calculate the distances to the hull of the two sections
        let (min_1, max_1)  = distances_for_section(section_1.clone(), point);
        let (min_2, max_2)  = distances_for_section(section_2.clone(), point);

        // If the min/max values are within precision of each other, then we may have found a close point
        if section.start_point().distance_to(&section.end_point()) <= precision {
            if (max_1 - min_1).abs() <= precision {
                candidates.push(section_1.t_for_t(0.5));
                continue;
            }

            if (max_2 - min_2).abs() <= precision {
                candidates.push(section_2.t_for_t(0.5));
                continue;
            }
        }

        // Add the two sections to consideration (or just the closest section if they don't overlap)
        if min_1 < max_2 {
            section_stack.push((section_1, min_1, max_1));
        }

        if min_2 < max_1 {
            section_stack.push((section_2, min_2, max_2));
        }
    }

    candidates.pop().unwrap_or(0.0)
}
