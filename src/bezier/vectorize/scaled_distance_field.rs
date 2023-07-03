use super::distance_field::*;
use super::intercept_scan_edge_iterator::*;
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

    /// The scale factor of 
    scale_factor: f64,
}

impl<'a, TDistanceField> SampledSignedDistanceField for &'a ScaledDistanceField<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    type Contour = &'a ScaledDistanceField<TDistanceField>;

    #[inline]
    fn field_size(self) -> ContourSize {
        let ContourSize(width, height)  = self.distance_field.field_size();

        let width   = (width as f64) * self.scale_factor;
        let height  = (height as f64) * self.scale_factor;
        let width   = width.ceil();
        let height  = height.ceil();

        ContourSize(width as usize, height as usize)
    }

    fn distance_at_point(self, pos: super::ContourPosition) -> f64 {
        let ContourPosition(x, y) = pos;

        // Scale the x & y positions
        let x = x as f64;
        let y = y as f64;
        let x = x / self.scale_factor;
        let y = y / self.scale_factor;

        // We want to read the distance between the low and high positions
        let low_x   = x.floor();
        let low_y   = y.floor();
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

        distance
    }

    #[inline]
    fn as_contour(self) -> Self::Contour {
        self
    }
}

impl<'a, TDistanceField> SampledContour for &'a ScaledDistanceField<TDistanceField> 
where
    TDistanceField: SampledSignedDistanceField,
{
    type EdgeCellIterator = InterceptScanEdgeIterator<&'a ScaledDistanceField<TDistanceField>>;

    #[inline]
    fn contour_size(self) -> ContourSize {
        self.field_size()
    }

    #[inline]
    fn intercepts_on_line(self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        ScaledContour::from_contour(self.distance_field.as_contour(), self.scale_factor).intercepts_on_line(y)
    }

    #[inline]
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        InterceptScanEdgeIterator::new(self)
    }
}
