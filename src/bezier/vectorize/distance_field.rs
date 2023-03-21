use super::sampled_contour::*;

///
/// A distance field represents a sampling of how far certain discrete points are from an edge in an image.
/// This is a signed distance field, where negative distances are used to indicate samples that are inside a shape.
///
/// This can be used to more precisely position points than is possible using a `SampledContour` alone.
///
/// Implement this trait on a reference to a storage type rather than the type itself
///
pub trait SampledSignedDistanceField : Copy {
    ///
    /// The size of this distance field
    ///
    fn size(self) -> ContourSize;

    ///
    /// Returns the distance to the nearest edge of the specified point (a negative value if the point is inside the shape)
    ///
    fn distance_at_point(self, pos: ContourPosition) -> f64;
}

///
/// Converts a signed distance field into a sampled contour
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ContourFromDistanceField<TDistanceField>(pub TDistanceField)
where
    TDistanceField: SampledSignedDistanceField;

impl<TDistanceField> SampledContour for ContourFromDistanceField<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    type EdgeCellIterator = SimpleEdgeCellIterator<Self>;

    #[inline]
    fn size(self) -> ContourSize {
        self.0.size()
    }

    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        self.0.distance_at_point(pos) <= 0.0
    }

    #[inline]
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        SimpleEdgeCellIterator::from_contour(self)
    }
}
