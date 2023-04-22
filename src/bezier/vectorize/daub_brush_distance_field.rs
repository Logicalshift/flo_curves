use super::distance_field::*;
use super::sampled_contour::*;

///
/// Describes a shape as a distance field made up by 'daubing' discrete brush shapes over a canvas
///
/// Each brush shape - 'daub' - is itself a distance field, and can be placed at any integer position on the canvas (to 
/// position at subpixels, they will need to be separately resampled). By combining these shapes, a distance field 
/// describing a brush stroke can be constructed, which can be converted into a vector using 
/// `trace_contours_from_distance_field()`
///
/// Note that for just creating a thick line, `offset_lms_sampling()` is much faster but it can only offset along a
/// fixed distance along the normal of the curve, so it doesn't produce good results if the offset is changing across
/// the span of the curve or if the curve is not particularly smooth. `offset_scaling()` is also available as an even
/// faster alternative for the simple cases, but is even more limited in terms of what it can produce.
///
/// This provides the most general purpose approach to generating vectors from brush strokes or other patterns.
///
pub struct DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    /// The size of this distance field, sufficient to contain all of the 'daubs'
    size: ContourSize,

    /// The 'daubs' that make up the brush stroke, and where they appear on the canvas. This is stored sorted by Y position
    /// to allow scanning downwards to find which 'daubs' influence which points
    daubs: Vec<(TDaub, ContourPosition)>
}

///
/// Iterates over the edges in a daub brush distance field
///
pub struct DaubBrushContourIterator<'a, TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    distance_field: &'a DaubBrushDistanceField<TDaub>,
}

impl<'a, TDaub> SampledContour for &'a DaubBrushDistanceField<TDaub> 
where
    TDaub: SampledSignedDistanceField,
{
    type EdgeCellIterator = DaubBrushContourIterator<'a, TDaub>;

    #[inline]
    fn size(self) -> ContourSize {
        self.size
    }

    fn point_is_inside(self, pos: ContourPosition) -> bool {
        todo!()
    }

    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        todo!()
    }
}

impl<'a, TDaub> SampledSignedDistanceField for &'a DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    type Contour = &'a DaubBrushDistanceField<TDaub>;

    #[inline]
    fn size(self) -> ContourSize {
        self.size
    }

    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        todo!()
    }

    fn as_contour(self) -> Self::Contour {
        self
    }
}

impl<'a, TDaub> Iterator for DaubBrushContourIterator<'a, TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    type Item = (ContourPosition, ContourCell);

    fn next(&mut self) -> Option<(ContourPosition, ContourCell)> {
        todo!()
    }
}

