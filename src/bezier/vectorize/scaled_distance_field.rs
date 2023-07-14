use super::column_sampled_contour::*;
use super::distance_field::*;
use super::sampled_contour::*;
use super::scaled_contour::*;

use smallvec::*;

use std::ops::{Range};

///
/// A distance field that uses bilinear filtering in order to adjust its size by a scale factor
///
pub struct ScaledDistanceField<TDistanceField> {
    /// The distance field that is being scaled
    distance_field: TDistanceField,

    /// The scale factor to apply to the source distance field
    scale_factor: f64,

    /// X offset to apply to the result
    offset_x: f64,

    /// Y offset to apply to the result
    offset_y: f64,

    /// The size of 
    size: ContourSize,
}

impl<TDistanceField> ScaledDistanceField<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    ///
    /// Creates scaled version of another distance field
    ///
    #[inline]
    pub fn from_distance_field(distance_field: TDistanceField, scale_factor: f64, offset: (f64, f64)) -> Self {
        // Multiply the original size by the scale factor to get the new size
        let ContourSize(width, height)  = distance_field.field_size();

        let width   = (width as f64) * scale_factor + offset.0;
        let height  = (height as f64) * scale_factor + offset.1;
        let width   = width.ceil();
        let height  = height.ceil();

        let size = ContourSize(width as _, height as _);

        // The offset is added to the position to allow for aligning the distance field to non-integer grids (eg, when this is used as a brush)
        let (offset_x, offset_y) = offset;

        ScaledDistanceField { distance_field, scale_factor, size, offset_x, offset_y }
    }
}

impl<TDistanceField> SampledSignedDistanceField for ScaledDistanceField<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    type Contour = ScaledDistanceField<TDistanceField>;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.size
    }

    fn distance_at_point(&self, pos: super::ContourPosition) -> f64 {
        let ContourPosition(x, y) = pos;

        // Scale the x & y positions
        let x = x as f64 - self.offset_x;
        let y = y as f64 - self.offset_y;
        let x = x / self.scale_factor;
        let y = y / self.scale_factor;

        let low_x   = x.floor();
        let low_y   = y.floor();

        if self.scale_factor >= 1.0 {
            // Read the position without interpolating/resampling
            self.distance_field.distance_at_point(ContourPosition(low_x as _, low_y as _))
        } else {
            // We want to read the distance between the low and high positions
            let high_x  = low_x + 1.0;
            let high_y  = low_y + 1.0;

            // Read the distances at the 4 corners
            let distances = [
                [self.distance_field.distance_at_point(ContourPosition(low_x as _, low_y as _)), self.distance_field.distance_at_point(ContourPosition(low_x as _, high_y as _))],
                [self.distance_field.distance_at_point(ContourPosition(high_x as _, low_y as _)), self.distance_field.distance_at_point(ContourPosition(high_x as _, high_y as _))]
            ];

            // Interpolate the distances
            let distance_x1 = ((high_x - x)/(high_x - low_x)) * distances[0][0] + ((x - low_x)/(high_x - low_x)) * distances[1][0];
            let distance_x2 = ((high_x - x)/(high_x - low_x)) * distances[0][1] + ((x - low_x)/(high_x - low_x)) * distances[1][1];
            let distance    = ((high_y - y)/(high_y - low_y)) * distance_x1 + ((y - low_y)/(high_y - low_y)) * distance_x2;

            distance * self.scale_factor
        }
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        self
    }
}

impl<TDistanceField> SampledContour for ScaledDistanceField<TDistanceField> 
where
    TDistanceField: SampledSignedDistanceField,
{
    #[inline]
    fn contour_size(&self) -> ContourSize {
        self.field_size()
    }

    #[inline]
    fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        ScaledContour::from_contour(self.distance_field.as_contour(), self.scale_factor, (self.offset_x, self.offset_y)).intercepts_on_line(y)
    }
}

impl<TDistanceField> ColumnSampledContour for ScaledDistanceField<TDistanceField> 
where
    TDistanceField:             SampledSignedDistanceField,
    TDistanceField::Contour:    ColumnSampledContour,
{
    #[inline]
    fn intercepts_on_column(&self, x: f64) -> SmallVec<[Range<f64>; 4]> {
        ScaledContour::from_contour(self.distance_field.as_contour(), self.scale_factor, (self.offset_x, self.offset_y)).intercepts_on_column(x)
    }
}
