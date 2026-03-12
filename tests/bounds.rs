#![allow(clippy::all)]  // Tests are lower priority to fix

extern crate flo_curves;

use flo_curves::*;

#[test]
fn overlapping_rects() {
    let r1 = (Coord2(30.0, 30.0), Coord2(60.0, 40.0));
    let r2 = (Coord2(20.0, 25.0), Coord2(35.0, 35.0));

    assert!(r1.overlaps(&r2));
}

#[test]
fn non_overlapping_rects() {
    let r1 = (Coord2(30.0, 30.0), Coord2(60.0, 40.0));
    let r2 = (Coord2(20.0, 25.0), Coord2(9.0, 10.0));

    assert!(!r1.overlaps(&r2));
}

#[test]
fn same_rects() {
    let r1 = (Coord2(30.0, 30.0), Coord2(60.0, 40.0));

    assert!(r1.overlaps(&r1));
}

#[test]
fn touching_rects() {
    let r1 = (Coord2(30.0, 30.0), Coord2(60.0, 40.0));
    let r2 = (Coord2(20.0, 25.0), Coord2(30.0, 30.0));

    assert!(r1.overlaps(&r2));
}

#[test]
fn overlap_interior_rect() {
    let r1 = (Coord2(30.0, 30.0), Coord2(60.0, 50.0));
    let r2 = (Coord2(35.0, 35.0), Coord2(55.0, 45.0));

    assert!(r1.overlaps(&r2));
}

#[test]
fn overlap_exterior_rect() {
    let r1 = (Coord2(30.0, 30.0), Coord2(60.0, 40.0));
    let r2 = (Coord2(20.0, 20.0), Coord2(70.0, 50.0));

    assert!(r1.overlaps(&r2));
}

#[test]
fn from_points() {
    let r = Bounds::<Coord2>::bounds_for_points(vec![
        Coord2(30.0, 30.0),
        Coord2(60.0, 40.0),
        Coord2(45.0, 70.0),
        Coord2(10.0, 35.0)
    ]);

    assert!(r.min() == Coord2(10.0, 30.0));
    assert!(r.max() == Coord2(60.0, 70.00));
}

#[test]
fn transform_bounding_box_identity() {
    let bbox = (Coord2(10.0, 20.0), Coord2(50.0, 60.0));
    let result = transform_bounding_box_axis_aligned(&bbox, &|p| p);

    assert!(result.min() == Coord2(10.0, 20.0));
    assert!(result.max() == Coord2(50.0, 60.0));
}

#[test]
fn transform_bounding_box_translation() {
    let bbox = (Coord2(10.0, 20.0), Coord2(50.0, 60.0));
    let result = transform_bounding_box_axis_aligned(&bbox, &|p| Coord2(p.x() + 5.0, p.y() + 10.0));

    assert!(result.min() == Coord2(15.0, 30.0));
    assert!(result.max() == Coord2(55.0, 70.0));
}

#[test]
fn transform_bounding_box_scale() {
    let bbox = (Coord2(10.0, 20.0), Coord2(50.0, 60.0));
    let result = transform_bounding_box_axis_aligned(&bbox, &|p| Coord2(p.x() * 2.0, p.y() * 2.0));

    assert!(result.min() == Coord2(20.0, 40.0));
    assert!(result.max() == Coord2(100.0, 120.0));
}

#[test]
fn transform_bounding_box_90_degree_rotation() {
    // Rotating 90 degrees counter-clockwise: (x, y) -> (-y, x)
    let bbox = (Coord2(0.0, 0.0), Coord2(40.0, 20.0));
    let result = transform_bounding_box_axis_aligned(&bbox, &|p| Coord2(-p.y(), p.x()));

    assert!(result.min() == Coord2(-20.0, 0.0));
    assert!(result.max() == Coord2(0.0, 40.0));
}

#[test]
fn transform_bounding_box_45_degree_rotation() {
    // Rotating 45 degrees: a square stays a square but the bounding box grows
    let sqrt2_over_2 = (2.0_f64).sqrt() / 2.0;
    let bbox = (Coord2(-1.0, -1.0), Coord2(1.0, 1.0));
    let result = transform_bounding_box_axis_aligned(&bbox, &|p| {
        Coord2(
            p.x() * sqrt2_over_2 - p.y() * sqrt2_over_2,
            p.x() * sqrt2_over_2 + p.y() * sqrt2_over_2,
        )
    });

    let expected_extent = (2.0_f64).sqrt();
    assert!((result.min().x() - (-expected_extent)).abs() < 1e-10);
    assert!((result.min().y() - (-expected_extent)).abs() < 1e-10);
    assert!((result.max().x() - expected_extent).abs() < 1e-10);
    assert!((result.max().y() - expected_extent).abs() < 1e-10);
}

#[test]
fn transform_bounding_box_flip_x() {
    // Reflecting across the y-axis: (x, y) -> (-x, y)
    let bbox = (Coord2(10.0, 20.0), Coord2(50.0, 60.0));
    let result = transform_bounding_box_axis_aligned(&bbox, &|p| Coord2(-p.x(), p.y()));

    assert!(result.min() == Coord2(-50.0, 20.0));
    assert!(result.max() == Coord2(-10.0, 60.0));
}
