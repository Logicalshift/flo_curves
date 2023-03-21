use super::sampled_contour::*;

///
/// A distance field represents a sampling of how far certain discrete points are from an edge in an image.
/// This is a signed distance field, where negative distances are used to indicate samples that are inside a shape.
///
/// This can be used to more precisely position points than is possible using a `SampledContour` alone.
///
/// Distances are typically in pixels, however some implementations (eg U8SampledDistanceField) may use arbitrary units.
/// (The units typically don't matter when searching for the edge as '0' is a common point)
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

///
/// Represents a signed distance field sampled with f32 values (>0 for values outside of the shape)
///
/// The vec here is represents the whole distance field from the top-left coordinate: it should be of size
/// ContourSize.0 * ContourSize.1
///
#[derive(Clone, PartialEq, Debug)]
pub struct F32SampledDistanceField(pub ContourSize, pub Vec<f32>);

///
/// Represents a signed distance field sampled with f64 values (>0 for values outside of the shape)
///
/// The vec here is represents the whole distance field from the top-left coordinate: it should be of size
/// ContourSize.0 * ContourSize.1
///
#[derive(Clone, PartialEq, Debug)]
pub struct F64SampledDistanceField(pub ContourSize, pub Vec<f64>);

///
/// Represents a signed distance field sampled with u8 values (>127 for values outside of the shape)
///
/// The vec here is represents the whole distance field from the top-left coordinate: it should be of size
/// ContourSize.0 * ContourSize.1
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct U8SampledDistanceField(pub ContourSize, pub Vec<u8>);

impl<'a> SampledSignedDistanceField for &'a F32SampledDistanceField {
    #[inline]
    fn size(self) -> ContourSize {
        self.0
    }

    #[inline]
    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        let width   = self.0.0;
        let pos     = pos.0 + (pos.1 * width);

        self.1[pos] as _
    }
}

impl<'a> SampledSignedDistanceField for &'a F64SampledDistanceField {
    #[inline]
    fn size(self) -> ContourSize {
        self.0
    }

    #[inline]
    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        let width   = self.0.0;
        let pos     = pos.0 + (pos.1 * width);

        self.1[pos]
    }
}

impl<'a> SampledSignedDistanceField for &'a U8SampledDistanceField {
    #[inline]
    fn size(self) -> ContourSize {
        self.0
    }

    #[inline]
    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        let width   = self.0.0;
        let pos     = pos.0 + (pos.1 * width);

        (self.1[pos] as f64) - 127.0
    }
}
