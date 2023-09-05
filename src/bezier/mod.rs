//!
//! # Routines for describing, querying and manipulating Bezier curves
//!
//! ```
//! # use flo_curves::*;
//! # use flo_curves::bezier::*;
//! # 
//! let curve           = Curve::from_points(Coord2(1.0, 2.0), (Coord2(2.0, 0.0), Coord2(3.0, 5.0)), Coord2(4.0, 2.0));
//!
//! let mid_point       = curve.point_at_pos(0.5);
//! let all_points      = walk_curve_evenly(&curve, 1.0, 0.01).map(|section| section.point_at_pos(0.5)).collect::<Vec<_>>();
//! let fitted_curve    = fit_curve::<Curve<Coord2>>(&all_points, 0.1);
//! let intersections   = curve_intersects_ray(&curve, &(Coord2(1.0, 1.0), Coord2(2.0, 2.0)));
//! let offset_curve    = offset(&curve, 2.0, 2.0);
//! ```
//!
//! Anything that implements the `BezierCurve` trait can be manipulated by the functions in this crate. The `Curve` type
//! is provided as a basic implementation for defining bezier curves, but the trait can be defined on any type that
//! represents a bezier curve.
//!
//! The `BezierCurveFactory` trait extends the `BezierCurve` trait for use with functions that can build/return new curves.
//!
//! For routines that deal with paths made up of bezier curves, see the `path` namespace.
//!

mod curve;
mod section;
mod basis;
mod subdivide;
mod derivative;
mod tangent;
mod normal;
mod bounds;
mod deform;
mod fit;
mod offset;
mod offset_lms;
mod offset_scaling;
mod offset_subdivision_lms;
mod search;
mod solve;
mod overlaps;
mod intersection;
mod characteristics;
mod length;
mod walk;
mod distort;
mod nearest_point;

pub mod path;
pub mod vectorize;
pub mod rasterize;
pub mod roots;

pub use curve::*;
pub use section::*;
pub use basis::*;
pub use subdivide::*;
pub use derivative::*;
pub use tangent::*;
pub use normal::*;
pub use bounds::*;
pub use deform::*;
pub use fit::*;
pub use offset::*;
pub use offset_lms::*;
pub use offset_scaling::*;
pub use offset_subdivision_lms::*;
pub use search::*;
pub use solve::*;
pub use overlaps::*;
pub use intersection::*;
pub use characteristics::*;
pub use length::*;
pub use walk::*;
pub use distort::*;
pub use nearest_point::*;

pub use super::geo::*;
