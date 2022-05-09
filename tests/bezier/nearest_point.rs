use flo_curves::*;
use flo_curves::bezier::*;
use flo_curves::line::*;

fn nearest_t_value_iteration<C>(curve: &C, point: &C::Point) -> f64
where
    C: BezierCurve,
{
    let mut min_distance    = f64::MAX;
    let mut min_t           = 0.0;

    // Walk the curve in increments of .1 pixel
    for t in walk_curve_evenly(curve, 0.1, 0.01).map(|section| section.t_for_t(0.5)) {
        let distance = curve.point_at_pos(t).distance_to(point);

        if distance < min_distance {
            min_distance = distance;
            min_t = t;
        }
    }

    min_t
}

#[test]
fn nearest_point_on_straight_line_subdivision() {
    // Create a curve from a line
    let line            = (Coord2(0.0, 0.0), Coord2(10.0, 7.0));
    let curve           = line_to_bezier::<_, bezier::Curve<_>>(&line);

    let line_near       = line.nearest_point(&Coord2(1.0, 5.0));
    let curve_near_t    = nearest_point_on_curve_subdivision(&curve, &Coord2(1.0, 5.0), 0.001);
    let curve_near      = curve.point_at_pos(curve_near_t);

    let iterate_t       = nearest_t_value_iteration(&curve, &Coord2(1.0, 5.0));
    let iterate_point   = curve.point_at_pos(iterate_t);

    println!("{:?} {:?} {:?}", line_near, curve_near, iterate_point);
    println!("{:?} {:?}", curve_near_t, iterate_t);

    assert!(iterate_point.distance_to(&curve_near) < 0.1);
    // assert!(line_near.distance_to(&curve_near) < 0.1); // -- TODO: this looks like a bug with the line algorithm
}

 
#[test]
fn nearest_point_on_curve_subdivision_1() {
    let curve = bezier::Curve::from_points(Coord2(10.0, 100.0), (Coord2(90.0, 30.0), Coord2(40.0, 140.0)), Coord2(220.0, 220.0));
    let point = Coord2(100.0, 130.0);

    let curve_near_t    = nearest_point_on_curve_subdivision(&curve, &point, 0.001);
    let curve_near      = curve.point_at_pos(curve_near_t);

    let iterate_t       = nearest_t_value_iteration(&curve, &point);
    let iterate_point   = curve.point_at_pos(iterate_t);

    println!("{:?} {:?}", curve_near, iterate_point);
    println!("{:?} {:?}", curve_near_t, iterate_t);

    assert!(iterate_point.distance_to(&curve_near) < 0.1);
}

