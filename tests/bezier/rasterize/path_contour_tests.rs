use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

#[test]
fn basic_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_contour  = PathContour::new_contour(vec![circle_path], ContourSize(1000, 1000));

    let mut num_intercepts = 0;
    for y in 0..1000 {
        let intercepts = circle_contour.intercepts_on_line(y as _);

        num_intercepts += intercepts.len();

        for range in intercepts {
            let p1 = Coord2(range.start as _, y as _);
            let p2 = Coord2(range.end as _, y as _);

            let d1 = p1.distance_to(&center);
            let d2 = p2.distance_to(&center);

            assert!((d1-radius).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
            assert!((d2-radius).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
        }
    }

    assert!(num_intercepts >= 600 && num_intercepts <= 602, "num_intercepts = {:?}", num_intercepts);
}

#[test]
fn doughnut() {
    let radius_outer    = 300.0;
    let radius_inner    = 200.0;
    let center          = Coord2(500.0, 500.0);
    let outer_circle    = Circle::new(center, radius_outer).to_path::<SimpleBezierPath>();
    let inner_circle    = Circle::new(center, radius_inner).to_path::<SimpleBezierPath>();

    let circle_contour  = PathContour::new_contour(vec![outer_circle, inner_circle], ContourSize(1000, 1000));

    let mut num_intercepts = 0;
    for y in 0..1000 {
        let intercepts = circle_contour.intercepts_on_line(y as _);

        num_intercepts += intercepts.len();

        for (idx, range) in intercepts.iter().enumerate() {
            let p1 = Coord2(range.start as _, y as _);
            let p2 = Coord2(range.end as _, y as _);

            let d1 = p1.distance_to(&center);
            let d2 = p2.distance_to(&center);

            if intercepts.len() == 1 {
                assert!((d1-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                assert!((d2-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
            } else {
                assert!(idx < 2);

                if idx == 0 {
                    assert!((d1-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                    assert!((d2-radius_inner).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                } else {
                    assert!((d1-radius_inner).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                    assert!((d2-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                }
            }
        }
    }

    // 600 intercepts on the outer circle, 400 on in the inner
    assert!(num_intercepts >= 997 && num_intercepts <= 1003, "num_intercepts = {:?}", num_intercepts);
}
