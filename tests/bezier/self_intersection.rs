use flo_curves::bezier::{
    find_self_intersection_point, BezierCurve, BezierCurveFactory, Coord2, Coordinate, Curve,
};

#[test]
fn find_simple_self_intersection() {
    let curve_with_loop = Curve::from_points(
        Coord2(148.0, 151.0),
        (Coord2(292.0, 199.0), Coord2(73.0, 221.0)),
        Coord2(249.0, 136.0),
    );
    let intersection_point = find_self_intersection_point(&curve_with_loop, 0.01);

    assert!(intersection_point.is_some());

    let (t1, t2) = intersection_point.unwrap();
    let (p1, p2) = (
        curve_with_loop.point_at_pos(t1),
        curve_with_loop.point_at_pos(t2),
    );

    assert!(p1.is_near_to(&p2, 0.01));
}

#[test]
fn whole_curve_is_a_loop() {
    let curve_with_loop = Curve::from_points(
        Coord2(205.0, 159.0),
        (Coord2(81.0, 219.0), Coord2(287.0, 227.0)),
        Coord2(205.0, 159.0),
    );
    let intersection_point = find_self_intersection_point(&curve_with_loop, 0.01);

    assert!(intersection_point.is_some());

    let (t1, t2) = intersection_point.unwrap();
    let (p1, p2) = (
        curve_with_loop.point_at_pos(t1),
        curve_with_loop.point_at_pos(t2),
    );

    assert!(p1.is_near_to(&p2, 0.01));
    assert!(t1 <= 0.0);
    assert!(t2 >= 1.0);
}

#[test]
fn narrow_loop() {
    let curve_with_loop = Curve::from_points(
        Coord2(549.2899780273438, 889.4202270507813),
        (
            Coord2(553.4288330078125, 893.8638305664063),
            Coord2(542.5203247070313, 889.04931640625),
        ),
        Coord2(548.051025390625, 891.1853637695313),
    );
    let intersection_point = find_self_intersection_point(&curve_with_loop, 0.01);

    assert!(intersection_point.is_some());

    let (t1, t2) = intersection_point.unwrap();
    let (p1, p2) = (
        curve_with_loop.point_at_pos(t1),
        curve_with_loop.point_at_pos(t2),
    );

    assert!(p1.is_near_to(&p2, 0.01));
}
