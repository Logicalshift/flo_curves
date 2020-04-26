use super::super::curve::*;
use super::super::basis::*;
use super::super::super::geo::*;
use super::super::super::line::*;

use smallvec::*;
use roots::{find_roots_cubic, find_roots_quadratic, Roots};

///
/// Solves the roots for a set of cubic coefficients
///
#[inline]
fn solve_roots(p: (f64, f64, f64, f64)) -> Roots<f64> {
    if p.0.abs() < 0.00000001 {
        if p.1.abs() < 0.00000001 {
            if p.2.abs() < 0.00000001 && p.3.abs() < 0.00000001 {
                // All coefficients 0. Treat the roots as 0, 1 (curve and line are collinear, most likely)
                Roots::Two([0.0, 1.0])
            } else {
                // Solve as a quadratic
                find_roots_quadratic(p.1, p.2, p.3)
            }
        } else {
            // Solve as a quadratic
            find_roots_quadratic(p.1, p.2, p.3)
        }
    } else {
        // Just solve as a cubic
        find_roots_cubic(p.0, p.1, p.2, p.3)
    }
}

///
/// Find the t values where a curve intersects a ray
///
/// Return value is a vector of (curve_t, line_t, intersection_point) values. The `line_t` value can be outside the
/// original line, so this will return all the points on the curve that lie on a line of infinite length.
/// 
pub fn curve_intersects_ray<C: BezierCurve, L: Line<Point=C::Point>>(curve: &C, line: &L) -> SmallVec<[(f64, f64, C::Point); 4]>
where C::Point: Coordinate2D {
    // Based upon https://www.particleincell.com/2013/cubic-line-intersection/

    // Line coefficients
    let (p1, p2)    = line.points();
    let a           = p2.y()-p1.y();
    let b           = p1.x()-p2.x();
    let c           = p1.x()*(p1.y()-p2.y()) + p1.y()*(p2.x()-p1.x());

    if a == 0.0 && b == 0.0 {
        return smallvec![];
    }

    // Bezier coefficients
    let (w2, w3)    = curve.control_points();
    let (w1, w4)    = (curve.start_point(), curve.end_point());
    let bx          = bezier_coefficients(0, &w1, &w2, &w3, &w4);
    let by          = bezier_coefficients(1, &w1, &w2, &w3, &w4);

    let p           = (
        a*bx.0+b*by.0,
        a*bx.1+b*by.1,
        a*bx.2+b*by.2,
        a*bx.3+b*by.3+c
    );

    let roots                       = solve_roots(p);
    let roots: SmallVec<[f64; 4]>   = match roots {
        Roots::No(_)    => smallvec![],
        Roots::One(r)   => SmallVec::from_slice(&r),
        Roots::Two(r)   => SmallVec::from_slice(&r),
        Roots::Three(r) => SmallVec::from_slice(&r),
        Roots::Four(r)  => SmallVec::from_buf(r)
    };

    let mut result = smallvec![];
    for t in roots.into_iter() {
        // Allow a small amount of 'slop' for items at the start/end as the root finding is not exact
        let t =
            if t < 0.0 && t > -0.01 {
                // If the line passes close enough to the start of the curve, set t to 0
                let factor      = (a*a + b*b).sqrt();
                let (a, b, c)   = (a/factor, b/factor, c/factor);
                let start_point = &w1;
                
                if (start_point.x()*a + start_point.y()*b + c).abs() < 0.00000001 {
                    0.0 
                } else {
                    t
                }
            } else if t > 1.0 && t < 1.01 {
                // If the line passes close enough to the end of the curve, set t to 1 
                let factor      = (a*a + b*b).sqrt();
                let (a, b, c)   = (a/factor, b/factor, c/factor);
                let end_point   = &w4;

                if (end_point.x()*a + end_point.y()*b + c).abs() < 0.00000001 {
                    1.0
                } else {
                    t
                }
            } else { t };

        if t >= 0.0 && t <= 1.0 {
            // Calculate the position on the curve
            let pos = de_casteljau4(t, w1, w2, w3, w4);

            // Coordinates on the curve
            let x   = pos.x();
            let y   = pos.y();

            // Solve for the position on the line
            let s = if b.abs() > a.abs() {
                (x-p1.x())/(p2.x()-p1.x())
            } else {
                (y-p1.y())/(p2.y()-p1.y())
            };

            test_assert!(!s.is_nan());
            test_assert!(!s.is_infinite());

            result.push((t, s, pos));
        }
    }

    result
}

///
/// Find the t values where a curve intersects a line
///
/// Return value is a vector of (curve_t, line_t, intersection_point) values
/// 
pub fn curve_intersects_line<C: BezierCurve, L: Line<Point=C::Point>>(curve: &C, line: &L) -> SmallVec<[(f64, f64, C::Point); 4]>
where C::Point: Coordinate2D {
    let mut ray_intersections = curve_intersects_ray(curve, line);
    ray_intersections.retain(|(_t, s, _pos)| s >= &mut 0.0 && s <= &mut 1.0);

    ray_intersections
}
