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

#[test]
fn merge_with_empty_into_empty() {
    // Degenerate case
    let mut a = RectRegion::from_bounds::<Coord2>(vec![]);
    let b     = RectRegion::from_bounds::<Coord2>(vec![]);

    a.merge_with(b);

    assert!(intercepts(&a, 5.0) == vec![], "{:?}", intercepts(&a, 5.0));
    assert!(a.slices().is_empty());
}

#[test]
fn merge_with_nonempty_into_empty() {
    // Short-circuits
    let mut a = RectRegion::from_bounds::<Coord2>(vec![]);
    let b     = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
    ]);

    a.merge_with(b);

    assert!(intercepts(&a, 5.0) == vec![0.0..10.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_empty_into_nonempty() {
    // Short-circuits
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
    ]);
    let b = RectRegion::from_bounds::<Coord2>(vec![]);

    a.merge_with(b);

    assert!(intercepts(&a, 5.0) == vec![0.0..10.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_non_overlapping_y() {
    // Two regions with completely separate y extents should both be present in the merged result
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 20.0), Coord2(10.0, 30.0)),
    ]);

    a.merge_with(b);

    assert!(intercepts(&a,  5.0) == vec![0.0..10.0], "{:?}", intercepts(&a, 5.0));
    assert!(intercepts(&a, 15.0) == vec![], "{:?}", intercepts(&a, 15.0));
    assert!(intercepts(&a, 25.0) == vec![0.0..10.0], "{:?}", intercepts(&a, 25.0));
}

#[test]
fn merge_with_same_y_different_x() {
    // Two regions covering the same y range but different, non-overlapping x ranges
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(20.0, 0.0), Coord2(30.0, 10.0)),
    ]);

    a.merge_with(b);

    assert!(intercepts(&a, 5.0) == vec![0.0..10.0, 20.0..30.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_same_y_overlapping_x() {
    // Two regions covering the same y range with overlapping x ranges should be unioned
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(15.0, 10.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 0.0), Coord2(25.0, 10.0)),
    ]);

    a.merge_with(b);

    assert!(intercepts(&a, 5.0) == vec![0.0..25.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_partially_overlapping_y() {
    // Regions with partially overlapping y extents — the overlapping band should union the x ranges
    // a covers y=0..20, b covers y=10..30; they overlap in y=10..20
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 20.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(20.0, 10.0), Coord2(30.0, 30.0)),
    ]);

    a.merge_with(b);

    // Only 'a' present above the overlap
    assert!(intercepts(&a,  5.0) == vec![0.0..10.0], "{:?}", intercepts(&a,  5.0));
    // Both present in the overlap band
    assert!(intercepts(&a, 15.0) == vec![0.0..10.0, 20.0..30.0], "{:?}", intercepts(&a, 15.0));
    // Only 'b' present below the overlap
    assert!(intercepts(&a, 25.0) == vec![20.0..30.0], "{:?}", intercepts(&a, 25.0));
}

#[test]
fn merge_with_updates_bounds() {
    // Bounds should reflect the union of both regions after merging
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(5.0, 0.0), Coord2(15.0, 10.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(20.0, 5.0), Coord2(40.0, 30.0)),
    ]);

    a.merge_with(b);

    let bounds: Bounds<Coord2> = a.bounds();
    assert!(bounds.min() == Coord2(5.0, 0.0), "min = {:?}", bounds.min());
    assert!(bounds.max() == Coord2(40.0, 30.0), "max = {:?}", bounds.max());
}

#[test]
fn merge_with_is_commutative() {
    // Merging is order-independent: a.merge_with(b) and b.merge_with(a) should produce equivalent results
    let a_bounds = vec![Bounds(Coord2(0.0, 0.0), Coord2(10.0, 20.0))];
    let b_bounds = vec![Bounds(Coord2(5.0, 10.0), Coord2(25.0, 30.0))];

    let mut ab = RectRegion::from_bounds(a_bounds.clone());
    ab.merge_with(RectRegion::from_bounds(b_bounds.clone()));

    let mut ba = RectRegion::from_bounds(b_bounds);
    ba.merge_with(RectRegion::from_bounds(a_bounds));

    for y in [5.0_f64, 15.0, 25.0] {
        assert!(intercepts(&ab, y) == intercepts(&ba, y), "y={}: ab={:?} ba={:?}", y, intercepts(&ab, y), intercepts(&ba, y));
    }
}

