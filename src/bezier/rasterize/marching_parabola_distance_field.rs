use crate::bezier::vectorize::*;

use smallvec::*;

use std::ops::{Range};

///
/// Distance field that's computed by following the marching parabolas algorithm
///
pub struct MarchingParabolaDistanceField {
    /// Width of the distance field in pixels
    width: usize,

    /// Height of the distance field in pixels
    height: usize,

    /// width * height pixels indicating the squared distance
    squared_distance_field: Vec<f64>,
}

impl MarchingParabolaDistanceField {
    ///
    /// Computes a distance field using the marching parabolas algorithm given functions that calculate the X and Y intercepts against a
    /// shape.
    ///
    /// Error can be up to 1 pixel for this function but is usually much less as we can compute the parabolas to a higher precision than
    /// one pixel.
    ///
    pub fn from_intercepts() -> Self {
        todo!()
    }

    ///
    /// Computes a distance field using the marching parabolas algorithm given an iterator that indicates whether or not a pixel is inside
    /// or outside of the shape.
    ///
    /// Calculated distances may be up to 1 pixel out compared to the source shape, but this can be used as a way to vectorize and scale
    /// silhouettes.
    ///
    pub fn from_bitfield() -> Self {
        todo!()
    }
}

impl SampledSignedDistanceField for MarchingParabolaDistanceField {
    type Contour = Self;

    #[inline]
    fn field_size(&self) -> ContourSize {
        ContourSize(self.width, self.height)
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        let distance_squared = self.squared_distance_field[pos.0 + pos.1 * self.width];

        distance_squared.sqrt()
    }

    fn as_contour<'b>(&'b self) -> &'b Self::Contour {
        self
    }
}

impl SampledContour for MarchingParabolaDistanceField {
    #[inline]
    fn contour_size(&self) -> ContourSize {
        ContourSize(self.width, self.height)
    }

    ///
    /// Given a y coordinate returns ranges indicating the filled pixels on that line
    ///
    /// The ranges must be provided in ascending order, and must also not overlap.
    ///
    fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        todo!()
    }
}