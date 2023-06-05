use crate::geo::*;
use crate::bezier::*;
use crate::line::*;

use smallvec::*;

///
/// Counts the number of times a bezier curve polygon crosses the x-axis (excluding the closing line of the polygon)
///
#[inline]
fn count_x_axis_crossings<TPoint, const N: usize>(points: &[TPoint; N]) -> usize
where
    TPoint: Coordinate + Coordinate2D,
{
    let mut num_crossings = 0;

    for idx in 0..(N-1) {
        let p1 = &points[idx];
        let p2 = &points[idx+1];

        if p1.y() < 0.0 && p2.y() >= 0.0        { num_crossings += 1; }
        else if p1.y() >= 0.0 && p2.y() < 0.0   { num_crossings += 1; }
    }

    return num_crossings;
}

///
/// Returns true if the control polygon is flat enough to try to find a root for it
///
#[inline]
fn flat_enough<TPoint, const N: usize>(points: &[TPoint; N]) -> bool 
where
    TPoint: Coordinate + Coordinate2D,
{
    const FLAT_ENOUGH: f64 = 0.1;

    // x coordinates increase monotonically so we just check that the y-components are all in the same direction
    let y_direction = (points[1].y() - points[0].y()).signum();
    for idx in 1..(N-1) {
        if (points[idx+1].y()-points[idx].y()).signum() != y_direction {
            return false;
        }
    }

    // Measure the distance from each control point to the baseline
    let baseline        = (TPoint::from_components(&[points[0].x(), points[0].y()]), TPoint::from_components(&[points[N-1].x(), points[N-1].y()]));
    let baseline_coeff  = baseline.coefficients();
    let mut max_distance: f64 = 0.0;

    // Find the furthest point from the baseline
    for p in points.iter() {
        let distance = baseline_coeff.distance_to(p);
        max_distance = max_distance.max(distance);
    }

    // The graphics gems code goes on to compute a bounding box to get a precise estimate of the maximum error, here we just use the furthest away control point as a measure of flatness
    return max_distance <= FLAT_ENOUGH;
}

///
/// Finds an x-intercept for a bezier curve that is 'flat enough', returning the t-value for the resulting point
///
#[inline]
fn find_x_intercept<TPoint, const N: usize>(points: &[TPoint; N]) -> f64 
where
    TPoint: Coordinate + Coordinate2D,
{
    // Pick a guess by finding where the baseline intercepts the y-axis
    let baseline        = (TPoint::from_components(&[points[0].x(), points[0].y()]), TPoint::from_components(&[points[N-1].x(), points[N-1].y()]));
    let coefficients    = baseline.coefficients();

    // Want the intercept point, relative to the current section of curve
    let t_guess         = -coefficients.2 / coefficients.0;
    let t_guess         = (t_guess-baseline.0.x()) / (baseline.1.x()-baseline.0.x());

    // Use newton-raphson to find the intercept
    let points  = points.iter().map(|point| point.y()).collect::<SmallVec<[f64; N]>>();
    let root    = find_x_intercept_newton_raphson(points, t_guess);

    root
}

///
/// Optimises an estimate of a nearest point on a bezier curve using the newton-raphson method
///
pub fn find_x_intercept_newton_raphson<const N: usize>(points: SmallVec<[f64; N]>, estimated_t: f64) -> f64 {
    const EPSILON: f64          = 1e-10;
    const MAX_ITERATIONS: usize = 30;

    // Find the derivative of the points
    let derivative      = derivative_n(points.clone());

    // Will update the estimate every iteration
    let mut estimated_t = estimated_t;

    for _ in 0..MAX_ITERATIONS {
        // Compute f(t) and f'(t)
        let numerator   = de_casteljau_n(estimated_t, points.clone());
        let denominator = de_casteljau_n(estimated_t, derivative.clone());

        if numerator.abs() <= EPSILON {
            return estimated_t;
        }

        if denominator == 0.0 {
            // Failed to converge due to hitting a singularity
            return estimated_t;
        }

        // t = t - (f(t)/f'(t))
        estimated_t = estimated_t - (numerator / denominator);
    }

    // Just use the best guess if we don't converge enough
    estimated_t
}

///
/// Finds the points (as t-values) where a bezier curve's y coordinate is 0
///
/// When combined with `polynomial_to_bezier` this can be used to find all of the roots for any polynomial, provided
/// that they lie in the range `0.0..1.0`. For higher-order polynomials, the maximum precision of the f64 type may
/// start to limit the effectiveness of this function.
///
pub fn find_bezier_roots<TPoint, const N: usize>(points: [TPoint; N]) -> SmallVec<[f64; 4]>
where
    TPoint: Coordinate + Coordinate2D,
{
    // See "A bezier curve-based root-finder", Philip J Schneider, Graphics Gems

    // List of sections waiting to be processed
    let mut sections    = vec![points];
    let mut roots       = smallvec![];

    loop {
        // Get the next section to process
        let section = if let Some(section) = sections.pop() { section } else { return roots; };

        // Find out how many times the polygon crosses the x
        let num_crossings = count_x_axis_crossings(&section);

        if num_crossings == 0 {
            // No roots if the control polygon does not cross the x-axis
            continue;
        }

        if num_crossings == 1 && flat_enough(&section) {
            // Find an x-intercept for this section
            let intercept = find_x_intercept(&section);
            roots.push(de_casteljau_n(intercept, section.into()).x());
            continue;
        }

        // Subdivide the curve in the middle to search for more crossings
        let (left, right) = subdivide_n(0.5, section);
        sections.push(right);
        sections.push(left);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::polynomial_to_bezier::*;

    #[test]
    fn find_roots_simple_polynomial() {
        // (x-0.5)(x-0.4)(x-0.3)(x-0.2)(x-0.1)
        //  == -0.0012 + 0.0274x - 0.225x^2  + 0.85x^3 - 1.5x^4 + x^5
        let bezier  = polynomial_to_bezier::<Coord2, 6>([-0.0012, 0.0274, -0.225, 0.85, -1.5, 1.0]);
        let roots   = find_bezier_roots(bezier);

        debug_assert!(roots.len() == 5, "{:?}", roots);
        debug_assert!((roots[0]-0.1).abs() < 0.001, "{:?}", roots);
        debug_assert!((roots[1]-0.2).abs() < 0.001, "{:?}", roots);
        debug_assert!((roots[2]-0.3).abs() < 0.001, "{:?}", roots);
        debug_assert!((roots[3]-0.4).abs() < 0.001, "{:?}", roots);
        debug_assert!((roots[4]-0.5).abs() < 0.001, "{:?}", roots);
    }
}
