use super::sampled_contour::*;

use smallvec::*;

use std::ops::{Range};

///
/// A `SampledContour` that can return the intercepts along columns as well as lines
///
/// This can be used as an alternative to a distance field to produce a more accurate tracing of a shape.
///
pub trait ColumnSampledContour : SampledContour {
    ///
    /// Given an x coordinate, returns ranges indicating the filled pixels on that column
    ///
    /// The ranges must be returned in ascending order and must not overlap
    ///
    fn intercepts_on_column(&self, x: f64) -> SmallVec<[Range<f64>; 4]>;

    ///
    /// Retrieves the intercepts on a column, rounded to pixel positions
    ///
    #[inline]
    fn rounded_intercepts_on_column(&self, x: f64) -> SmallVec<[Range<usize>; 4]> {
        let intercepts = self.intercepts_on_column(x)
            .into_iter()
            .map(|intercept| {
                let min_y_ceil = intercept.start.ceil();
                let max_y_ceil = intercept.end.ceil();

                let min_y = min_y_ceil as usize;
                let max_y = max_y_ceil as usize;

                min_y..max_y
            })
            .filter(|intercept| intercept.start != intercept.end)
            .collect::<SmallVec<_>>();

        if intercepts.len() <= 1 {
            intercepts
        } else {
            merge_overlapping_intercepts(intercepts)
        }
    }
}

impl<'a, T> ColumnSampledContour for &'a T
where
    T: ColumnSampledContour,
{
    #[inline] fn intercepts_on_column(&self, x: f64) -> SmallVec<[Range<f64>; 4]> { (*self).intercepts_on_column(x) }
}
