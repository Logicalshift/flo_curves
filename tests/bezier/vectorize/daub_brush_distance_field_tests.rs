use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;

use itertools::*;

use std::f64;
use std::collections::{HashMap};

fn check_contour_against_bitmap<TContour: SampledContour>(contour: TContour) {
    check_intercepts(contour);

    // Use point_is_inside to generate a bitmap version of the contour
    let bitmap = (0..(contour.contour_size().0 * contour.contour_size().1)).into_iter()
        .map(|pos| (pos % contour.contour_size().1, pos / contour.contour_size().1))
        .map(|(x, y)| contour.point_is_inside(ContourPosition(x, y)))
        .collect::<Vec<_>>();

    let bitmap = BoolSampledContour(contour.contour_size(), bitmap);

    // Get the edges from both
    let bitmap_edges    = bitmap.edge_cell_iterator().collect::<Vec<_>>();
    let contour_edges   = contour.edge_cell_iterator().collect::<Vec<_>>();

    // Should generate identical results
    let edges_for_y_bitmap  = bitmap_edges.iter().cloned().group_by(|(pos, _)| pos.1).into_iter().map(|(ypos, group)| (ypos, group.count())).collect::<HashMap<_, _>>();
    let edges_for_y_contour  = contour_edges.iter().cloned().group_by(|(pos, _)| pos.1).into_iter().map(|(ypos, group)| (ypos, group.count())).collect::<HashMap<_, _>>();

    assert!(edges_for_y_bitmap.len() == edges_for_y_contour.len(), "Returned different number of lines (bitmap has {} vs contour with {})\n{:?}\n\n{:?}", edges_for_y_bitmap.len(), edges_for_y_contour.len(), bitmap_edges, contour_edges);
    assert!(contour_edges.len() == bitmap_edges.len(), "Returned different number of edges ({} vs {}). Edges counts were: \n  {}\n\nBitmap edges were \n  {}\n\nContour edges were \n  {}",
        bitmap_edges.len(),
        contour_edges.len(),
        edges_for_y_bitmap.keys().map(|ypos| format!("{} {:?} {:?}", ypos, edges_for_y_bitmap.get(ypos), edges_for_y_contour.get(ypos))).collect::<Vec<_>>().join("\n  "),
        bitmap_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "),
        contour_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "));

    assert!(contour_edges == bitmap_edges, "Edges were \n  {}", 
        bitmap_edges.iter().zip(contour_edges.iter())
            .map(|(bitmap_edge, contour_edge)| format!("({:?}) {:?}    {:?}", bitmap_edge == contour_edge, bitmap_edge, contour_edge))
            .collect::<Vec<_>>()
            .join("\n  "));
}

fn check_intercepts<TContour: SampledContour>(contour: TContour) {
    for y in 0..contour.contour_size().height() {
        let intercepts  = contour.intercepts_on_line(y);
        let mut row     = vec![false; contour.contour_size().width()];

        for intercept in intercepts.iter() {
            for x in intercept.clone() {
                assert!(row[x] == false, "Overlapping intercept at {}, {}", x, y);
                row[x] = true;
            }
        }

        for x in 0..contour.contour_size().width() {
            assert!(row[x] == contour.point_is_inside(ContourPosition(x, y)), "Row content mismatch at y={} {:?} (intercepts look like:\n  {} but should be:\n  {})", y, intercepts,
                row.iter().map(|p| if *p { '#' } else { '.' }).collect::<String>(),
                (0..contour.contour_size().width()).into_iter().map(|x| if contour.point_is_inside(ContourPosition(x, y)) { '#' } else { '.' }).collect::<String>());
        }
    }
}

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

    let circle_contours     = (&circle_1.as_contour()).edge_cell_iterator().collect::<Vec<_>>();
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
    let size        = distance_field.field_size();
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

    let center = 36.0;
    for path in doughnut.iter() {
        let mut max_distance = 0.0;
        let mut min_distance = 1e12;

        for curve in path.to_curves::<Curve<Coord2>>() {
            for t in 0..100 {
                let t           = (t as f64)/100.0;
                let point       = curve.point_at_pos(t);
                let distance    = point.distance_to(&Coord2(center+1.0, center+1.0));

                max_distance = f64::max(max_distance, distance);
                min_distance = f64::min(min_distance, distance);
            }
        }

        assert!((max_distance-35.0).abs() <= 1.0 || (max_distance-25.0).abs() <= 1.0, "Max distance incorrect: {:?} {:?}\n\n{}\n", max_distance, min_distance, text_field);
        assert!((min_distance-35.0).abs() <= 1.0 || (min_distance-25.0).abs() <= 1.0, "Min distance incorrect: {:?} {:?}\n\n{}\n", max_distance, min_distance, text_field);
    }
}

