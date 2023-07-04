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

    /// X offset to apply to the result
    offset_x: f64,

    /// Y offset to apply to the result
    offset_y: f64,

    /// The size of 
    size: ContourSize,
}

impl<TContour> ScaledContour<TContour>
where
    TContour: SampledContour,
{
    ///
    /// Creates scaled version of another contour
    ///
    #[inline]
    pub fn from_contour(contour: TContour, scale_factor: f64, offset: (f64, f64)) -> Self {
        // Multiply the original size by the scale factor to get the new size
        let ContourSize(width, height)  = contour.contour_size();

        let width   = (width as f64) * scale_factor + offset.0;
        let height  = (height as f64) * scale_factor + offset.1;
        let width   = width.ceil();
        let height  = height.ceil();

        let size = ContourSize(width as _, height as _);

        // The offset is added to the position to allow for aligning the distance field to non-integer grids (eg, when this is used as a brush)
        let (offset_x, offset_y) = offset;

        ScaledContour { contour, scale_factor, size, offset_x, offset_y }
    }
}

impl<'a, TContour> SampledContour for &'a ScaledContour<TContour>
where
    TContour: SampledContour,
{
    type EdgeCellIterator = InterceptScanEdgeIterator<&'a ScaledContour<TContour>>;

    #[inline]
    fn contour_size(self) -> ContourSize {
        self.size
    }

    #[inline]
    fn intercepts_on_line(self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        let y = (y - self.offset_y) / self.scale_factor;

        self.contour.intercepts_on_line(y)
            .into_iter()
            .map(|range| {
                (range.start * self.scale_factor + self.offset_x)..(range.end * self.scale_factor + self.offset_x)
            })
            .collect()
    }

    #[inline]
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        InterceptScanEdgeIterator::new(self)
    }
}
