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

// Breaks the exported API if auto-fixed (can remove these with a version bump)
#![allow(clippy::ptr_arg)]
#![allow(clippy::from_over_into)]

// Breaks stylistic choices/algorithm readability
#![allow(clippy::redundant_field_names)]                // Used for consistency when initialising some types
#![allow(clippy::collapsible_if)]                       // Often used to clarify algorithm structure (rewrites need to be at least as clear)
#![allow(clippy::collapsible_else_if)]                  // Often used to clarify algorithm structure
#![allow(clippy::module_inception)]                     // The 'line' module has a 'Line' type in it, for example. Makes sense the file is called 'line'...

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
