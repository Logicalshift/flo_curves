use flo_curves::bezier::*;
use flo_curves::line::*;

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

#[test]
fn fit_straight_line() {
    let curve       = line_to_bezier::<Curve<Coord2>>(&(Coord2(412.0, 500.0), Coord2(308.0, 665.0)));
    let points      = (0..=100).map(|t| t as f64/100.0).map(|t| curve.point_at_pos(t)).collect::<Vec<_>>();
    let fit_curve   = fit_curve::<Curve<Coord2>>(&points, 0.01);

    assert!(fit_curve.is_some());
    let fit_curve = fit_curve.unwrap();

    assert!(fit_curve.len() == 1, "{} curves: {:?}", fit_curve.len(), fit_curve);

    for some_curve in fit_curve {
        for t in 0..=20 {
            let t = t as f64/20.0;
            let p = some_curve.point_at_pos(t);
            let d = curve.distance_to(&p);

            assert!(d < 0.001, "Distance = {} at t={}", d, t);
        }
    }
}

#[test]
fn fit_square() {
    let points = (0..=100).map(|t| {
        let p = (t%25) as f64 / 25.0 * 100.0;
        if t < 25 {
            Coord2(100.0 + p, 100.0)
        } else if t < 50 {
            Coord2(200.0, 100.0 + p)
        } else if t < 75 {
            Coord2(200.0 - p, 200.0)
        } else {
            Coord2(100.0, 200.0 - p)
        }
    }).collect::<Vec<_>>();
    let fit_curve = fit_curve::<Curve<Coord2>>(&points, 0.01);

    assert!(fit_curve.is_some());
    let fit_curve = fit_curve.unwrap();

    assert!(fit_curve.len() == 7, "{} curves: {:?}", fit_curve.len(), fit_curve);

    for some_curve in fit_curve {
        for t in 0..=20 {
            let t = t as f64/20.0;
            let p = some_curve.point_at_pos(t);

            assert!((p.x()-100.0).abs() < 0.01 || (p.y()-100.0).abs() < 0.01 || (p.x()-200.0).abs() < 0.01 || (p.y()-200.0).abs() < 0.01, "{:?}", p);
        }
    }
}
