use super::super::coordinate::*;
use super::super::bezier::path::*;

use std::fmt::*;

///
/// Writes out a path as a Rust simple bezier path definition
/// 
/// This can be used to generate code for a test when a path definition fails to perform as expected
///
pub fn bezier_path_to_rust_definition<C: Coordinate+Coordinate2D, P: BezierPath<Point=C>>(path: &P) -> String {
    let mut rust_code = String::new();

    let start = path.start_point();
    write!(&mut rust_code, "BezierPathBuilder::<SimpleBezierPath>::start(Coord2({}, {}))", start.x(), start.y()).unwrap();

    for (cp1, cp2, endpoint) in path.points() {
        write!(&mut rust_code, "\n    .curve_to((Coord2({}, {}), Coord2({}, {})), Coord2({}, {}))", cp1.x(), cp1.y(), cp2.x(), cp2.y(), endpoint.x(), endpoint.y()).unwrap();
    }
    write!(&mut rust_code, "\n    .build()").unwrap();

    rust_code
}
