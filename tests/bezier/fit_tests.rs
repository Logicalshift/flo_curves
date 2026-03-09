use flo_curves::bezier::*;

#[test]
fn fit_basic_curve() {
    let curve       = Curve::from_points(Coord2(412.0, 500.0), (Coord2(442.0, 520.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let points      = (0..=100).map(|t| t as f64/100.0).map(|t| curve.point_at_pos(t)).collect::<Vec<_>>();
    let fit_curve   = fit_curve::<Curve<Coord2>>(&points, 0.01);

    assert!(fit_curve.is_some());
    let fit_curve = fit_curve.unwrap();

    assert!(fit_curve.len() == 14, "{} curves: {:?}", fit_curve.len(), fit_curve);

    for some_curve in fit_curve {
        for t in 0..=20 {
            let t = t as f64/20.0;
            let p = some_curve.point_at_pos(t);
            let d = curve.distance_to(&p);

            assert!(d < 0.02, "Distance = {} at t={}", d, t);
        }
    }
}

#[test]
fn fit_basic_curve_degenerate() {
    let curve       = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let points      = (0..=100).map(|t| t as f64/100.0).map(|t| curve.point_at_pos(t)).collect::<Vec<_>>();
    let fit_curve   = fit_curve::<Curve<Coord2>>(&points, 0.01);

    assert!(fit_curve.is_some());
    let fit_curve = fit_curve.unwrap();

    assert!(fit_curve.len() == 9, "{} curves: {:?}", fit_curve.len(), fit_curve);

    for some_curve in fit_curve {
        for t in 0..=20 {
            let t = t as f64/20.0;
            let p = some_curve.point_at_pos(t);
            let d = curve.distance_to(&p);

            assert!(d < 0.02, "Distance = {} at t={}", d, t);
        }
    }
}
