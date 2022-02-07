use flo_curves::bezier;
use flo_curves::{BezierCurve, BezierCurveFactory, Coord2, Coordinate, Line};

mod algorithms;
mod path;

mod basis;
mod bounds;
mod characteristics;
mod curve_intersection_clip;
mod deform;
mod derivative;
mod distort;
mod intersection;
mod length;
mod normal;
mod offset;
mod overlaps;
mod search;
mod section;
mod self_intersection;
mod solve;
mod subdivide;
mod tangent;
mod walk;

pub fn approx_equal(a: f64, b: f64) -> bool {
    f64::floor(f64::abs(a - b) * 10000.0) == 0.0
}

#[test]
fn read_curve_control_points() {
    let curve = bezier::Curve::from_points(
        Coord2(1.0, 1.0),
        (Coord2(3.0, 3.0), Coord2(4.0, 4.0)),
        Coord2(2.0, 2.0),
    );

    assert!(curve.start_point() == Coord2(1.0, 1.0));
    assert!(curve.end_point() == Coord2(2.0, 2.0));
    assert!(curve.control_points() == (Coord2(3.0, 3.0), Coord2(4.0, 4.0)));
}

#[test]
fn read_curve_points() {
    let curve = bezier::Curve::from_points(
        Coord2(1.0, 1.0),
        (Coord2(3.0, 3.0), Coord2(4.0, 4.0)),
        Coord2(2.0, 2.0),
    );

    for x in 0..100 {
        let t = (x as f64) / 100.0;

        let point = curve.point_at_pos(t);
        let another_point = bezier::de_casteljau4(
            t,
            Coord2(1.0, 1.0),
            Coord2(3.0, 3.0),
            Coord2(4.0, 4.0),
            Coord2(2.0, 2.0),
        );

        assert!(point.distance_to(&another_point) < 0.001);
    }
}
