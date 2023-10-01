use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::line::line_to_bezier;

#[test]
fn line_is_flat_1() {
    let line        = (Coord2(100.0, 100.0), Coord2(1234.0, 5678.0));
    let line        = line_to_bezier::<Curve<_>>(&line);
    let flatness    = line.flatness();

    assert!((flatness-0.0).abs() < 1e-6, "Line is not flat");
}

#[test]
fn line_is_flat_2() {
    let line        = Curve::from_points(Coord2(100.0, 100.0), (Coord2(100.0, 100.0), Coord2(1234.0, 5678.0)), Coord2(1234.0, 5678.0));
    let flatness    = line.flatness();

    assert!((flatness-0.0).abs() < 1e-6, "Line is not flat");
}
