use crate::geo::*;

use smallvec::*;

///
/// Finds the points (as t-values) where a bezier curve's y coordinate is 0
///
pub fn find_roots<TPoint, const N: usize>(points: &[TPoint]) -> SmallVec<[f64; 4]>
where
    TPoint: Coordinate + Coordinate2D + Clone,
{
    todo!()
}
