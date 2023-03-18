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

pub use self::curve::*;
pub use self::section::*;
pub use self::basis::*;
pub use self::subdivide::*;
pub use self::derivative::*;
pub use self::tangent::*;
pub use self::normal::*;
pub use self::bounds::*;
pub use self::deform::*;
pub use self::fit::*;
pub use self::offset::*;
pub use self::offset_lms::*;
pub use self::offset_scaling::*;
pub use self::search::*;
pub use self::solve::*;
pub use self::overlaps::*;
pub use self::intersection::*;
pub use self::characteristics::*;
pub use self::length::*;
pub use self::walk::*;
pub use self::distort::*;
pub use self::nearest_point::*;

pub use super::geo::*;
