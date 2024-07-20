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

    assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(251.0, 251.0));

            assert!((distance - (radius/2.0)) < 2.0, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn read_distances_half_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_field      = ScaledDistanceField::from_distance_field(&circle_field, 0.5, (0.0, 0.0));

    let half_center     = center * 0.5;
    let half_radius     = radius * 0.5;

    let mut max_error   = 0.0f64;

    for y in 0..half_field.field_size().1 {
        for x in 0..half_field.field_size().0 {
            let field_distance  = half_field.distance_at_point(ContourPosition(x, y));
            let to_center       = half_center.distance_to(&Coord2(x as _, y as _));
            let to_edge         = to_center - half_radius;

            let error = (field_distance - to_edge).abs();

            assert!(error < 2.0, "{}, {} has error = {} (prior max {})", x, y, error, max_error);
            max_error = max_error.max(error);
        }
    }

    assert!(max_error < 0.3, "Max error {:?}", max_error);
}

#[test]
fn trace_half_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_field      = ScaledDistanceField::from_distance_field(&circle_field, 0.5, (0.0, 0.0));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&half_field, 0.1);

    assert!(traced_circle.len() == 1);

    let mut num_points  = 0;
    let mut max_error   = 0.0f64;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(251.0, 251.0));
            let error       = (distance - (radius/2.0)).abs();

            max_error       = max_error.max(error);

            assert!(error < 1.0, "Point #{} at distance {:?} ({:?})", num_points, distance, (distance - (radius/2.0)));
        }
    }

    assert!(max_error < 0.3, "max_error > 0.3 ({}, {} curves)", max_error, traced_circle[0].to_curves::<Curve<_>>().len());
    assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_half_circle_offset() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_field      = ScaledDistanceField::from_distance_field(&circle_field, 0.5, (0.3, 0.4));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&half_field, 0.1);

    assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(251.3, 251.4));

            assert!((distance - (radius/2.0)) < 0.3, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_third_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let half_field      = ScaledDistanceField::from_distance_field(&circle_field, 1.0/3.0, (0.0, 0.0));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&half_field, 0.1);

    assert!(traced_circle.len() == 1);

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(500.0 / 3.0 + 1.0, 500.0 / 3.0 + 1.0));

            assert!((distance - (radius/3.0)) < 1.0, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_circle_and_a_half() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));
    let bigger_field    = ScaledDistanceField::from_distance_field(&circle_field, 1.5, (0.0, 0.0));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&bigger_field, 0.1);

    assert!(traced_circle.len() == 1);
    assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 256, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());

    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(500.0 * 1.5, 500.0 * 1.5));

            assert!((distance - (radius*1.5)).abs() < 2.0, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 38, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}
