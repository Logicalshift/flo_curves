use flo_curves::bezier::vectorize::*;

use smallvec::*;

use std::ops::{Range};

struct RectContour {
    size:   ContourSize,
    width:  Range<f64>
}

impl SampledContour for RectContour {
    fn contour_size(&self) -> ContourSize {
        self.size
    }

    fn intercepts_on_line(&self, _y: f64) -> smallvec::SmallVec<[Range<f64>; 4]> {
        smallvec![self.width.clone()]
    }
}

#[test]
fn add_basic_contour() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add a rectangle to it
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![32.0..96.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(96.0).into_iter().collect::<Vec<_>>() == vec![], "Line 96 = {:?}", contour.intercepts_on_line(96.0));
}

#[test]
fn add_non_overlapping_contours() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add two non-overlapping rectangles to it
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (128.0, 32.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![32.0..96.0, 128.0..192.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(96.0).into_iter().collect::<Vec<_>>() == vec![], "Line 96 = {:?}", contour.intercepts_on_line(96.0));
}

#[test]
fn merge_with_overlap_1() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add 3 rectangles to it (2 non overlapping, and then 1 that joins them together)
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (128.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (80.0, 32.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![32.0..192.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(96.0).into_iter().collect::<Vec<_>>() == vec![], "Line 96 = {:?}", contour.intercepts_on_line(96.0));
}

#[test]
fn merge_with_overlap_2() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add 3 rectangles to it (2 non overlapping, and then 1 that joins them together)
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (80.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (128.0, 32.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![32.0..192.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(96.0).into_iter().collect::<Vec<_>>() == vec![], "Line 96 = {:?}", contour.intercepts_on_line(96.0));
}

#[test]
fn merge_with_overlap_3() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add 3 rectangles to it (2 non overlapping, and then 1 that joins them together)
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (80.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (128.0, 32.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![32.0..192.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(96.0).into_iter().collect::<Vec<_>>() == vec![], "Line 96 = {:?}", contour.intercepts_on_line(96.0));
}

#[test]
fn merge_with_overlap_4() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add 3 rectangles to it (2 non overlapping, and then 1 that joins them together)
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (80.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (128.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![32.0..192.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(96.0).into_iter().collect::<Vec<_>>() == vec![], "Line 96 = {:?}", contour.intercepts_on_line(96.0));
}

#[test]
fn merge_total_overlap_1() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add a rectangle to it and then completely overlap it with another rectangle
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));
    contour.add_contour(&RectContour { size: ContourSize(128, 128), width: 0.0..128.0 }, (0.0, 0.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![0.0..128.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![0.0..128.0], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(128.0).into_iter().collect::<Vec<_>>() == vec![], "Line 128 = {:?}", contour.intercepts_on_line(96.0));
}

#[test]
fn merge_total_overlap_2() {
    // Start with an empty contour
    let mut contour = RangeContour::default();

    // Add a rectangle to it and then another rectangle that is entirely overlapped by the first one
    contour.add_contour(&RectContour { size: ContourSize(128, 128), width: 0.0..128.0 }, (0.0, 0.0));
    contour.add_contour(&RectContour { size: ContourSize(64, 64), width: 0.0..64.0 }, (32.0, 32.0));

    // Check some intercepts (middle, bottom, top)
    assert!(contour.intercepts_on_line(48.0).into_iter().collect::<Vec<_>>() == vec![0.0..128.0], "Line 48 = {:?}", contour.intercepts_on_line(48.0));
    assert!(contour.intercepts_on_line(16.0).into_iter().collect::<Vec<_>>() == vec![0.0..128.0], "Line 16 = {:?}", contour.intercepts_on_line(16.0));
    assert!(contour.intercepts_on_line(128.0).into_iter().collect::<Vec<_>>() == vec![], "Line 128 = {:?}", contour.intercepts_on_line(96.0));
}
