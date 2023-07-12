use super::daub_brush_distance_field_tests::{check_contour_against_bitmap};

use flo_curves::arc::*;
use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

use std::f64;

fn curve_is_smooth<TCurve>(curve: &TCurve) -> bool
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate2D, 
{
    let (sp, (cp1, cp2), ep) = curve.all_points();
    let (d1, d2, d3) = (sp.distance_to(&cp1), cp2.distance_to(&ep), sp.distance_to(&ep));

    if (d1 > d3 * 20.0) || (d2 > d3 * 20.0) {
        return false;
    }

    true
}

fn path_is_smooth<TPath>(path: &TPath) -> bool 
where
    TPath: BezierPath,
    TPath::Point: Coordinate2D,
{
    for curve in path.to_curves::<Curve<_>>() {
        if !curve_is_smooth(&curve) {
            return false;
        }
    }

    return true;
}

fn brush_curve(counter: i64) -> Curve<Coord3> {
    let pos  = (counter as f64)/400.0 * 2.0*f64::consts::PI;
    let pos  = (pos.sin() + 1.0) * 200.0;
    let off1 = 200.0 - pos/2.0;
    let off2 = pos/2.0;

    let t  = (counter as f64) / 40.0; 
    let p0 = Coord2(-(t*1.0/2.0).cos() * 400.0, (t*1.0/3.0).sin() * 500.0) + Coord2(500.0, 500.0);
    let p1 = Coord2(-(t*2.0/3.0).cos() * 400.0, (t*1.0/4.0).sin() * 200.0) + Coord2(500.0, 500.0);
    let p2 = Coord2((t*1.0/4.0).cos() * 200.0, -(t*2.0/3.0).sin() * 400.0) + Coord2(500.0, 500.0);
    let p3 = Coord2((t*1.0/3.0).cos() * 500.0, -(t*1.0/2.0).sin() * 200.0) + Coord2(500.0, 500.0);

    let p0_3 = Coord3::from((p0, off1));
    let p1_3 = Coord3::from((p1, (off2-off1)*(1.0/3.0) + off1));
    let p2_3 = Coord3::from((p2, (off2-off1)*(2.0/3.0) + off1));
    let p3_3 = Coord3::from((p3, off2));

    let brush_curve      = Curve::from_points(p0_3, (p1_3, p2_3), p3_3);

    brush_curve
}

#[test]
fn broken_brush_is_smooth_1() {
    // 463 367.161472273654 16.419263863173 183.580736136827
    let counter = 463;

    let brush_curve      = brush_curve(counter);
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);
    let paths               = trace_paths_from_distance_field::<SimpleBezierPath>(&daub_distance_field, 0.5);

    for path in paths {
        assert!(path_is_smooth(&path));
    }
}

#[test]
fn broken_brush_is_smooth_2() {
    for counter in 464..507 {
        println!("counter = {:?}", counter);

        let brush_curve      = brush_curve(counter);
        let (daubs, _offset) = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

        let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);
        let paths               = trace_paths_from_distance_field::<SimpleBezierPath>(&daub_distance_field, 0.5);

        for path in paths {
            assert!(path_is_smooth(&path));
        }
    }
}

#[test]
fn broken_brush_is_smooth_3() {
    for counter in 370..390 {
        println!("counter = {}", counter);

        let brush_curve = brush_curve(counter);
        let paths       = brush_stroke_from_curve::<SimpleBezierPath, _, _>(&CircularBrush, &brush_curve, 0.5, 0.25);

        for path in paths {
            assert!(path_is_smooth(&path));
        }
    }
}

#[test]
fn broken_brush_is_smooth_4() {
    for counter in 370..390 {
        println!("counter = {}", counter);

        let brush_curve = brush_curve(counter);
        let brush_path  = BezierPathBuilder::<SimpleBezierPath3>::start(brush_curve.start_point()).curve_to(brush_curve.control_points(), brush_curve.end_point()).build();
        let paths       = brush_stroke_from_path::<SimpleBezierPath, _, _>(&CircularBrush, &brush_path, 0.5, 0.25);

        for path in paths {
            assert!(path_is_smooth(&path));
        }
    }
}

#[test]
fn broken_brush_is_smooth_5() {
    // 463 367.161472273654 16.419263863173 183.580736136827
    let counter = 463;

    let brush_curve      = brush_curve(counter);
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);
    let paths               = trace_paths_from_intercepts::<SimpleBezierPath>(&daub_distance_field, 0.5);

    for path in paths {
        assert!(path_is_smooth(&path));
    }
}

#[test]
fn broken_brush_stroke_check_contour_1() {
    let counter = 463;

    let brush_curve      = brush_curve(counter);
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_contour_against_bitmap(&daub_distance_field);
}

#[test]
fn broken_brush_stroke_check_contour_2() {
    let counter = 507;

    let brush_curve      = brush_curve(counter);
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_contour_against_bitmap(&daub_distance_field);
}

#[test]
fn broken_brush_stroke_check_contour_3() {
    let counter = 466;

    let brush_curve      = brush_curve(counter);
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_contour_against_bitmap(&daub_distance_field);
}

#[test]
fn broken_brush_stroke_check_contour_4() {
    let counter = 379;

    let brush_curve      = brush_curve(counter);
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_contour_against_bitmap(&daub_distance_field);
}

#[test]
fn path_brush_check_contour_1() {
    let radius              = 32.0;
    let center              = Coord2(radius+1.0, radius+1.0);
    let circle_path         = Circle::new(center, radius).to_path::<SimpleBezierPath>();
    let (circle_field, _)   = PathDistanceField::center_path(vec![circle_path], 0);

    let brush               = ScaledBrush::from_distance_field(&circle_field);

    let counter = 463;

    let brush_curve      = brush_curve(counter);
    let brush            = &brush;
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&brush, &brush_curve, 0.5, 0.25);
    let daubs            = daubs.collect::<Vec<_>>();
    assert!(daubs.len() > 1, "Created {:?} daubs", daubs.len());

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_contour_against_bitmap(&daub_distance_field);

    let paths = trace_paths_from_distance_field::<SimpleBezierPath>(&daub_distance_field, 0.5);
    assert!(paths.len() == 1, "Found {:?} paths", paths.len());

    for path in paths {
        assert!(path_is_smooth(&path));
    }
}

#[test]
fn path_brush_check_contour_2() {
    let radius              = 32.0;
    let center              = Coord2(radius+1.0, radius+1.0);
    let circle_path         = Circle::new(center, radius).to_path::<SimpleBezierPath>();
    let (circle_field, _)   = PathDistanceField::center_path(vec![circle_path], 0);

    let brush               = ScaledBrush::from_distance_field(&circle_field);

    let counter = 463;

    let brush_curve      = brush_curve(counter);
    let brush            = &brush;
    let (daubs, _offset) = brush_stroke_daubs_from_curve(&brush, &brush_curve, 0.5, 0.25);
    let daubs            = daubs.collect::<Vec<_>>();
    assert!(daubs.len() > 1, "Created {:?} daubs", daubs.len());

    let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

    check_contour_against_bitmap(&daub_distance_field);

    let paths = trace_paths_from_intercepts::<SimpleBezierPath>(&daub_distance_field, 0.5);
    assert!(paths.len() == 1, "Found {:?} paths", paths.len());

    for path in paths {
        assert!(path_is_smooth(&path));
    }
}
