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
}
