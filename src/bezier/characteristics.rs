use super::super::geo::*;
use super::super::line::*;
use super::super::consts::*;

use std::f64;

const SMALL_DIVISOR: f64 = 0.0000001;

///
/// Possible types of a two-dimensional cubic bezier curve
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CurveCategory {
    /// The control points are all at the same position
    Point,

    /// The control points are in a line
    Linear,

    /// A simple curve that does not change direction or self-intersect
    Arch,

    /// A curve that changes direction once
    SingleInflectionPoint,

    /// A curve that changes direction twice
    DoubleInflectionPoint,

    /// A curve that can be represented as a quadratic curve rather than a cubic one
    Parabolic,

    /// A curve with a cusp (an abrupt change in direction)
    Cusp,

    /// A curve containing a loop
    Loop
}

///
/// Computes an affine transform that translates from an arbitrary bezier curve to one that has the first three control points
/// fixed at w1 = (0,0), w2 = (0, 1) and w3 = (1, 1).
/// 
/// Bezier curves maintain their properties when transformed so this provides a curve with equivalent properties to the input
/// curve but only a single free point (w4). This will return 'None' for the degenerate cases: where two points overlap or
/// where the points are collinear.
///
fn canonical_curve_transform<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point) -> Option<(f64, f64, f64, f64, f64, f64)> {
    // Fetch the coordinates
    let (x0, y0) = (w1.x(), w1.y());
    let (x1, y1) = (w2.x(), w2.y());
    let (x2, y2) = (w3.x(), w3.y());

    let a_divisor = (y2-y1)*(x0-x1)-(x2-x1)*(y0-y1);
    if a_divisor.abs() > SMALL_DIVISOR {
        // Transform is:
        // 
        // [ a, b, c ]   [ x ]
        // [ d, e, f ] . [ y ]
        // [ 0, 0, 1 ]   [ 1 ]
        // 
        // This will move w1 to 0,0, w2 to 0, 1 and w3 to 1, 1, which will form our canonical curve that we use for the classification algorithm
        let a = (-(y0-y1)) / a_divisor;
        let b = (-(x0-x1)) / ((x2-x1)*(y0-y1)-(y2-y1)*(x0-x1));
        let c = -a*x0 - b*y0;
        let d = (y1-y2) / ((x0-x1)*(y2-y1) - (x2-x1)*(y0-y1));
        let e = (x1-x2) / ((y0-y1)*(x2-x1) - (y2-y1)*(x0-x1));
        let f = -d*x0 - e*y0;

        Some((a, b, c, d, e, f))
    } else {
        // Is a degenerate case (points overlap or line up)
        None
    }
}

///
/// Converts a set of points to a 'canonical' curve
/// 
/// This is the curve such that w1 = (0.0), w2 = (1, 0) and w3 = (1, 1), if such a curve exists. The return value is the point w4
/// for this curve.
///
fn to_canonical_curve<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point, w4: &Point) -> Option<Point> {
    // Retrieve the affine transform for the curve
    if let Some((a, b, c, d, e, f)) = canonical_curve_transform(w1, w2, w3) {
        // Calculate the free point w4 based on the transform
        let x4  = w4.x();
        let y4  = w4.y();

        let x   = a*x4 + b*y4 + c;
        let y   = d*x4 + e*y4 + f;

        Some(Point::from_components(&[x, y]))
    } else {
        None
    }
}

/*
///
/// Determines the characteristic coefficients A, B, C and Delta for an arbitrary bezier curve
/// 
/// See `A Geometric Characterization of Parametric Cubic Curves` by Stone and DeRose for a description of these values.
///
fn characteristic_coefficients<Point: Coordinate+Coordinate2D>(canonical_end_point: &Point) -> (f64, f64, f64, f64) {
    let x           = canonical_end_point.x();
    let y           = canonical_end_point.y();

    let a           = 9.0*(x + y - 3.0);
    let b           = -9.0*(x - 3.0);
    let c           = -9.0;
    let delta       = x*x - 2.0*x + 4.0*y - 3.0;

    (a, b, c, delta)
}
*/

