use super::bounding_box::*;
use super::geo::*;

///
/// Trait implemented by types that have a bounding box associated with them
///
pub trait HasBoundingBox: Geo {
    ///
    /// Returns the bounding box that encloses this item
    ///
    fn get_bounding_box<Bounds: BoundingBox<Point = Self::Point>>(&self) -> Bounds;
}