#[test]
fn brush_stroke_intercept_scan() {
    // TODO: looks like we generate a lot of blank lines here

    let pos  = 0.3 * 2.0*f64::consts::PI;
    let pos  = (pos.sin() + 1.0) * 200.0;
    let off1 = 200.0 - pos/2.0;
    let off2 = pos/2.0;

    let t  = 0.4f64;
    let p0 = Coord2(-(t*1.0/2.0).cos() * 400.0, (t*1.0/3.0).sin() * 500.0) + Coord2(500.0, 500.0);
    let p1 = Coord2(-(t*2.0/3.0).cos() * 400.0, (t*1.0/4.0).sin() * 200.0) + Coord2(500.0, 500.0);
    let p2 = Coord2((t*1.0/4.0).cos() * 200.0, -(t*2.0/3.0).sin() * 400.0) + Coord2(500.0, 500.0);
    let p3 = Coord2((t*1.0/3.0).cos() * 500.0, -(t*1.0/2.0).sin() * 200.0) + Coord2(500.0, 500.0);

    let p0_3 = Coord3::from((p0, off1));
    let p1_3 = Coord3::from((p1, (off2-off1)*(1.0/3.0) + off1));
    let p2_3 = Coord3::from((p2, (off2-off1)*(2.0/3.0) + off1));
    let p3_3 = Coord3::from((p3, off2));

    let brush_curve      = Curve::from_points(p0_3, (p1_3, p2_3), p3_3);
    let (daubs, _offset) = brush_stroke_daubs::<CircularDistanceField, _>(&brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_intercepts(&daub_distance_field);
}

#[test]
fn doughnut_intercept_scan() {
    // Create a distance field from 300 grid-aligned circles
    let brush           = CircularDistanceField::with_radius(5.0);
    let distance_field  = DaubBrushDistanceField::from_daubs((0..300).into_iter()
        .map(|t| {
            let t       = (t as f64)/300.0;
            let t       = t * (f64::consts::PI * 2.0);
            let (x, y)  = (t.sin()*30.0 + 30.0, t.cos()*30.0 + 30.0);
            (&brush, ContourPosition(x.round() as _, y.round() as _))
        }));

    check_intercepts(&distance_field);
}

#[test]
fn circle_at_position() {
    let center          = Coord2(123.4, 345.6);
    let radius          = 32.1;
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![CircularDistanceField::centered_at_position(center, radius).unwrap()]);
    let circle          = trace_paths_from_distance_field::<SimpleBezierPath>(&distance_field, 0.1);

    assert!(circle.len() == 1);

    for curve in circle[0].to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t           = (t as f64)/100.0;
            let point       = curve.point_at_pos(t);
            let distance    = point.distance_to(&Coord2(center.0, center.1));

            assert!((distance-radius).abs() < 0.2, "Found point at distance {:?}", distance);
        }
    }
}

#[test]
fn broken_brush_stroke_check_contour_1() {
    // 463 367.161472273654 16.419263863173 183.580736136827
    let counter = 463;

    let pos  = (counter as f64)/400.0 * 2.0*f64::consts::PI;
    let pos  = (pos.sin() + 1.0) * 200.0;
    let off1 = 200.0 - pos/2.0;
    let off2 = pos/2.0;

    let t  = 0.4f64;
    let p0 = Coord2(-(t*1.0/2.0).cos() * 400.0, (t*1.0/3.0).sin() * 500.0) + Coord2(500.0, 500.0);
    let p1 = Coord2(-(t*2.0/3.0).cos() * 400.0, (t*1.0/4.0).sin() * 200.0) + Coord2(500.0, 500.0);
    let p2 = Coord2((t*1.0/4.0).cos() * 200.0, -(t*2.0/3.0).sin() * 400.0) + Coord2(500.0, 500.0);
    let p3 = Coord2((t*1.0/3.0).cos() * 500.0, -(t*1.0/2.0).sin() * 200.0) + Coord2(500.0, 500.0);

    let p0_3 = Coord3::from((p0, off1));
    let p1_3 = Coord3::from((p1, (off2-off1)*(1.0/3.0) + off1));
    let p2_3 = Coord3::from((p2, (off2-off1)*(2.0/3.0) + off1));
    let p3_3 = Coord3::from((p3, off2));

    let brush_curve      = Curve::from_points(p0_3, (p1_3, p2_3), p3_3);
    let (daubs, _offset) = brush_stroke_daubs::<CircularDistanceField, _>(&brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_contour_against_bitmap(&daub_distance_field);
}
