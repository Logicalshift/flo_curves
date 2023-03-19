use flo_curves::geo::*;
use flo_curves::bezier::vectorize::*;

#[test]
fn single_sample_loop() {
    // Single 'inside' sample, should produce 4 edge cells
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            0, 0, 0,
            0, 1, 0,
            0, 0, 0,
        ]);

    let loops = trace_contours_from_samples::<Coord2>(&contour);

    assert!(loops.len() == 1, "{:?}", loops);
}

#[test]
fn double_loops() {
    // Two loops at the edge
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            1, 0, 0,
            0, 0, 0,
            0, 0, 1,
        ]);

    let loops = trace_contours_from_samples::<Coord2>(&contour);

    assert!(loops.len() == 2, "{:?}", loops);
}

#[test]
fn filled() {
    // One big square, filling the entire space
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

    let loops = trace_contours_from_samples::<Coord2>(&contour);

    assert!(loops.len() == 1, "{:?}", loops);
}

#[test]
fn filled_without_edges() {
    // Square with a border
    let contour = U8SampledContour(ContourSize(5, 5), vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 0, 0, 0, 0,
        ]);

    let loops = trace_contours_from_samples::<Coord2>(&contour);

    assert!(loops.len() == 1, "{:?}", loops);
}

#[test]
fn perimeter() {
    // Two loops, one inner, one outer
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            1, 1, 1,
            1, 0, 1,
            1, 1, 1,
        ]);

    let loops = trace_contours_from_samples::<Coord2>(&contour);

    assert!(loops.len() == 2, "{:?}", loops);
}

#[test]
fn triple_loops() {
    // This is ambiguous in terms of how it can be interpreted, we happen to choose three loops here (with field weights)
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            1, 0, 0,
            0, 1, 0,
            0, 0, 1,
        ]);

    let loops = trace_contours_from_samples::<Coord2>(&contour);

    assert!(loops.len() == 3, "{:?}", loops);
}