///
/// Determines the characteristics of a paritcular bezier curve: whether or not it is an arch, or changes directions
/// (has inflection points), or self-intersects (has a loop)
///
pub fn characterize_curve<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point, w4: &Point) -> CurveCategory {
    // b4 is the end point of an equivalent curve with the other control points fixed at (0, 0), (0, 1) and (1, 1) 
    let b4          = to_canonical_curve(w1, w2, w3, w4);

    if let Some(b4) = b4 {
        // These coefficients can be used to characterise the curve
        let x       = b4.x();
        let y       = b4.y();
        let delta   = x*x - 2.0*x + 4.0*y - 3.0;

        if delta.abs() <= f64::EPSILON {
            // Curve has a cusp (but we don't know if it's in the range 0<=t<=1)
            if x <= 1.0 {
                // Cusp is within the curve
                CurveCategory::Cusp
            } else {
                // Cusp is outside of the region of this curve
                CurveCategory::Arch
            }
        } else if delta <= 0.0 {
            // Curve has a loop (but we don't know if it's in the range 0<=t<=1)
            if x > 1.0 {
                // Arch or inflection point
                if y > 1.0 {
                    CurveCategory::SingleInflectionPoint
                } else {
                    CurveCategory::Arch
                }
            } else if x*x - 3.0*x + 3.0*y >= 0.0 {
                if x*x + y*y + x*y - 3.0*x >= 0.0 {
                    // Curve lies within the loop region
                    CurveCategory::Loop
                } else {
                    // Loop is outside of 0<=t<=1 (double point is t < 0)
                    CurveCategory::Arch
                }
            } else {
                // Loop is outside of 0<=t<=1 (double point is t > 1)
                CurveCategory::Arch
            }
        } else {
            if y >= 1.0 {
                CurveCategory::SingleInflectionPoint
            } else if x <= 0.0 {
                CurveCategory:: DoubleInflectionPoint
            } else {
                if (x-3.0).abs() <= f64::EPSILON && (y-0.0).abs() <= f64::EPSILON {
                    CurveCategory::Parabolic
                } else {
                    CurveCategory::Arch
                }
            }
        }
    } else {
        // Degenerate case: there's no canonical form for this curve
        if w2.is_near_to(w3, SMALL_DISTANCE) {
            if w2.is_near_to(w1, SMALL_DISTANCE) {
                if w3.is_near_to(w4, SMALL_DISTANCE) {
                    // All 4 control points at the same position
                    CurveCategory::Point
                } else {
                    // 3 control points at the same position (makes a line)
                    CurveCategory::Linear
                }
            } else if w3.is_near_to(w4, SMALL_DISTANCE) {
                // 3 control points at the same position (makes a line)
                CurveCategory::Linear
            } else {
                // w2 and w3 are the same. If w1, w2, w3 and w4 are collinear then we have a straight line, otherwise we have a curve with an inflection point.
                let line        = (w1.clone(), w3.clone());
                let (a, b, c)   = line_coefficients_2d(&line);

                let distance    = a*w4.x() + b*w4.y() + c;
                if distance.abs() < SMALL_DISTANCE {
                    // w1, w3 and w4 are collinear (and w2 is the same as w3)
                    CurveCategory::Linear
                } else {
                    // Cubic with inflections at t=0 and t=1 (both control points in the same place but start and end point in different places)
                    CurveCategory::DoubleInflectionPoint
                }
            }
        } else {
            // w1, w2, w3 must be collinear (w2 and w3 are known not to overlap)
            let line        = (w2.clone(), w3.clone());
            let (a, b, c)   = line_coefficients_2d(&line);

            let distance    = a*w4.x() + b*w4.y() + c;
            if distance.abs() < SMALL_DISTANCE {
                // All 4 points are in a line
                CurveCategory::Linear
            } else {
                // w2, w3, w4 are not in a line, we can reverse the curve to get a firm result
                characterize_curve(w4, w3, w2, w1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn canonical_curve_coeffs_are_valid_1() {
        // Mapping the three control points via the affine transform should leave them at (0,0), (0,1) and (1,1)
        let w1                  = Coord2(1.0, 1.0);
        let w2                  = Coord2(2.0, 3.0);
        let w3                  = Coord2(5.0, 2.0);

        let (a, b, c, d, e, f)  = canonical_curve_transform(&w1, &w2, &w3).unwrap();

        let w1_new_x            = w1.x()*a + w1.y()*b + c;
        let w1_new_y            = w1.x()*d + w1.y()*e + f;
        let w2_new_x            = w2.x()*a + w2.y()*b + c;
        let w2_new_y            = w2.x()*d + w2.y()*e + f;
        let w3_new_x            = w3.x()*a + w3.y()*b + c;
        let w3_new_y            = w3.x()*d + w3.y()*e + f;

        assert!((w1_new_x-0.0).abs() < 0.0001);
        assert!((w1_new_y-0.0).abs() < 0.0001);

        assert!((w2_new_x-0.0).abs() < 0.0001);
        assert!((w2_new_y-1.0).abs() < 0.0001);

        assert!((w3_new_x-1.0).abs() < 0.0001);
        assert!((w3_new_y-1.0).abs() < 0.0001);
    }

    #[test]
    fn detect_loop_1() {
        let w1 = Coord2(148.0, 151.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Loop);
    }

    #[test]
    fn detect_loop_2() {
        let w1 = Coord2(161.0, 191.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Loop);
    }

    #[test]
    fn detect_loop_3() {
        let w1 = Coord2(205.0, 159.0);
        let w2 = Coord2(81.0, 219.0);
        let w3 = Coord2(287.0, 227.0);
        let w4 = Coord2(205.0, 159.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Loop);
    }

    #[test]
    fn not_loop_1() {
        let w1 = Coord2(219.0, 173.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn not_loop_2() {
        let w1 = Coord2(286.0, 101.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn not_loop_3() {
        let w1 = Coord2(205.0, 159.0);
        let w2 = Coord2(81.0, 219.0);
        let w3 = Coord2(287.0, 227.0);
        let w4 = Coord2(206.0, 159.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn not_loop_4() {
        let w1 = Coord2(215.0, 214.0);
        let w2 = Coord2(123.0, 129.0);
        let w3 = Coord2(72.0, 92.0);
        let w4 = Coord2(48.0, 77.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) != CurveCategory::Loop);
    }

    #[test]
    fn not_loop_5() {
        let w1 = Coord2(215.0, 214.0);
        let w2 = Coord2(123.0, 129.0);
        let w3 = Coord2(72.0, 92.0);
        let w4 = Coord2(48.0, 77.0);

        assert!(characterize_curve(&w4, &w3, &w2, &w1) != CurveCategory::Loop);
    }

    #[test]
    fn cusp_1() {
        let w1 = Coord2(55.0, 200.0);
        let w2 = Coord2(287.0, 227.0);
        let w3 = Coord2(55.0, 227.0);
        let w4 = Coord2(287.0, 200.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Cusp);
    }

    #[test]
    fn single_inflection_1() {
        let w1 = Coord2(278.0, 260.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::SingleInflectionPoint);
    }

    #[test]
    fn arch_1() {
        let w1 = Coord2(65.0, 146.0);
        let w2 = Coord2(95.0, 213.0);
        let w3 = Coord2(249.0, 218.0);
        let w4 = Coord2(256.0, 181.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn arch_2() {
        let w1 = Coord2(11.0, 143.0);
        let w2 = Coord2(156.0, 261.0);
        let w3 = Coord2(23.0, 278.0);
        let w4 = Coord2(24.0, 200.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn double_inflection_1() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(238.0, 232.0);
        let w3 = Coord2(108.0, 233.0);
        let w4 = Coord2(329.0, 129.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::DoubleInflectionPoint);
    }

    #[test]
    fn degenerate_single_point() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(56.0, 162.0);
        let w3 = Coord2(56.0, 162.0);
        let w4 = Coord2(56.0, 162.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Point);
    }

    #[test]
    fn degenerate_horizontal_line() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(64.0, 162.0);
        let w3 = Coord2(72.0, 162.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Linear);
    }

    #[test]
    fn degenerate_horizontal_line_overlapping_control_points() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(64.0, 162.0);
        let w3 = Coord2(64.0, 162.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Linear);
    }

    #[test]
    fn degenerate_line_only_two_control_points() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(56.0, 162.0);
        let w3 = Coord2(56.0, 162.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::Linear);
    }

    #[test]
    fn degenerate_cubic_curve() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(72.0, 172.0);
        let w3 = Coord2(72.0, 172.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::DoubleInflectionPoint);
    }

    #[test]
    fn degenerate_needs_reversal() {
        let w1 = Coord2(55.0, 200.0);
        let w2 = Coord2(131.0, 200.0);
        let w3 = Coord2(290.0, 200.0);
        let w4 = Coord2(290.0, 95.0);

        assert!(characterize_curve(&w1, &w2, &w3, &w4) == CurveCategory::SingleInflectionPoint);
    }
}
