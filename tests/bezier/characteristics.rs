use flo_curves::*;
use flo_curves::bezier::*;

#[test]
fn detect_loop() {
    let curve = Curve::from_points(Coord2(110.0, 150.0), (Coord2(287.0, 227.0), Coord2(70.0, 205.0)), Coord2(205.0, 159.0));
    assert!(curve.characteristics() == bezier::CurveCategory::Loop);
}
