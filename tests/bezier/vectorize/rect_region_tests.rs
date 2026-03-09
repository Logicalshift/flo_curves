use flo_curves::bezier::vectorize::*;
use flo_curves::geo::*;

use std::ops::Range;

fn intercepts(region: &RectRegion, y: f64) -> Vec<Range<f64>> {
    region.intercepts_on_line(y).into_iter().collect()
}

#[test]
fn empty_region() {
    let region = RectRegion::from_bounds::<Coord2>(vec![]);

    assert!(intercepts(&region, 0.0) == vec![], "{:?}", intercepts(&region, 0.0));
    assert!(intercepts(&region, 5.0) == vec![], "{:?}", intercepts(&region, 5.0));
}

#[test]
fn single_rect_bounds() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 20.0), Coord2(50.0, 80.0)),
    ]);

    let bounds: Bounds<Coord2> = region.bounds();
    assert!(bounds.min() == Coord2(10.0, 20.0), "min = {:?}", bounds.min());
    assert!(bounds.max() == Coord2(50.0, 80.0), "max = {:?}", bounds.max());
}

#[test]
fn single_rect_intercepts_inside() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 20.0), Coord2(50.0, 80.0)),
    ]);

    // A y value well inside the rect
    assert!(intercepts(&region, 50.0) == vec![10.0..50.0], "{:?}", intercepts(&region, 50.0));
}

#[test]
fn single_rect_intercepts_outside_above() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 20.0), Coord2(50.0, 80.0)),
    ]);

    assert!(intercepts(&region, 10.0) == vec![], "{:?}", intercepts(&region, 10.0));
}

#[test]
fn single_rect_intercepts_outside_below() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 20.0), Coord2(50.0, 80.0)),
    ]);

    assert!(intercepts(&region, 90.0) == vec![], "{:?}", intercepts(&region, 90.0));
}

#[test]
fn two_non_overlapping_rects_same_y() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
        Bounds(Coord2(20.0, 0.0), Coord2(30.0, 10.0)),
    ]);

    assert!(intercepts(&region, 5.0) == vec![0.0..10.0, 20.0..30.0], "{:?}", intercepts(&region, 5.0));
}

#[test]
fn two_overlapping_rects_same_y() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(15.0, 10.0)),
        Bounds(Coord2(10.0, 0.0), Coord2(30.0, 10.0)),
    ]);

    // Overlapping ranges should be merged
    assert!(intercepts(&region, 5.0) == vec![0.0..30.0], "{:?}", intercepts(&region, 5.0));
}

#[test]
fn two_rects_different_y_extents() {
    // Tall rect and short rect starting at the same y
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 20.0)),
        Bounds(Coord2(20.0, 0.0), Coord2(30.0, 10.0)),
    ]);

    // In the range where both are present
    assert!(intercepts(&region, 5.0) == vec![0.0..10.0, 20.0..30.0], "{:?}", intercepts(&region, 5.0));

    // After the short rect ends, only the tall rect remains
    assert!(intercepts(&region, 15.0) == vec![0.0..10.0], "{:?}", intercepts(&region, 15.0));

    // Beyond both rects
    assert!(intercepts(&region, 25.0) == vec![], "{:?}", intercepts(&region, 25.0));
}

#[test]
fn two_rects_stacked_vertically() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
        Bounds(Coord2(0.0, 20.0), Coord2(10.0, 30.0)),
    ]);

    assert!(intercepts(&region, 5.0) == vec![0.0..10.0], "{:?}", intercepts(&region, 5.0));
    assert!(intercepts(&region, 15.0) == vec![], "{:?}", intercepts(&region, 15.0));
    assert!(intercepts(&region, 25.0) == vec![0.0..10.0], "{:?}", intercepts(&region, 25.0));
}

#[test]
fn three_rects_with_gap() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
        Bounds(Coord2(20.0, 0.0), Coord2(30.0, 10.0)),
        Bounds(Coord2(40.0, 0.0), Coord2(50.0, 10.0)),
    ]);

    assert!(intercepts(&region, 5.0) == vec![0.0..10.0, 20.0..30.0, 40.0..50.0], "{:?}", intercepts(&region, 5.0));
}

#[test]
fn three_rects_merged_into_one() {
    // Each rect overlaps with the next
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(15.0, 10.0)),
        Bounds(Coord2(10.0, 0.0), Coord2(25.0, 10.0)),
        Bounds(Coord2(20.0, 0.0), Coord2(35.0, 10.0)),
    ]);

    assert!(intercepts(&region, 5.0) == vec![0.0..35.0], "{:?}", intercepts(&region, 5.0));
}

#[test]
fn single_rect_slices() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(5.0, 10.0), Coord2(25.0, 40.0)),
    ]);

    let slices = region.slices();
    assert!(slices.len() == 1, "Expected 1 slice, got {}", slices.len());
    assert!(slices[0].y_range() == (10.0..40.0), "y_range = {:?}", slices[0].y_range());
    assert!(slices[0].x_ranges() == &[5.0..25.0], "x_ranges = {:?}", slices[0].x_ranges());
}

#[test]
fn contour_size_from_region() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(100.0, 50.0)),
    ]);

    let size = region.contour_size();
    assert!(size == ContourSize(100, 50), "size = {:?}", size);
}

#[test]
fn contour_size_non_integer() {
    let region = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.5, 20.3)),
    ]);

    let size = region.contour_size();
    // Should ceil the values
    assert!(size == ContourSize(11, 21), "size = {:?}", size);
}
