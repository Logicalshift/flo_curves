use flo_curves::bezier::path::{BezierPathBuilder, PathWithIsClockwise, SimpleBezierPath};
use flo_curves::Coord2;

#[test]
pub fn rectangle_is_clockwise() {
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .build();

    assert!(rectangle1.is_clockwise());
}

#[test]
pub fn rectangle_is_anticlockwise() {
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    assert!(!rectangle1.is_clockwise());
}
