use crate::geo::*;
use super::basis::*;

use std::convert::{TryInto};

///
/// Subdivides a bezier curve with any number of weights at a particular point. Returns the weights for the
/// two curves on either side of the subdivision point.
///
pub (crate) fn subdivide_n<TPoint, const N: usize>(t: f64, points: [TPoint; N]) -> ([TPoint; N], [TPoint; N])
where
    TPoint: Coordinate,
{
    // Want to store 1+2+3+...+N weights in total
    let num_weights = (N * (N+1))/2;

    // Calculate the weights at each layer using the de Casteljau algorithm
    // TODO: num_weights is a constant so we could just use a slice instead of a vec here (Rust doesn't currently allow it, though)
    let mut weights = Vec::with_capacity(num_weights);

    // Copy the points into the initial set of weights
    weights.extend(points);

    // Use de Casteljau algorithm to generate all of the weights for the curve
    let mut last_pos = 0;

    for depth in (1..N).into_iter().rev() {
        // Apply de Casteljau to the last set of weights (producing the weights for the tangents)
        for p in 0..depth {
            let wn = weights[last_pos+p]*(1.0-t) + weights[last_pos+p+1]*t;
            weights.push(wn);
        }

        // Process the next set of weights to generate the full set of derivatives for the curve
        last_pos += depth+1;
    }

    // First set of points are the first weight from each level, second set are the last weight from each level
    // TODO: would be nice to avoid creating the vecs here (think this is a little more possible than for the main weights list with the current version of Rust)
    let mut first_weights   = Vec::with_capacity(N);
    let mut second_weights  = Vec::with_capacity(N);

    let mut last_pos = 0;
    for depth in (0..N).into_iter().rev() {
        first_weights.push(weights[last_pos]);
        second_weights.push(weights[last_pos+depth]);

        last_pos += depth + 1;
    }

    // Second set of weights will be reversed at this point, so we need to switch them around
    if let (Ok(first_weights), Ok(second_weights)) = (first_weights.try_into(), second_weights.try_into()) {
        let mut second_weights: [TPoint; N] = second_weights;
        second_weights.reverse();

        (first_weights, second_weights)
    } else {
        unreachable!()
    }
}

///
/// Subdivides a cubic bezier curve at a particular point, returning the weights of
/// the two component curves
/// 
pub fn subdivide4<Point: Coordinate>(t: f64, w1: Point, w2: Point, w3: Point, w4: Point) -> 
    ((Point, Point, Point, Point), (Point, Point, Point, Point)) {
    // Weights (from de casteljau)
    let wn1 = w1*(1.0-t) + w2*t;
    let wn2 = w2*(1.0-t) + w3*t;
    let wn3 = w3*(1.0-t) + w4*t;

    // Further refine the weights
    let wnn1 = wn1*(1.0-t) + wn2*t;
    let wnn2 = wn2*(1.0-t) + wn3*t;

    // Get the point at which the two curves join
    let p = de_casteljau2(t, wnn1, wnn2);

    // Curves are built from the weight calculations and the final points
    ((w1, wn1, wnn1, p), (p, wnn2, wn3, w4))
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn approx_equal(a: f64, b: f64) -> bool {
        f64::floor(f64::abs(a-b)*10000.0) == 0.0
    }

    #[test]
    fn subdivide_n_1() {
        // Initial curve
        let (w1, w2, w3, w4) = (1.0, 2.0, 3.0, 4.0);

        // Subdivide at 33%, creating two curves
        let ([wa1, wa2, wa3, wa4], [wb1, wb2, wb3, wb4])            = subdivide_n(0.33, [w1, w2, w3, w4]);
        let ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4))    = subdivide4(0.33, w1, w2, w3, w4);

        debug_assert!(approx_equal(wa1, waa1), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));
        debug_assert!(approx_equal(wa2, waa2), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));
        debug_assert!(approx_equal(wa3, waa3), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));
        debug_assert!(approx_equal(wa4, waa4), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));

        debug_assert!(approx_equal(wb1, wbb1), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));
        debug_assert!(approx_equal(wb2, wbb2), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));
        debug_assert!(approx_equal(wb3, wbb3), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));
        debug_assert!(approx_equal(wb4, wbb4), "{:?} != {:?}", ((wa1, wa2, wa3, wa4), (wb1, wb2, wb3, wb4)), ((waa1, waa2, waa3, waa4), (wbb1, wbb2, wbb3, wbb4)));

        // Check that the original curve corresponds to the basis function for wa
        for x in 0..100 {
            let t = (x as f64)/100.0;

            let original    = basis(t*0.33, w1, w2, w3, w4);
            let subdivision = basis(t, wa1, wa2, wa3, wa4);

            assert!(approx_equal(original, subdivision));
        }
    }

}