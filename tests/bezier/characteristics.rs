use flo_curves::bezier::{
    characterize_curve, features_for_curve, BezierCurve, BezierCurve2D, BezierCurveFactory, Coord2,
    Coordinate, Curve, CurveFeatures,
};
use flo_curves::{bezier, Line};

#[test]
fn detect_loop_1() {
    let curve = Curve::from_points(
        Coord2(110.0, 150.0),
        (Coord2(287.0, 227.0), Coord2(70.0, 205.0)),
        Coord2(205.0, 159.0),
    );
    assert!(curve.characteristics() == bezier::CurveCategory::Loop);
}

#[test]
fn detect_loop_2() {
    let curve = Curve::from_points(
        Coord2(549.2899780273438, 889.4202270507813),
        (
            Coord2(553.4288330078125, 893.8638305664063),
            Coord2(542.5203247070313, 889.04931640625),
        ),
        Coord2(548.051025390625, 891.1853637695313),
    );
    assert!(characterize_curve(&curve) == bezier::CurveCategory::Loop);
}

#[test]
fn detect_loop_2_features() {
    let curve = Curve::from_points(
        Coord2(549.2899780273438, 889.4202270507813),
        (
            Coord2(553.4288330078125, 893.8638305664063),
            Coord2(542.5203247070313, 889.04931640625),
        ),
        Coord2(548.051025390625, 891.1853637695313),
    );

    match features_for_curve(&curve, 0.01) {
        CurveFeatures::Loop(t1, t2) => {
            let (p1, p2) = (curve.point_at_pos(t1), curve.point_at_pos(t2));
            assert!(p1.is_near_to(&p2, 0.01));
        }
        _ => assert!(false),
    }
}
