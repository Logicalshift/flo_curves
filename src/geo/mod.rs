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

mod bounding_box;
mod coordinate;
mod coordinate_ext;
mod geo;
mod has_bounds;
mod sweep;

pub use self::bounding_box::*;
pub use self::coordinate::*;
pub use self::coordinate_ext::*;
pub use self::geo::*;
pub use self::has_bounds::*;
pub use self::sweep::*;
