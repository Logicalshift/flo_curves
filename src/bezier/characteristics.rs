use super::curve::*;
use super::intersection::*;
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
/// Describes the features of a two-dimensional cubic bezier curve
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CurveFeatures {
    /// All control points are in the same position
    Point,

    /// The control points are in a line
    Linear,

    /// A simple curve that does not change direction or self-intersect
    Arch,

    /// A curve that changes direction once (and the t value where this occurs)
    SingleInflectionPoint(f64),

    /// A curve that changes direction twice (and the t value where this occurs)
    DoubleInflectionPoint(f64, f64),

    /// A curve that can be represented as a quadratic curve rather than a cubic one
    Parabolic,

    /// A curve with a cusp
    Cusp,

    /// A curve containing a loop and the two t values where it self-intersects
    Loop(f64, f64)
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

///
/// Returns the category of a curve given its characteristic point in the canonical form
///
#[inline]
fn characterize_from_canonical_point(b4: (f64, f64)) -> CurveCategory {
    // These coefficients can be used to characterise the curve
    let (x, y)  = b4;
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
            CurveCategory::DoubleInflectionPoint
        } else {
            if (x-3.0).abs() <= f64::EPSILON && (y-0.0).abs() <= f64::EPSILON {
                CurveCategory::Parabolic
            } else {
                CurveCategory::Arch
            }
        }
    }
}

///
/// Determines the characteristics of a particular bezier curve: whether or not it is an arch, or changes directions
/// (has inflection points), or self-intersects (has a loop)
///
pub fn characterize_cubic_bezier<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point, w4: &Point) -> CurveCategory {
    // b4 is the end point of an equivalent curve with the other control points fixed at (0, 0), (0, 1) and (1, 1) 
    let b4          = to_canonical_curve(w1, w2, w3, w4);

    if let Some(b4) = b4 {
        let x       = b4.x();
        let y       = b4.y();

        characterize_from_canonical_point((x, y))
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
            // If w4 is also co-linear then the result is a line. If the points w1,w2,w3 are co-linear and w4,w3,w2 are co-linear
            // then all 4 points must therefore be co-linear.
            let b1 = to_canonical_curve(w4, w3, w2, w1);

            if let Some(b1) = b1 {
                // w4 is not co-linear with w1, w2, w3
                let x       = b1.x();
                let y       = b1.y();

                characterize_from_canonical_point((x, y))
            } else {
                // All four points are co-linear (to the precision allowed by SMALL_DIVISOR)
                CurveCategory::Linear
            }
        }
    }
}

///
/// The location of the inflection points for a curve (t-values)
///
enum InflectionPoints {
    Zero,
    One(f64),
    Two(f64, f64)
}

///
/// Finds the inflection points for a curve that has been reduced to our canonical form, given the free point b4
///
fn find_inflection_points(b4: (f64, f64)) -> InflectionPoints {
    // Compute coefficients
    let (x4, y4)    = b4;
    let a           = -3.0+x4+y4;
    let b           = 3.0-x4;

    if a.abs() <= f64::EPSILON {
        // No solution
        InflectionPoints::Zero
    } else {
        // Solve the quadratic for this curve
        let lhs = (-b)/(2.0*a);
        let rhs = (4.0*a + b*b).sqrt()/(2.0*a);

        let t1  = lhs - rhs;
        let t2  = lhs + rhs;

        // Want points between 0 and 1
        if t1 < 0.0 || t1 > 1.0 {
            if t2 < 0.0 || t2 > 1.0 {
                InflectionPoints::Zero
            } else {
                InflectionPoints::One(t2)
            }
        } else {
            if t2 < 0.0 || t2 > 1.0 {
                InflectionPoints::One(t1)
            } else {
                InflectionPoints::Two(t1, t2)
            }
        }
    }
}

impl Into<CurveFeatures> for InflectionPoints {
    #[inline]
    fn into(self) -> CurveFeatures {
        match self {
            InflectionPoints::Zero          => CurveFeatures::Arch,
            InflectionPoints::One(t)        => CurveFeatures::SingleInflectionPoint(t),
            InflectionPoints::Two(t1, t2)   => CurveFeatures::DoubleInflectionPoint(t1, t2)
        }
    }
}

