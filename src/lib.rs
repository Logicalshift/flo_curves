//! 
//! # flo_curves
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
//! ## Examples
//! 
//! Creating a curve:
//! 
//! ```
//! use flo_curves::*;
//! use flo_curves::bezier;
//! 
//! let curve = bezier::Curve::from_points(Coord2(1.0, 2.0), (Coord2(2.0, 0.0), Coord2(3.0, 5.0)), Coord2(4.0, 2.0));
//! ```
//! 
//! Finding a point on a curve:
//! 
//! ```
//! # use flo_curves::*;
//! # use flo_curves::bezier;
//! # 
//! # let curve = bezier::Curve::from_points(Coord2(1.0, 2.0), (Coord2(2.0, 0.0), Coord2(3.0, 5.0)), Coord2(4.0, 2.0));
//! # 
//! let pos = curve.point_at_pos(0.5);
//! ```
//! 
//! Intersections:
//! 
//! ```
//! use flo_curves::*;
//! use flo_curves::bezier;
//! # 
//! # let curve1 = bezier::Curve::from_points(Coord2(1.0, 2.0), (Coord2(2.0, 0.0), Coord2(3.0, 5.0)), Coord2(4.0, 2.0));
//! # let curve2 = bezier::Curve::from_points(Coord2(2.0, 1.0), (Coord2(0.0, 2.0), Coord2(5.0, 3.0)), Coord2(2.0, 4.0));
//! 
//! for (t1, t2) in bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01) {
//!     let pos = curve1.point_at_pos(t1);
//!     println!("Intersection, curve1 t: {}, curve2 t: {}, position: {}, {}", t1, t2, pos.x(), pos.y());
//! }
//! ```
//! 
//! Creating a path:
//! 
//! ```
//! use flo_curves::*;
//! use flo_curves::bezier;
//! use flo_curves::bezier::path::*;
//! 
//! let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
//!     .line_to(Coord2(5.0, 1.0))
//!     .line_to(Coord2(5.0, 5.0))
//!     .line_to(Coord2(1.0, 5.0))
//!     .line_to(Coord2(1.0, 1.0))
//!     .build();
//! ```
//! 
//! Path arithmetic:
//! 
//! ```
//! use flo_curves::*;
//! use flo_curves::arc::*;
//! use flo_curves::bezier::path::*;
//! 
//! let rectangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
//!     .line_to(Coord2(5.0, 1.0))
//!     .line_to(Coord2(5.0, 5.0))
//!     .line_to(Coord2(1.0, 5.0))
//!     .line_to(Coord2(1.0, 1.0))
//!     .build();
//! let circle = Circle::new(Coord2(3.0, 3.0), 1.0).to_path::<SimpleBezierPath>();
//!
//! let rectangle_with_hole = path_sub::<SimpleBezierPath>(&vec![rectangle], &vec![circle], 0.01);
//! ```
//! 

#![warn(bare_trait_objects)]

// Breaks the exported API if auto-fixed (can remove these with a version bump)
#![allow(clippy::ptr_arg)]
#![allow(clippy::from_over_into)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]

// Breaks stylistic choices/algorithm readability
#![allow(clippy::redundant_field_names)]                // Used for consistency when initialising some types
#![allow(clippy::collapsible_if)]                       // Often used to clarify algorithm structure (rewrites need to be at least as clear)
#![allow(clippy::collapsible_else_if)]                  // Often used to clarify algorithm structure
#![allow(clippy::if_same_then_else)]                    // Often used to clarify algorithm structure
#![allow(clippy::module_inception)]                     // The 'line' module has a 'Line' type in it, for example. Makes sense the file is called 'line'...
#![allow(clippy::let_and_return)]                       // Often we want to say what the return value means (eg: calling something 'tangent' instead of just 'de_cateljau3' is much more clear)

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
