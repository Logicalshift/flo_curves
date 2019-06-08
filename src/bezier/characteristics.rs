use super::super::geo::*;

///
/// Possible types of a two-dimensional cubic bezier curve
///
pub enum CurveType {
    /// A simple curve that does not change direction or self-intersect
    Arch,

    /// A curve that changes direction once
    SingleInflectionPoint,

    /// A curve that changes direction twice
    DoubleInflectionPoint,

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
/// curve but only a single free point (w4)
///
fn canonical_curve_transform<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point) -> (f64, f64, f64, f64, f64, f64) {
    // Fetch the coordinates
    let (x0, y0) = (w1.x(), w1.y());
    let (x1, y1) = (w2.x(), w2.y());
    let (x2, y2) = (w3.x(), w3.y());

    // Transform is:
    // 
    // [ a, b, c ]   [ x ]
    // [ d, e, f ] . [ y ]
    // [ 0, 0, 1 ]   [ 1 ]
    // 
    // This will move w1 to 0,0, w2 to 0, 1 and w3 to 1, 1, which will form our canonical curve that we use for the classification algorithm
    let a = (-(y0-y1)) / ((y2-y1)*(x0-x1)-(x2-x1)*(y0-y1));
    let b = (-(x0-x1)) / ((x2-x1)*(y0-y1)-(y2-y1)*(x0-x1));
    let c = -a*x0 - b*y0;
    let d = (y1-y2) / ((x0-x1)*(y2-y1) - (x2-x1)*(y0-y1));
    let e = (x1-x2) / ((y0-y1)*(x2-x1) - (y2-y1)*(x0-x1));
    let f = -d*x0 - e*y0;

    (a, b, c, d, e, f)
}

///
/// Converts a set of points to a 'canonical' curve
/// 
/// This is the curve such that w1 = (0.0), w2 = (1, 0) and w3 = (1, 1), if such a curve exists. The return value is the point w4
/// for this curve.
///
fn to_canonical_curve<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point, w4: &Point) -> Point {
    // Retrieve the affine transform for the curve
    let (a, b, c, d, e, f) = canonical_curve_transform(w1, w2, w3);

    // Calculate the free point w4 based on the transform
    let x3  = w4.x();
    let y3  = w4.y();

    let x   = a*x3 + b*y3 + c;
    let y   = d*x3 + e*y3 + f;

    Point::from_components(&[x, y])
}

/*
///
/// Determines the characteristic coefficients A, B, C and Delta for an arbitrary bezier curve
///
fn characteristic_coefficients<Point: Coordinate+Coordinate2D>(w1: &Point, w2: &Point, w3: &Point, w4: &Point) -> (f64, f64, f64, f64) {
    let free_point  = to_canonical_curve(w1, w2, w3, w4);

    let a           = 9.0*(free_point.x() + free_point.y() - 3.0);
    let b           = -9.0*(free_point.x() - 3.0);
    let c           = -9.0;
    let delta       = 0.0;

    (a, b, c, delta)
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn canonical_curve_coeffs_are_valid_1() {
        let w1                  = Coord2(1.0, 1.0);
        let w2                  = Coord2(2.0, 3.0);
        let w3                  = Coord2(5.0, 2.0);

        let (a, b, c, d, e, f)  = canonical_curve_transform(&w1, &w2, &w3);

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
}
