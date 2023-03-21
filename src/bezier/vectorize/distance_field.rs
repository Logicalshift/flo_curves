use super::sampled_contour::*;

///
/// A distance field represents a sampling of how far certain discrete points are from an edge in an image.
/// This is a signed distance field, where negative distances are used to indicate samples that are inside a shape.
///
/// This can be used to more precisely position points than is possible using a `SampledContour` alone.
///
/// Implement this trait on a reference to a storage type rather than the type itself
///
pub trait SignedDistanceField {
    ///
    /// The size of this distance field
    ///
    fn size(self) -> ContourSize;

    ///
    /// Returns the distance to the nearest edge of the specified point (a negative value if the point is inside the shape)
    ///
    fn distance_at_point(self, pos: ContourPosition) -> f64;
}
