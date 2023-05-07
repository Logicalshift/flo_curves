use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;

use std::f64;

#[test]
fn overlapping_circles_point_inside_first() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let circle_2        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(5, 5)),
        (&circle_2, ContourPosition(5, 18)),
    ]);

    assert!(distance_field.as_contour().point_is_inside(ContourPosition(15, 8)));
    assert!((distance_field.distance_at_point(ContourPosition(16, 16)) - -10.0).abs() < 0.1, "{}", distance_field.distance_at_point(ContourPosition(16, 16)));
}

#[test]
fn overlapping_circles_point_inside_second() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let circle_2        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(5, 5)),
        (&circle_2, ContourPosition(5, 18)),
    ]);

    assert!(distance_field.as_contour().point_is_inside(ContourPosition(15, 21)));
    assert!((distance_field.distance_at_point(ContourPosition(16, 29)) - -10.0).abs() < 0.1, "{}", distance_field.distance_at_point(ContourPosition(16, 29)));
}

#[test]
fn single_circle_contours() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(0, 0)),
    ]);

    let circle_contours     = circle_1.as_contour().edge_cell_iterator().collect::<Vec<_>>();
    let distance_contours   = distance_field.as_contour().edge_cell_iterator().collect::<Vec<_>>();

    assert!(circle_contours == distance_contours, "{:?}\n\n{:?}", distance_contours, circle_contours);
}

#[test]
fn trace_single_circle_only_samples() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(5, 5)),
    ]);

    let circle = trace_paths_from_samples::<SimpleBezierPath>(distance_field.as_contour(), 0.1);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);

    // Allow 0.1px of error (distance fields provide much better estimates of where the edge really is)
    let mut max_error   = 0.0;
    let center          = 16.5;
    let radius          = 10.0;

    for curve in circle[0].to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t           = (t as f64)/100.0;
            let point       = curve.point_at_pos(t);
            let distance    = point.distance_to(&Coord2(center+1.0, center+1.0));
            let offset      = (distance-radius).abs();

            max_error = f64::max(max_error, offset);
        }
    }

    assert!(max_error <= 2.0, "Max error {:?} > 2.0. Path generated was {:?}", max_error, circle);
}

#[test]
fn trace_single_circle() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(5, 5)),
    ]);

    let circle = trace_paths_from_distance_field::<SimpleBezierPath>(&distance_field, 0.1);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);

    // Allow 0.1px of error (distance fields provide much better estimates of where the edge really is)
    let mut max_error   = 0.0;
    let center          = 16.5;
    let radius          = 10.0;

    for curve in circle[0].to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t           = (t as f64)/100.0;
            let point       = curve.point_at_pos(t);
            let distance    = point.distance_to(&Coord2(center+1.0, center+1.0));
            let offset      = (distance-radius).abs();

            max_error = f64::max(max_error, offset);
        }
    }

    assert!(max_error <= 1.0, "Max error {:?} > 1.0. Path generated was {:?}", max_error, circle);
}

#[test]
fn trace_overlapping_circle() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let circle_2        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(5, 5)),
        (&circle_2, ContourPosition(5, 5)),
    ]);

    let circle = trace_paths_from_distance_field::<SimpleBezierPath>(&distance_field, 0.1);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);

    // Allow 0.1px of error (distance fields provide much better estimates of where the edge really is)
    let mut max_error   = 0.0;
    let center          = 16.5;
    let radius          = 10.0;

    for curve in circle[0].to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t           = (t as f64)/100.0;
            let point       = curve.point_at_pos(t);
            let distance    = point.distance_to(&Coord2(center+1.0, center+1.0));
            let offset      = (distance-radius).abs();

            max_error = f64::max(max_error, offset);
        }
    }

    assert!(max_error <= 1.0, "Max error {:?} > 1.0. Path generated was {:?}", max_error, circle);
}

#[test]
fn trace_int_doughnut() {
    // Create a distance field from 300 grid-aligned circles
    let brush           = CircularDistanceField::with_radius(5.0);
    let distance_field  = DaubBrushDistanceField::from_daubs((0..300).into_iter()
        .map(|t| {
            let t       = (t as f64)/300.0;
            let t       = t * (f64::consts::PI * 2.0);
            let (x, y)  = (t.sin()*30.0 + 30.0, t.cos()*30.0 + 30.0);
            (&brush, ContourPosition(x.round() as _, y.round() as _))
        }));

    // Create a text representation of the distance field for debugging
    let size        = distance_field.size();
    let text_field  = (0..size.height()).into_iter()
        .map(|y| {
            (0..size.width()).into_iter()
                .map(|x| {
                    if distance_field.as_contour().point_is_inside(ContourPosition(x, y)) {
                        "#"
                    } else {
                        "."
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Should trace as a 'doughnut' shape
    let doughnut = trace_paths_from_distance_field::<SimpleBezierPath>(&distance_field, 0.1);
    assert!(doughnut.len() == 2, "Made {} paths for the 'doughnut' shape ({:?})\n\n{}\n", doughnut.len(), doughnut, text_field);
}

