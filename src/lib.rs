//!
//! ```toml
//! flo_curves = "0.4"
//! ```
//! 
//! flo_curves
//! ==========
//!
//! `flo_curves` is a library of routines for inspecting and manipulating curves, with a focus on cubic Bézier curves. In 
//! this library, you'll find routines for computing points on curves, performing collision detection between curves and 
//! lines or other curves, all the way up to routines for combining paths made up of multiple curves.
//! 
//! Anyone doing any work with Bézier curves will likely find something in this library that is of use, but its range of
//! functions makes it particularly useful for collision detection or performing path arithmetic.
//! 
//! A set of curve and coordinate types are provided by the library, as well as a set of traits that can be implemented
//! on any types with suitable properties. Implementing these traits makes it possible to add the extra features of this
//! library to any existing code that has its own way of representing coordinates, curves or paths.
//! 
//! `flo_curves` was built as a support library for `flowbetween`, an animation tool I'm working on.
//!

#![warn(bare_trait_objects)]

#[macro_use] mod test_assert;
mod consts;
pub mod bezier;
pub mod line;
pub mod arc;
pub mod debug;

pub mod geo;
pub use self::geo::*;

pub use self::bezier::BezierCurveFactory;
pub use self::bezier::BezierCurve;
pub use self::line::Line;
