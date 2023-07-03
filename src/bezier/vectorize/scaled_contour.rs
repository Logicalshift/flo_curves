use super::intercept_scan_edge_iterator::*;
use super::sampled_contour::*;

use smallvec::*;

use std::ops::{Range};

///
/// A sampled contour whose result is scaled by the specified scale factor. The results of scaling a contour depends on how its
/// implementation of `intercepts_on_line` works: for many contour types (such as path contours), this will produce accurate
/// results on non-integer positions, so will scale smoothly.
///
pub struct ScaledContour<TContour> {
    /// The contour whose contents will be scaled
    contour: TContour,

    /// The scale factor
    scale_factor: f64,
}

impl<TContour> ScaledContour<TContour>
where
    TContour: SampledContour,
{
    ///
    /// Creates scaled version of another contour
    ///
    #[inline]
    pub fn from_contour(contour: TContour, scale_factor: f64) -> Self {
        ScaledContour { contour, scale_factor }
    }
}

impl<'a, TContour> SampledContour for &'a ScaledContour<TContour>
where
    TContour: SampledContour,
{
    type EdgeCellIterator = InterceptScanEdgeIterator<&'a ScaledContour<TContour>>;

    #[inline]
    fn contour_size(self) -> ContourSize {
        let ContourSize(width, height)  = self.contour.contour_size();

        let width   = (width as f64) * self.scale_factor;
        let height  = (height as f64) * self.scale_factor;
        let width   = width.ceil();
        let height  = height.ceil();

        ContourSize(width as usize, height as usize)
    }

    #[inline]
    fn intercepts_on_line(self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        let y = y * self.scale_factor;

        self.contour.intercepts_on_line(y)
            .into_iter()
            .map(|range| {
                (range.start * self.scale_factor)..(range.end * self.scale_factor)
            })
            .collect()
    }

    #[inline]
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        InterceptScanEdgeIterator::new(self)
    }
}
