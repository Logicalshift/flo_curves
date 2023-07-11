use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

use itertools::*;

#[test]
fn corners_are_outside() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));

    assert!(circle_field.distance_at_point(ContourPosition(0, 0)) > 0.0, "Distance at 0,0 is {:?}", circle_field.distance_at_point(ContourPosition(0, 0)));
    assert!(circle_field.distance_at_point(ContourPosition(999, 0)) > 0.0, "Distance at 999,0 is {:?}", circle_field.distance_at_point(ContourPosition(999, 0)));
    assert!(circle_field.distance_at_point(ContourPosition(0, 999)) > 0.0, "Distance at 0,999 is {:?}", circle_field.distance_at_point(ContourPosition(0, 999)));
    assert!(circle_field.distance_at_point(ContourPosition(999, 999)) > 0.0, "Distance at 999,999 is {:?}", circle_field.distance_at_point(ContourPosition(999, 999)));
}

#[test]
fn center_is_inside() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));

    println!("{:?}", circle_field.distance_at_point(ContourPosition(501, 501)));

    assert!(circle_field.distance_at_point(ContourPosition(501, 501)) < 0.0, "Distance at center is {:?}", circle_field.distance_at_point(ContourPosition(501, 501)));
    assert!(circle_field.distance_at_point(ContourPosition(500, 500)) < 0.0, "Distance near center is {:?}", circle_field.distance_at_point(ContourPosition(500, 500)));
    assert!(circle_field.distance_at_point(ContourPosition(499, 499)) < 0.0, "Distance near center is {:?}", circle_field.distance_at_point(ContourPosition(499, 499)));
}

#[test]
fn outside_point_distances() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));

    for y in 0..1000 {
        for x in 0..1000 {
            let to_center = Coord2(x as _, y as _).distance_to(&center);

            // Points with a distance shorter than the radius will be outside of the circle
            if to_center <= radius {
                continue;
            }

            let field_distance = circle_field.distance_at_point(ContourPosition(x, y));

            assert!((field_distance-(to_center-300.0)).abs() < 2.0, "Distance at {}, {} is {} ({} to center)", x, y, field_distance, to_center);

            if field_distance < 0.0 && field_distance >= -0.05 {
                // Might be an inaccuracy due to the path
            } else {
                // Usual test: point must be inside circle
                assert!(field_distance >= 0.0, "Distance at {}, {} is {} (this point is not inside the circle)", x, y, field_distance);
            }
        }
    }
}

#[test]
fn inside_point_distances() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(1000, 1000));

    for y in 0..1000 {
        for x in 0..1000 {
            let to_center = Coord2(x as _, y as _).distance_to(&center);

            if to_center >= radius {
                continue;
            }

            let field_distance = circle_field.distance_at_point(ContourPosition(x, y));

            assert!(field_distance <= 0.0, "Distance at {}, {} is {} (this point is not outside the circle)", x, y, field_distance);
            assert!((field_distance-(to_center-300.0)).abs() < 2.0, "Distance at {}, {} is {} ({} to center)", x, y, field_distance, to_center);
        }
    }
}

#[test]
fn nearby_point_distances() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path.clone()], ContourSize(1000, 1000));

    for y in 0..1000 {
        for x in 0..1000 {
            let field_distance  = circle_field.distance_at_point(ContourPosition(x, y));
            let to_center       = Coord2(x as _, y as _).distance_to(&center);

            if field_distance.abs() < 1.0 {
                let path_distance = circle_path.to_curves::<Curve<_>>()
                    .into_iter()
                    .map(|curve| curve.nearest_point(&Coord2(x as _, y as _)))
                    .map(|nearest| nearest.distance_to(&Coord2(x as _, y as _)))
                    .reduce(f64::min)
                    .unwrap();

                assert!((path_distance.abs()-field_distance.abs()).abs() < 0.1, "Point ({}, {}) has a distance of {} in the field but closest point has distance {} (perfect distance is {})", x, y, field_distance, path_distance, to_center - radius);
            }
        }
    }
}

