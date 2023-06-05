use crate::geo::*;

use std::convert::{TryInto};

///
/// Generates the control polygon corresponding to a polynomial
///
/// The polynomial has the form `c[0] + c[1]*x + c[2]*x^2 + c[3]*x^3 ...` where `c` is the list of coefficints
///
#[allow(dead_code)] // Used for testing, will make a public function in v0.8
pub fn polynomial_to_bezier<TPoint, const N: usize>(coefficients: [f64; N]) -> [TPoint; N] 
where
    TPoint: Coordinate + Coordinate2D,
{
    // See "A bezier curve-based root-finder", Philip J Schneider, Graphics Gems
    let mut coefficients = coefficients;

    for j in 1..N {
        let c       = 1.0 / (N as f64 - j as f64);
        let mut d   = 1.0;
        let mut e   = c;

        for i in (j..N).into_iter().rev() {
            coefficients[i] = d * coefficients[i] + e * coefficients[i-1];
            d = d - c;
            e = e + c;
        }
    }

    // Convert to points (range is 0..1)
    let coefficients = coefficients.iter().enumerate().map(|(x, y)| {
        let x = (x as f64) / ((N-1) as f64);
        TPoint::from_components(&[x, *y])
    }).collect::<Vec<_>>().try_into();

    if let Ok(coefficients) = coefficients {
        coefficients
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bezier::*;

    #[test]
    fn simple_polynomial() {
        // (x-0.5)(x-0.4)(x-0.3)(x-0.2)(x-0.1)
        //  == -0.0012 + 0.0274x - 0.225x^2  + 0.85x^3 - 1.5x^4 + x^5
        let bezier  = polynomial_to_bezier::<Coord2, 6>([-0.0012, 0.0274, -0.225, 0.85, -1.5, 1.0]);
        let point5  = de_casteljau_n(0.5, bezier.clone().into());
        let point4  = de_casteljau_n(0.4, bezier.clone().into());
        let point3  = de_casteljau_n(0.3, bezier.clone().into());
        let point2  = de_casteljau_n(0.2, bezier.clone().into());
        let point1  = de_casteljau_n(0.1, bezier.clone().into());

        assert!(point1.y().abs() < 0.1, "{:?} {:?} {:?} {:?} {:?}", point1, point2, point3, point4, point5);
        assert!(point2.y().abs() < 0.1, "{:?} {:?} {:?} {:?} {:?}", point1, point2, point3, point4, point5);
        assert!(point3.y().abs() < 0.1, "{:?} {:?} {:?} {:?} {:?}", point1, point2, point3, point4, point5);
        assert!(point4.y().abs() < 0.1, "{:?} {:?} {:?} {:?} {:?}", point1, point2, point3, point4, point5);
        assert!(point5.y().abs() < 0.1, "{:?} {:?} {:?} {:?} {:?}", point1, point2, point3, point4, point5);
    }
}
