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

        let width   = (width as f64) / self.scale_factor;
        let height  = (height as f64) / self.scale_factor;
        let width   = width.ceil();
        let height  = height.ceil();

        ContourSize(width as usize, height as usize)
    }

    fn distance_at_point(self, pos: super::ContourPosition) -> f64 {
        let ContourPosition(x, y) = pos;

        let x = x as f64;
        let y = y as f64;
        let x = x * self.scale_factor;
        let y = y * self.scale_factor;

        todo!()
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
