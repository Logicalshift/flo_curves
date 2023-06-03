use crate::geo::*;
use crate::bezier::*;
use crate::line::*;

use smallvec::*;
use ::roots::{find_root_newton_raphson, SimpleConvergency};

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

    // Measure the distance from each control point to the baseline
    let baseline = (TPoint::from_components(&[points[0].x(), points[0].y()]), TPoint::from_components(&[points[N-1].x(), points[N-1].y()]));
    let mut max_distance: f64 = 0.0;

    // Find the furthest point from the baseline
    for p in points.iter() {
        let distance = baseline.distance_to(p);
        max_distance = max_distance.max(distance);
    }

    // The graphics gems code goes on to compute a bounding box to get a precise estimate of the maximum error, here we just use the furthest away control point as a measure of flatness
    return max_distance <= FLAT_ENOUGH;
}

///
/// Finds an x-intercept for a bezier curve that is 'flat enough', returning the t-value for the resulting point
///
#[inline]
fn find_x_intercept<TPoint, const N: usize>(t_guess: f64, points: &[TPoint; N]) -> f64 
where
    TPoint: Coordinate + Coordinate2D,
{
    // Use newton-raphson to find the intercept
    let points      = points.iter().map(|point| point.y()).collect::<SmallVec<[f64; N]>>();
    let derivative  = derivative_n(points.clone());

    let mut convergency = SimpleConvergency { eps: 1e-15f64, max_iter: 10 };
    let root            = find_root_newton_raphson(t_guess, move |t| de_casteljau_n(t, points.clone()), move |t| de_casteljau_n(t, derivative.clone()), &mut convergency);

    root.unwrap()
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
            let intercept = find_x_intercept(0.5, &section);
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
