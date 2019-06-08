use flo_curves::*;
use flo_curves::bezier::*;

#[test]
fn find_simple_self_intersection() {
    let curve_with_loop     = Curve::from_points(Coord2(148.0, 151.0), (Coord2(292.0, 199.0), Coord2(73.0, 221.0)), Coord2(249.0, 136.0));
    let intersection_point  = find_self_intersection_point(&curve_with_loop, 0.01);

    assert!(intersection_point.is_some());

    let (t1, t2) = intersection_point.unwrap();
    let (p1, p2) = (curve_with_loop.point_at_pos(t1), curve_with_loop.point_at_pos(t2));

    assert!(p1.is_near_to(&p2, 0.01));
}

#[test]
fn whole_curve_is_a_loop() {
    let curve_with_loop     = Curve::from_points(Coord2(205.0, 159.0), (Coord2(81.0, 219.0), Coord2(287.0, 227.0)), Coord2(205.0, 159.0));
    let intersection_point  = find_self_intersection_point(&curve_with_loop, 0.01);

    assert!(intersection_point.is_some());

    let (t1, t2) = intersection_point.unwrap();
    let (p1, p2) = (curve_with_loop.point_at_pos(t1), curve_with_loop.point_at_pos(t2));

    assert!(p1.is_near_to(&p2, 0.01));
    assert!(t1 <= 0.0);
    assert!(t2 >= 1.0);
}