#[test]
fn merge_with_multiple_overlapping_regions() {
    // Many regions overlapping a single target region in the y axis
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(15.0, 100.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 10.0), Coord2(25.0, 15.0)),
        Bounds(Coord2(10.0, 20.0), Coord2(25.0, 25.0)),
        Bounds(Coord2(10.0, 30.0), Coord2(25.0, 35.0)),
    ]);

    a.merge_with(b);

    assert!(a.slices()[0].x_ranges() == &[0.0..15.0], "a.slices()[0].x_ranges() = {:?}", a.slices()[0].x_ranges());
    assert!(a.slices()[1].x_ranges() == &[0.0..25.0], "a.slices()[1].x_ranges() = {:?}", a.slices()[1].x_ranges());
    assert!(a.slices()[2].x_ranges() == &[0.0..15.0], "a.slices()[2].x_ranges() = {:?}", a.slices()[2].x_ranges());
    assert!(a.slices()[3].x_ranges() == &[0.0..25.0], "a.slices()[3].x_ranges() = {:?}", a.slices()[3].x_ranges());
    assert!(a.slices()[4].x_ranges() == &[0.0..15.0], "a.slices()[4].x_ranges() = {:?}", a.slices()[4].x_ranges());
    assert!(a.slices()[5].x_ranges() == &[0.0..25.0], "a.slices()[5].x_ranges() = {:?}", a.slices()[5].x_ranges());
    assert!(a.slices()[6].x_ranges() == &[0.0..15.0], "a.slices()[6].x_ranges() = {:?}", a.slices()[6].x_ranges());

    assert!(a.slices()[0].y_range() == (0.0..10.0), "a.slices()[0].y_range() = {:?}", a.slices()[0].y_range());
    assert!(a.slices()[1].y_range() == (10.0..15.0), "a.slices()[1].y_range() = {:?}", a.slices()[1].y_range());
    assert!(a.slices()[2].y_range() == (15.0..20.0), "a.slices()[2].y_range() = {:?}", a.slices()[2].y_range());
    assert!(a.slices()[3].y_range() == (20.0..25.0), "a.slices()[3].y_range() = {:?}", a.slices()[3].y_range());
    assert!(a.slices()[4].y_range() == (25.0..30.0), "a.slices()[4].y_range() = {:?}", a.slices()[4].y_range());
    assert!(a.slices()[5].y_range() == (30.0..35.0), "a.slices()[5].y_range() = {:?}", a.slices()[5].y_range());
    assert!(a.slices()[6].y_range() == (35.0..100.0), "a.slices()[6].y_range() = {:?}", a.slices()[6].y_range());

    assert!(a.slices().len() == 7);

    assert!(intercepts(&a, 11.0) == vec![0.0..25.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_multiple_overlapping_regions_last_leaves_bounds() {
    // Many regions overlapping a single target region in the y axis. Last one extends past the original region.
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(15.0, 100.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 10.0), Coord2(25.0, 15.0)),
        Bounds(Coord2(10.0, 20.0), Coord2(25.0, 25.0)),
        Bounds(Coord2(10.0, 30.0), Coord2(25.0, 105.0)),
    ]);

    a.merge_with(b);

    assert!(a.slices()[0].x_ranges() == &[0.0..15.0], "a.slices()[0].x_ranges() = {:?}", a.slices()[0].x_ranges());
    assert!(a.slices()[1].x_ranges() == &[0.0..25.0], "a.slices()[1].x_ranges() = {:?}", a.slices()[1].x_ranges());
    assert!(a.slices()[2].x_ranges() == &[0.0..15.0], "a.slices()[2].x_ranges() = {:?}", a.slices()[2].x_ranges());
    assert!(a.slices()[3].x_ranges() == &[0.0..25.0], "a.slices()[3].x_ranges() = {:?}", a.slices()[3].x_ranges());
    assert!(a.slices()[4].x_ranges() == &[0.0..15.0], "a.slices()[4].x_ranges() = {:?}", a.slices()[4].x_ranges());
    assert!(a.slices()[5].x_ranges() == &[0.0..25.0], "a.slices()[5].x_ranges() = {:?}", a.slices()[5].x_ranges());
    assert!(a.slices()[6].x_ranges() == &[10.0..25.0], "a.slices()[5].x_ranges() = {:?}", a.slices()[5].x_ranges());

    assert!(a.slices()[0].y_range() == (0.0..10.0), "a.slices()[0].y_range() = {:?}", a.slices()[0].y_range());
    assert!(a.slices()[1].y_range() == (10.0..15.0), "a.slices()[1].y_range() = {:?}", a.slices()[1].y_range());
    assert!(a.slices()[2].y_range() == (15.0..20.0), "a.slices()[2].y_range() = {:?}", a.slices()[2].y_range());
    assert!(a.slices()[3].y_range() == (20.0..25.0), "a.slices()[3].y_range() = {:?}", a.slices()[3].y_range());
    assert!(a.slices()[4].y_range() == (25.0..30.0), "a.slices()[4].y_range() = {:?}", a.slices()[4].y_range());
    assert!(a.slices()[5].y_range() == (30.0..100.0), "a.slices()[5].y_range() = {:?}", a.slices()[5].y_range());
    assert!(a.slices()[6].y_range() == (100.0..105.0), "a.slices()[6].y_range() = {:?}", a.slices()[6].y_range());

    assert!(a.slices().len() == 7, "Slices: {:?}", a.slices().iter().map(|slice| format!("y range: {:?}", slice.y_range())).collect::<Vec<_>>());

    assert!(intercepts(&a, 11.0) == vec![0.0..25.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_multiple_overlapping_regions_last_matches_bounds() {
    // Many regions overlapping a single target region in the y axis. Last one has a matching y coordinate
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(15.0, 100.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 10.0), Coord2(25.0, 15.0)),
        Bounds(Coord2(10.0, 20.0), Coord2(25.0, 25.0)),
        Bounds(Coord2(10.0, 30.0), Coord2(25.0, 100.0)),
    ]);

    a.merge_with(b);

    assert!(a.slices()[0].x_ranges() == &[0.0..15.0], "a.slices()[0].x_ranges() = {:?}", a.slices()[0].x_ranges());
    assert!(a.slices()[1].x_ranges() == &[0.0..25.0], "a.slices()[1].x_ranges() = {:?}", a.slices()[1].x_ranges());
    assert!(a.slices()[2].x_ranges() == &[0.0..15.0], "a.slices()[2].x_ranges() = {:?}", a.slices()[2].x_ranges());
    assert!(a.slices()[3].x_ranges() == &[0.0..25.0], "a.slices()[3].x_ranges() = {:?}", a.slices()[3].x_ranges());
    assert!(a.slices()[4].x_ranges() == &[0.0..15.0], "a.slices()[4].x_ranges() = {:?}", a.slices()[4].x_ranges());
    assert!(a.slices()[5].x_ranges() == &[0.0..25.0], "a.slices()[5].x_ranges() = {:?}", a.slices()[5].x_ranges());

    assert!(a.slices()[0].y_range() == (0.0..10.0), "a.slices()[0].y_range() = {:?}", a.slices()[0].y_range());
    assert!(a.slices()[1].y_range() == (10.0..15.0), "a.slices()[1].y_range() = {:?}", a.slices()[1].y_range());
    assert!(a.slices()[2].y_range() == (15.0..20.0), "a.slices()[2].y_range() = {:?}", a.slices()[2].y_range());
    assert!(a.slices()[3].y_range() == (20.0..25.0), "a.slices()[3].y_range() = {:?}", a.slices()[3].y_range());
    assert!(a.slices()[4].y_range() == (25.0..30.0), "a.slices()[4].y_range() = {:?}", a.slices()[4].y_range());
    assert!(a.slices()[5].y_range() == (30.0..100.0), "a.slices()[5].y_range() = {:?}", a.slices()[5].y_range());

    assert!(a.slices().len() == 6, "Slices: {:?}", a.slices().iter().map(|slice| format!("y range: {:?}", slice.y_range())).collect::<Vec<_>>());

    assert!(intercepts(&a, 11.0) == vec![0.0..25.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_multiple_overlapping_regions_first_matches_bounds() {
    // Many regions overlapping a single target region in the y axis
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(15.0, 100.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(10.0, 0.0), Coord2(25.0, 15.0)),
        Bounds(Coord2(10.0, 20.0), Coord2(25.0, 25.0)),
        Bounds(Coord2(10.0, 30.0), Coord2(25.0, 35.0)),
    ]);

    a.merge_with(b);

    assert!(a.slices()[0].x_ranges() == &[0.0..25.0], "a.slices()[0].x_ranges() = {:?}", a.slices()[0].x_ranges());
    assert!(a.slices()[1].x_ranges() == &[0.0..15.0], "a.slices()[1].x_ranges() = {:?}", a.slices()[1].x_ranges());
    assert!(a.slices()[2].x_ranges() == &[0.0..25.0], "a.slices()[2].x_ranges() = {:?}", a.slices()[2].x_ranges());
    assert!(a.slices()[3].x_ranges() == &[0.0..15.0], "a.slices()[3].x_ranges() = {:?}", a.slices()[3].x_ranges());
    assert!(a.slices()[4].x_ranges() == &[0.0..25.0], "a.slices()[4].x_ranges() = {:?}", a.slices()[4].x_ranges());
    assert!(a.slices()[5].x_ranges() == &[0.0..15.0], "a.slices()[5].x_ranges() = {:?}", a.slices()[5].x_ranges());

    assert!(a.slices()[0].y_range() == (0.0..15.0), "a.slices()[0].y_range() = {:?}", a.slices()[0].y_range());
    assert!(a.slices()[1].y_range() == (15.0..20.0), "a.slices()[1].y_range() = {:?}", a.slices()[1].y_range());
    assert!(a.slices()[2].y_range() == (20.0..25.0), "a.slices()[2].y_range() = {:?}", a.slices()[2].y_range());
    assert!(a.slices()[3].y_range() == (25.0..30.0), "a.slices()[3].y_range() = {:?}", a.slices()[3].y_range());
    assert!(a.slices()[4].y_range() == (30.0..35.0), "a.slices()[4].y_range() = {:?}", a.slices()[4].y_range());
    assert!(a.slices()[5].y_range() == (35.0..100.0), "a.slices()[5].y_range() = {:?}", a.slices()[5].y_range());

    assert!(a.slices().len() == 6);

    assert!(intercepts(&a, 11.0) == vec![0.0..25.0], "{:?}", intercepts(&a, 5.0));
}

#[test]
fn merge_with_adjacent_y_same_x_combines_slices() {
    // Two regions that are adjacent in y (touching, not overlapping) with identical x ranges.
    let mut a = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 0.0), Coord2(10.0, 10.0)),
    ]);
    let b = RectRegion::from_bounds(vec![
        Bounds(Coord2(0.0, 10.0), Coord2(10.0, 20.0)),
    ]);

    a.merge_with(b);

    // Intercepts should be correct regardless
    assert!(intercepts(&a, 5.0) == vec![0.0..10.0], "{:?}", intercepts(&a,  5.0));
    assert!(intercepts(&a, 15.0) == vec![0.0..10.0], "{:?}", intercepts(&a, 15.0));

    // TODO: adjacent slices with equal x_ranges should be collapsed into one
    assert!(a.slices().len() == 1, "Expected 1 combined slice, got {} slices", a.slices().len());
}
