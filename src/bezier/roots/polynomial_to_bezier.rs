use crate::geo::*;

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
    
    todo!()
}