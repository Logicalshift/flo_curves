use super::curve::*;
use super::basis::*;
use super::characteristics::*;
use crate::geo::*;

use smallvec::*;

///
/// Finds the 't' value of the closest point on a curve to the supplied point
///
/// Note that in interactive applications the true 'closest' point may not be the most useful for the user trying to indicate
/// a point on the curve. This is because on the inside of convex regions of the curve, a moving point far enough away will
/// jump between the end points of the convex region. Consider using ray-casting instead via `curve_intersects_ray()` instead
/// to find points that the user might be indicating instead.
///
pub fn nearest_point_on_curve<C>(curve: &C, point: &C::Point) -> f64
where
    C: BezierCurve + BezierCurve2D
{
    nearest_point_on_curve_newton_raphson(curve, point)
}

///
/// Optimises an estimate of a nearest point on a bezier curve using the newton-raphson method
///
pub fn nearest_point_on_curve_newton_raphson<C>(curve: &C, point: &C::Point) -> f64
where
    C: BezierCurve + BezierCurve2D
{
    use CurveFeatures::*;

    // Choose the initial test points based on the curve features
    let test_positions: SmallVec<[f64; 5]> = match curve.features(0.01) {
        Point                           => smallvec![0.0, 0.5, 1.0],
        Linear                          => smallvec![0.0, 0.5, 1.0],
        Arch                            => smallvec![0.0, 0.5, 1.0],
        Parabolic                       => smallvec![0.0, 0.5, 1.0],
        Cusp                            => smallvec![0.0, 0.5, 1.0],
        SingleInflectionPoint(t)        => smallvec![0.0, t/2.0, (1.0-t)/2.0 + t, 1.0],
        DoubleInflectionPoint(t1, t2)   => smallvec![0.0, t1/2.0, (t2-t1)/2.0 + t1, (1.0-t2)/2.0 + t2, 1.0],
        Loop(t1, t2)                    => smallvec![0.0, t1/2.0, (t2-t1)/2.0 + t1, (1.0-t2)/2.0 + t2, 1.0],
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
