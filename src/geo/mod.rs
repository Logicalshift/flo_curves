//!
//! # Traits for basic geometric definitions
//! 
//! This provides some basic geometric definitions. The `Geo` trait can be implemented by any type that has
//! a particular type of coordinate - for example, implementations of `BezierCurve` need to implement `Geo`
//! in order to describe what type they use for coordinates.
//! 
//! `BoundingBox` provides a way to describe axis-aligned bounding boxes. It too is a trait, making it
//! possible to request bounding boxes in types other than the default `Bounds` type supplied by the
//! library.
//!

mod geo;
mod sweep;
mod has_bounds;
mod coordinate;
mod coord1;
mod coord2;
mod coord3;
mod coordinate_ext;
mod bounding_box;

pub use self::geo::*;
pub use self::sweep::*;
pub use self::coord1::*;
pub use self::coord2::*;
pub use self::coord3::*;
pub use self::has_bounds::*;
pub use self::coordinate::*;
pub use self::bounding_box::*;
pub use self::coordinate_ext::*;