#[test]
fn trace_circle_without_distance_field() {
    // This is the equivalent of trace_circle except we don't load it into a distance field first
    // If this test fails, then the other test will likely fail due to a problem with tracing the points rather than the distance field
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_points   = circle_path.to_curves::<Curve<_>>()
        .into_iter()
        .flat_map(|curve| {
            walk_curve_evenly_map(curve, 1.0, 0.1, |section| section.point_at_pos(1.0))
        })
        .collect::<Vec<_>>();
    let traced_circle   = fit_curve::<Curve<_>>(&circle_points, 0.1).unwrap();

    debug_assert!(traced_circle.len() < 20, "Result has {} curves", traced_circle.len());

    let mut num_points = 0;
    for curve in traced_circle {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(500.0, 500.0));

            debug_assert!((distance - radius) < 1.0, "Point #{} at distance {:?}", num_points, distance);
        }
    }
}

#[test]
fn trace_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = PathDistanceField::from_path(vec![circle_path.clone()], ContourSize(1000, 1000));
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&circle_field, 0.1);

    debug_assert!(traced_circle.len() == 1);

    // Test against the ideal circle
    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(501.0, 501.0));

            debug_assert!((distance - radius) < 0.2, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    // Test against the actual path
    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);
            let point       = point - Coord2(1.0, 1.0);

            let nearest_distance = circle_path.to_curves::<Curve<_>>().into_iter()
                .map(|curve| curve.distance_to(&point))
                .reduce(|d1, d2| d1.min(d2))
                .unwrap();

            debug_assert!(nearest_distance.abs() < 0.1, "Point #{} at distance {:?}", num_points, nearest_distance);
        }
    }

    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_chisel_contours() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();

    let (chisel_field, offset)  = PathDistanceField::center_path(vec![chisel.clone()]);
    let traced_chisel           = trace_contours_from_distance_field::<Coord2>(&chisel_field);

    debug_assert!(traced_chisel.len() == 1);

    let mut num_points  = 0;
    let mut max_error   = 0.0f64;
    let mut total_error = 0.0f64;
    let mut error_count = 0;
    for point in traced_chisel[0].iter().copied() {
        num_points += 1;

        let point = point + offset - Coord2(1.0, 1.0);

        let nearest_distance = chisel.to_curves::<Curve<_>>().into_iter()
            .map(|curve| curve.distance_to(&point))
            .reduce(|d1, d2| d1.min(d2))
            .unwrap()
            .abs();
        max_error   = max_error.max(nearest_distance);
        total_error += nearest_distance;

        if nearest_distance > 0.1 {
            error_count += 1;
        }
    }

    let avg_error = total_error / (num_points as f64);

    debug_assert!(max_error < 0.1, "Max error was {} (average {}, num >0.1 {}/{})", max_error, avg_error, error_count, num_points);
}

#[test]
fn chisel_no_very_close_points() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();
    let (chisel_field, _) = PathDistanceField::center_path(vec![chisel.clone()]);

    let chisel_points = trace_contours_from_distance_field::<Coord2>(&chisel_field);
    assert!(chisel_points.len() > 0);

    for subpath in chisel_points {
        for (p1, p2) in subpath.iter().tuple_windows() {
            let distance = p1.distance_to(p2);

            assert!(distance > 0.1, "{:?} {:?} are very close", p1, p2);
            assert!(distance < 2.0, "{:?} {:?} are very far apart", p1, p2);
        }
    }
}

#[test]
fn trace_chisel_paths() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();

    let (chisel_field, offset)  = PathDistanceField::center_path(vec![chisel.clone()]);
    let traced_chisel           = trace_paths_from_distance_field::<SimpleBezierPath>(&chisel_field, 0.1);

    debug_assert!(traced_chisel.len() == 1);

    let mut num_points  = 0;
    let mut max_error   = 0.0f64;
    let mut total_error = 0.0f64;
    for curve in traced_chisel[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);
            let point       = point + offset - Coord2(1.0, 1.0);

            let nearest_distance = chisel.to_curves::<Curve<_>>().into_iter()
                .map(|curve| curve.distance_to(&point))
                .reduce(|d1, d2| d1.min(d2))
                .unwrap();
            max_error   = max_error.max(nearest_distance);
            total_error += nearest_distance;

            debug_assert!(nearest_distance.abs() < 0.4, "Point #{} at distance {:?}", num_points, nearest_distance);
        }
    }

    let avg_error = total_error / (num_points as f64);

    debug_assert!(max_error < 0.4, "Max error was {:?} (average {:?})", max_error, avg_error);
    debug_assert!(traced_chisel[0].to_curves::<Curve<_>>().len() < 16, "Result has {} curves", traced_chisel[0].to_curves::<Curve<_>>().len());
}
