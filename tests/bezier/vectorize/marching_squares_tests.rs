use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;

#[test]
fn single_sample_loop() {
    // Single 'inside' sample, should produce 4 edge cells
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            0, 0, 0,
            0, 1, 0,
            0, 0, 0,
        ]);

    let loops = trace_contours_from_samples(&contour);

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

    let loops = trace_contours_from_samples(&contour);

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

    let loops = trace_contours_from_samples(&contour);

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

    let loops = trace_contours_from_samples(&contour);

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

    let loops = trace_contours_from_samples(&contour);

    assert!(loops.len() == 2, "{:?}", loops);
}

#[test]
fn perimeter_without_edges() {
    // Two loops, one inner, one outer
    let contour = U8SampledContour(ContourSize(5, 5), vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            0, 1, 0, 1, 0,
            0, 1, 1, 1, 0,
            0, 0, 0, 0, 0,
        ]);

    let loops = trace_contours_from_samples(&contour);

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

    let loops = trace_contours_from_samples(&contour);

    assert!(loops.len() == 3, "{:?}", loops);
}

#[test]
fn circle_points_from_contours() {
    // Create a contour containing a circle in the middle
    let size    = 100;
    let radius  = 30.0;
    let center  = (size/2) as f64;
    let contour = (0..(size*size)).into_iter()
        .map(|pos| {
            let x = (pos % size) as f64;
            let y = (pos / size) as f64;
            let x = x - center;
            let y = y - center;

            let r_squared = (x*x) + (y*y);
            if r_squared > radius * radius {
                false
            } else {
                true
            }
        })
        .collect();
    let contour = BoolSampledContour(ContourSize(size, size), contour);

    // Trace the samples to generate a vector
    let circle = trace_contours_from_samples(&contour);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);

    let circle = circle[0].iter().map(|edge| edge.to_coords::<Coord2>(ContourSize(size, size))).collect::<Vec<_>>();

    // Allow 1.5px of error
    let mut max_error = 0.0;

    for point in circle.iter() {
        let distance    = point.distance_to(&Coord2(center+1.0, center+1.0));
        let offset      = (distance-radius).abs();

        max_error = f64::max(max_error, offset);
    }

    assert!(max_error <= 1.5, "Max error {:?} > 1.5. Path generated was {:?}", max_error, circle);

    // Last point in the circle should be the same as the first point (because it forms a loop)
    assert!(circle[0] == circle[circle.len()-1]);
}

#[test]
fn circle_path_from_contours() {
    // Create a contour containing a circle in the middle
    let size    = 100;
    let radius  = 30.0;
    let center  = (size/2) as f64;
    let contour = (0..(size*size)).into_iter()
        .map(|pos| {
            let x = (pos % size) as f64;
            let y = (pos / size) as f64;
            let x = x - center;
            let y = y - center;

            let r_squared = (x*x) + (y*y);
            if r_squared > radius * radius {
                false
            } else {
                true
            }
        })
        .collect();
    let contour = BoolSampledContour(ContourSize(size, size), contour);

    // Trace the samples to generate a vector
    let circle = trace_paths_from_samples::<SimpleBezierPath>(&contour);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);

    // Allow 2.0px of error (between the fitting algorithm and the sampled circle itself)
    let mut max_error = 0.0;

    for curve in circle[0].to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t           = (t as f64)/100.0;
            let point       = curve.point_at_pos(t);
            let distance    = point.distance_to(&Coord2(center+1.0, center+1.0));
            let offset      = (distance-radius).abs();

            max_error = f64::max(max_error, offset);
        }
    }

    // The error here is semi-random due to the hash table used to store the edge graph
    assert!(max_error <= 2.0, "Max error {:?} > 2.0. Path generated was {:?}", max_error, circle);
}
