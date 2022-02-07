//!
//! # Manipulates multiple Bezier curves joined into a path
//!
//! The `BezierPath` trait provides a way to represent a bezier path. `flo_curves` considers a path to be a single
//! closed loop, unlike many libraries which allow for open paths and paths with subpaths. Instead, a path with
//! multiple subpaths is represented as a collection - ie `Vec<impl Path>`. This reduces the number of edge cases
//! the library has to deal with.
//!
//! The `path_add()`, `path_sub()` and `path_intersect()` functions can be used to perform path arithmetic: combining
//! multiple paths into a single result. The `GraphPath` type is used to implement these functions: it can represent
//! paths where points can have more than one following edge attached to them and provides functions for implementing
//! similar operations.
//!
//! `BezierPathBuilder` provides a way to quickly build paths from any type implementing the factory trait without
//! needing to generate all of the primitives manually.
//!

pub mod algorithms;
mod arithmetic;
mod bounds;
mod graph_path;
mod intersection;
mod is_clockwise;
mod path;
mod path_builder;
mod point;
mod ray;
mod to_curves;

pub use self::arithmetic::*;
pub use self::bounds::*;
pub use self::graph_path::*;
pub use self::intersection::*;
pub use self::is_clockwise::*;
pub use self::path::*;
pub use self::path_builder::*;
pub use self::point::*;
pub use self::to_curves::*;
