use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

#[test]
fn trace_half_circle_sampled() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_contour  = PathContour::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_contour    = ScaledContour::from_contour(&circle_contour, 0.5, (0.0, 0.0));
    let traced_circle   = trace_paths_from_samples::<SimpleBezierPath>(&half_contour, 2.0);

    debug_assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(251.0, 251.0));

            debug_assert!((distance - (radius/2.0)) < 2.0, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_half_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_field      = ScaledDistanceField::from_distance_field(&circle_field, 0.5, (0.0, 0.0));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&half_field, 0.1);

    debug_assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(251.0, 251.0));

            debug_assert!((distance - (radius/2.0)) < 0.3, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_half_circle_offset() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_field      = ScaledDistanceField::from_distance_field(&circle_field, 0.5, (0.3, 0.4));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&half_field, 0.1);

    debug_assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(251.3, 251.4));

            debug_assert!((distance - (radius/2.0)) < 0.3, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_third_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_field      = ScaledDistanceField::from_distance_field(&circle_field, 1.0/3.0, (0.0, 0.0));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&half_field, 0.1);

    debug_assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(500.0 / 3.0 + 1.0, 500.0 / 3.0 + 1.0));

            debug_assert!((distance - (radius/3.0)) < 1.0, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_circle_and_a_half() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let bigger_field    = ScaledDistanceField::from_distance_field(&circle_field, 1.5, (0.0, 0.0));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&bigger_field, 0.1);

    debug_assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(751.0, 751.0));

            debug_assert!((distance - (radius*1.5)) < 1.0, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}