///
/// Returns the features from a curve where we have discovered the canonical point
///
fn features_from_canonical_point<Point: Coordinate+Coordinate2D>(x: f64, y: f64, w1: &Point, w2: &Point, w3: &Point, w4: &Point, accuracy: f64) -> CurveFeatures {
    match characterize_from_canonical_point((x, y)) {
        CurveCategory::Arch                     => CurveFeatures::Arch,
        CurveCategory::Linear                   => CurveFeatures::Linear,
        CurveCategory::Cusp                     => CurveFeatures::Cusp,
        CurveCategory::Parabolic                => CurveFeatures::Parabolic,
        CurveCategory::Point                    => CurveFeatures::Point,
        CurveCategory::DoubleInflectionPoint    |
        CurveCategory::SingleInflectionPoint    => find_inflection_points((x, y)).into(),
        CurveCategory::Loop                     => {
            let curve       = Curve::from_points(w1.clone(), (w2.clone(), w3.clone()), w4.clone());
            let loop_pos    = find_self_intersection_point(&curve, accuracy);

            // TODO: if we can't find the loop_pos, we could probably find a cusp position instead
            loop_pos.map(|(t1, t2)| CurveFeatures::Loop(t1, t2))
                .unwrap_or(CurveFeatures::Arch)
        }
    }
}

///
/// Determines the characteristics of a paritcular bezier curve: whether or not it is an arch, or changes directions
/// (has inflection points), or self-intersects (has a loop)
///
pub fn features_for_cubic_bezier<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point, w4: &Point, accuracy: f64) -> CurveFeatures {
    // b4 is the end point of an equivalent curve with the other control points fixed at (0, 0), (0, 1) and (1, 1) 
    let b4          = to_canonical_curve(w1, w2, w3, w4);

    if let Some(b4) = b4 {
        // For the inflection points, we rely on the fact that the canonical curve is generated by an affine transform of the original
        // (and the features are invariant in such a situation)
        let x       = b4.x();
        let y       = b4.y();

        features_from_canonical_point(x, y, w1, w2, w3, w4, accuracy)
    } else {
        // Degenerate case: there's no canonical form for this curve
        if w2.is_near_to(w3, SMALL_DISTANCE) {
            if w2.is_near_to(w1, SMALL_DISTANCE) {
                if w3.is_near_to(w4, SMALL_DISTANCE) {
                    // All 4 control points at the same position
                    CurveFeatures::Point
                } else {
                    // 3 control points at the same position (makes a line)
                    CurveFeatures::Linear
                }
            } else if w3.is_near_to(w4, SMALL_DISTANCE) {
                // 3 control points at the same position (makes a line)
                CurveFeatures::Linear
            } else {
                // w2 and w3 are the same. If w1, w2, w3 and w4 are collinear then we have a straight line, otherwise we have a curve with an inflection point.
                let line        = (w1.clone(), w3.clone());
                let (a, b, c)   = line_coefficients_2d(&line);

                let distance    = a*w4.x() + b*w4.y() + c;
                if distance.abs() < SMALL_DISTANCE {
                    // w1, w3 and w4 are collinear (and w2 is the same as w3)
                    CurveFeatures::Linear
                } else {
                    // Cubic with inflections at t=0 and t=1 (both control points in the same place but start and end point in different places)
                    CurveFeatures::DoubleInflectionPoint(0.0, 1.0)
                }
            }
        } else {
            // w1, w2, w3 must be collinear (w2 and w3 are known not to overlap)
            // w4 may or may not be co-linear: determine the features of the curve when reversed
            let b1          = to_canonical_curve(w4, w3, w2, w1);

            if let Some(b1) = b1 {
                // w4 is not co-linear with w2 and w3
                let x       = b1.x();
                let y       = b1.y();

                // Reverse the curve coordinates for the features
                match features_from_canonical_point(x, y, w1, w2, w3, w4, accuracy) {
                    CurveFeatures::SingleInflectionPoint(t)         => CurveFeatures::SingleInflectionPoint(1.0-t),
                    CurveFeatures::DoubleInflectionPoint(t1, t2)    => CurveFeatures::DoubleInflectionPoint(1.0-t1, 1.0-t2),
                    CurveFeatures::Loop(t1, t2)                     => CurveFeatures::Loop(1.0-t1, 1.0-t2),
                    other                                           => other
                }
            } else {
                // w1, w2 and w3 are co-linear and w2, w3 and w4 are co-linear, so all of w1, w2, w3 and w4 must be along the same line
                CurveFeatures::Linear
            }
        }
    }
}

