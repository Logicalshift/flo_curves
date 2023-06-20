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
        let intercepts = circle_contour.intercepts_on_line(y);

        num_intercepts += intercepts.len();

        for range in intercepts {
            let p1 = Coord2(range.start as _, y as _);
            let p2 = Coord2(range.end as _, y as _);

            let d1 = p1.distance_to(&center);
            let d2 = p2.distance_to(&center);

            assert!((d1-300.0).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
            assert!((d2-300.0).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
        }
    }

    assert!(num_intercepts >= 600 && num_intercepts <= 602, "num_intercepts = {:?}", num_intercepts);
}
