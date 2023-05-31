use crate::geo::*;

use std::convert::{TryInto};

///
/// Generates the control polygon corresponding to a polynomial
///
/// The polynomial has the form `c[0] + c[1]*x + c[2]*x^2 + c[3]*x^3 ...` where `c` is the list of coefficints
///
pub fn polynomial_to_bezier<TPoint, const N: usize>(coefficients: [f64; N]) -> [TPoint; N] 
where
    TPoint: Coordinate + Coordinate2D,
{
    // See "A bezier curve-based root-finder", Philip J Schneider, Graphics Gems
    let mut coefficients = coefficients;

    for j in 1..=N {
        let c       = 1.0 / (N as f64 + 1.0 - j as f64);
        let mut d   = 1.0;
        let mut e   = c;

        for i in (j..=N).into_iter().rev() {
            coefficients[i] = d * coefficients[i] + e * coefficients[i-1];
            d = d - c;
            e = e + c;
        }
    }

    // Convert to points (range is 0..1)
    let coefficients = coefficients.iter().enumerate().map(|(x, y)| {
        let x = (x as f64) / (N as f64);
        TPoint::from_components(&[x, *y])
    }).collect::<Vec<_>>().try_into();

    if let Ok(coefficients) = coefficients {
        coefficients
    } else {
        unreachable!()
    }
}