///
/// Discovers the 'character' of a particular bezier curve, returning a value indicating what kinds of features
/// it has (for example, whether it has a loop or a cusp) 
///
#[inline]
pub fn characterize_curve<C: BezierCurve>(curve: &C) -> CurveCategory
where C::Point: Coordinate+Coordinate2D {
    let start_point = curve.start_point();
    let (cp1, cp2)  = curve.control_points();
    let end_point   = curve.end_point();

    characterize_cubic_bezier(&start_point, &cp1, &cp2, &end_point)
}

///
/// Discovers what kind of features a curve has and where they are located
///
#[inline]
pub fn features_for_curve<C: BezierCurve>(curve: &C, accuracy: f64) -> CurveFeatures
where C::Point: Coordinate+Coordinate2D {
    let start_point = curve.start_point();
    let (cp1, cp2)  = curve.control_points();
    let end_point   = curve.end_point();

    features_for_cubic_bezier(&start_point, &cp1, &cp2, &end_point, accuracy)
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

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Loop);
    }

    #[test]
    fn detect_loop_1_feature() {
        let w1 = Coord2(148.0, 151.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        match features_for_cubic_bezier(&w1, &w2, &w3, &w4, 0.01) {
            CurveFeatures::Loop(t1, t2) => {
                let curve = Curve::from_points(w1, (w2, w3), w4);
                assert!(curve.point_at_pos(t1).is_near_to(&curve.point_at_pos(t2), 0.01));
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn detect_loop_2() {
        let w1 = Coord2(161.0, 191.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Loop);
    }

    #[test]
    fn detect_loop_3() {
        let w1 = Coord2(205.0, 159.0);
        let w2 = Coord2(81.0, 219.0);
        let w3 = Coord2(287.0, 227.0);
        let w4 = Coord2(205.0, 159.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Loop);
    }

    #[test]
    fn not_loop_1() {
        let w1 = Coord2(219.0, 173.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn not_loop_2() {
        let w1 = Coord2(286.0, 101.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn not_loop_3() {
        let w1 = Coord2(205.0, 159.0);
        let w2 = Coord2(81.0, 219.0);
        let w3 = Coord2(287.0, 227.0);
        let w4 = Coord2(206.0, 159.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn not_loop_4() {
        let w1 = Coord2(215.0, 214.0);
        let w2 = Coord2(123.0, 129.0);
        let w3 = Coord2(72.0, 92.0);
        let w4 = Coord2(48.0, 77.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) != CurveCategory::Loop);
    }

    #[test]
    fn not_loop_5() {
        let w1 = Coord2(215.0, 214.0);
        let w2 = Coord2(123.0, 129.0);
        let w3 = Coord2(72.0, 92.0);
        let w4 = Coord2(48.0, 77.0);

        assert!(characterize_cubic_bezier(&w4, &w3, &w2, &w1) != CurveCategory::Loop);
    }

    #[test]
    fn cusp_1() {
        let w1 = Coord2(55.0, 200.0);
        let w2 = Coord2(287.0, 227.0);
        let w3 = Coord2(55.0, 227.0);
        let w4 = Coord2(287.0, 200.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Cusp);
    }

    #[test]
    fn single_inflection_1() {
        let w1 = Coord2(278.0, 260.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::SingleInflectionPoint);
    }

    fn is_inflection_point<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point, w4: &Point, t: f64) -> bool {
        let a = 3.0 * (w2.x() - w1.x());
        let b = 3.0 * (w3.x() - w2.x());
        let c = 3.0 * (w4.x() - w3.x());
        let u = 2.0 * (b - a);
        let v = 2.0 * (c - b);

        let d = 3.0 * (w2.y() - w1.y());
        let e = 3.0 * (w3.y() - w2.y());
        let f = 3.0 * (w4.y() - w3.y());
        let w = 2.0 * (e - d);
        let z = 2.0 * (f - e);

        let bx1 = a * (1.0-t)*(1.0-t) + 2.0 * b * (1.0-t)*t + c * t*t;
        let bx2 = u * (1.0-t) + v*t;
        let by1 = d * (1.0-t)*(1.0-t) + 2.0 * e * (1.0-t)*t + f * t*t;
        let by2 = w * (1.0-t) + z*t;

        let curvature = bx1*by2 - by1*bx2;
        curvature.abs() < 0.0001
    }

    #[test]
    fn single_inflection_1_feature() {
        let w1 = Coord2(278.0, 260.0);
        let w2 = Coord2(292.0, 199.0);
        let w3 = Coord2(73.0, 221.0);
        let w4 = Coord2(249.0, 136.0);

        match features_for_cubic_bezier(&w1, &w2, &w3, &w4, 0.01) {
            CurveFeatures::SingleInflectionPoint(t) => {
                assert!(is_inflection_point(&w1, &w2, &w3, &w4, t));
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn arch_1() {
        let w1 = Coord2(65.0, 146.0);
        let w2 = Coord2(95.0, 213.0);
        let w3 = Coord2(249.0, 218.0);
        let w4 = Coord2(256.0, 181.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn arch_2() {
        let w1 = Coord2(11.0, 143.0);
        let w2 = Coord2(156.0, 261.0);
        let w3 = Coord2(23.0, 278.0);
        let w4 = Coord2(24.0, 200.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Arch);
    }

    #[test]
    fn double_inflection_1() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(238.0, 232.0);
        let w3 = Coord2(108.0, 233.0);
        let w4 = Coord2(329.0, 129.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::DoubleInflectionPoint);
    }

    #[test]
    fn double_inflection_1_feature() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(238.0, 232.0);
        let w3 = Coord2(108.0, 233.0);
        let w4 = Coord2(329.0, 129.0);

        match features_for_cubic_bezier(&w1, &w2, &w3, &w4, 0.01) {
            CurveFeatures::DoubleInflectionPoint(t1, t2) => {
                assert!(is_inflection_point(&w1, &w2, &w3, &w4, t1));
                assert!(is_inflection_point(&w1, &w2, &w3, &w4, t2));
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn degenerate_single_point() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(56.0, 162.0);
        let w3 = Coord2(56.0, 162.0);
        let w4 = Coord2(56.0, 162.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Point);
    }

    #[test]
    fn degenerate_horizontal_line() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(64.0, 162.0);
        let w3 = Coord2(72.0, 162.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Linear);
    }

    #[test]
    fn degenerate_horizontal_line_overlapping_control_points() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(64.0, 162.0);
        let w3 = Coord2(64.0, 162.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Linear);
    }

    #[test]
    fn degenerate_line_only_two_control_points() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(56.0, 162.0);
        let w3 = Coord2(56.0, 162.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Linear);
    }

    #[test]
    fn degenerate_line_overlapping_all_control_points() {
        let w1 = Coord2(518.0, 765.0);
        let w2 = Coord2(518.0, 765.0);
        let w3 = Coord2(163.0, 611.0);
        let w4 = Coord2(163.0, 611.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::Linear);
    }

    #[test]
    fn features_for_degenerate_line_overlapping_all_control_points() {
        let w1 = Coord2(518.0, 765.0);
        let w2 = Coord2(518.0, 765.0);
        let w3 = Coord2(163.0, 611.0);
        let w4 = Coord2(163.0, 611.0);

        assert!(features_for_cubic_bezier(&w1, &w2, &w3, &w4, 0.00001) == CurveFeatures::Linear);
    }

    #[test]
    fn degenerate_cubic_curve() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(72.0, 172.0);
        let w3 = Coord2(72.0, 172.0);
        let w4 = Coord2(128.0, 162.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::DoubleInflectionPoint);
    }

    #[test]
    fn degenerate_cubic_curve_feature() {
        let w1 = Coord2(56.0, 162.0);
        let w2 = Coord2(72.0, 172.0);
        let w3 = Coord2(72.0, 172.0);
        let w4 = Coord2(128.0, 162.0);

        match features_for_cubic_bezier(&w1, &w2, &w3, &w4, 0.01) {
            CurveFeatures::DoubleInflectionPoint(t1, t2) => {
                assert!(is_inflection_point(&w1, &w2, &w3, &w4, t1));
                assert!(is_inflection_point(&w1, &w2, &w3, &w4, t2));
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn degenerate_needs_reversal() {
        let w1 = Coord2(55.0, 200.0);
        let w2 = Coord2(131.0, 200.0);
        let w3 = Coord2(290.0, 200.0);
        let w4 = Coord2(290.0, 95.0);

        assert!(characterize_cubic_bezier(&w1, &w2, &w3, &w4) == CurveCategory::SingleInflectionPoint);
    }
}
