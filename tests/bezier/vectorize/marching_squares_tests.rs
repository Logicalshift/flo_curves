use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

use itertools::*;

///
/// Creates a slow but accurate signed distance field from a path
///
fn slow_distance_field_from_path(path: Vec<SimpleBezierPath>) -> F64SampledDistanceField {
    // Use PathContour to determine if a point is inside or not, and also to generate an offset for the path
    let (contour, offset) = PathContour::center_path(path.clone());

    // Create the distance field by slowly measuring the path at every point
    let distance_field = create_distance_field(|x, y| {
        let is_inside = contour_point_is_inside(&contour, ContourPosition(x as _, y as _));
        let distance  = path.iter()
            .map(|subpath| path_closest_point(subpath, &(Coord2(x, y)+offset)))
            .map(|(_, _, distance, _)| distance)
            .reduce(|a, b| {
                if a < b { a } else { b }
            })
            .unwrap()
            .abs();

        if distance.is_nan() {
            panic!("NaN distance");
        }

        if is_inside {
            -distance
        } else {
            distance
        }
    }, contour.contour_size());

    let width   = contour.contour_size().width();
    let height  = contour.contour_size().height();

    for y in 0..height {
        for x in 0..width {
            let distance = distance_field.distance_at_point(ContourPosition(x, y));

            if distance.is_nan() {
                print!("/");
            }

            if distance <= 0.0 {
                print!("#");
            } else if distance < 1.0 {
                print!("*");
            } else if distance < 4.0 {
                print!("!");
            } else if distance < 8.0 {
                print!(".");
            } else {
                print!(" ");
            }
        }

        println!();
    }

    println!();

    for y in 0..height {
        let intercepts  = distance_field.as_contour().rounded_intercepts_on_line(y as _);
        let mut line    = vec![false; width];

        for range in intercepts {
            for x in range {
                line[x] = true;
            }
        }

        for x in 0..width {
            if line[x] {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }

    println!();

    distance_field
}

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
fn circle_edges_from_contours() {
    // Create a contour containing a circle in the middle
    let size    = 80;
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

    // Fetch the contour coordinates (the edges, counting from 1,1 in the source)
    let circle = circle[0].iter().map(|edge| edge.to_contour_coords(ContourSize(size, size))).collect::<Vec<_>>();

    // Every edge should lie on a transition
    let mut all_edges = true;
    let mut is_edge = vec![false; size*size];
    for (from, to) in circle {
        let from    = ContourPosition(from.0-1, from.1-1);
        let to      = ContourPosition(to.0-1, to.1-1);

        is_edge[from.0 + from.1 * size] = true;
        is_edge[to.0 + to.1 * size] = true;

        let from_inside = contour.point_is_inside(from);
        let to_inside   = contour.point_is_inside(to);

        if (from_inside && to_inside)
            || (!from_inside && !to_inside) {
            all_edges = false;
            println!("Not an edge {:?} {:?} ({:?}-{:?})", from, to, from_inside, to_inside);
        }
    }

    let mut circle = String::new();
    for (idx, p) in contour.1.iter().enumerate() {
        if (idx%size) == 0 && idx != 0 { circle.push('\n'); }
        if *p && is_edge[idx] {
            circle.push('E');
        } else if *p {
            circle.push('x');
        } else if is_edge[idx] {
            circle.push('o');
        } else {
            circle.push('.');
        }
    }

    println!("{}", circle);

    assert!(all_edges, "Not all edges are circle edges");
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
    let circle = trace_paths_from_samples::<SimpleBezierPath>(&contour, 1.5);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);

    // Allow 1.5px of error (between the fitting algorithm and the sampled circle itself)
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
    assert!(max_error <= 1.5, "Max error {:?} > 1.5. Path generated was {:?}", max_error, circle);
}

#[test]
fn circle_path_from_distance_field() {
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

            let distance_to_center = ((x*x) + (y*y)).sqrt();
            let distance_to_circle = distance_to_center - radius;

            distance_to_circle
        })
        .collect();
    let distance_field = F64SampledDistanceField(ContourSize(size, size), contour);

    // Trace the samples to generate a vector
    let circle = trace_paths_from_distance_field::<SimpleBezierPath>(&distance_field, 0.1);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);
    assert!(circle[0].to_curves::<Curve<_>>().len() < 10, "Path has {} curves", circle[0].to_curves::<Curve<_>>().len());

    // Allow 0.1px of error (distance fields provide much better estimates of where the edge really is)
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
    assert!(max_error <= 0.2, "Max error {:?} > 0.2. Path generated was {:?}", max_error, circle);
}

#[test]
fn chisel_from_contours() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();
    let chisel_field = slow_distance_field_from_path(vec![chisel.clone()]);

    let chisel_again    = trace_paths_from_samples::<SimpleBezierPath>(&chisel_field, 1.0);
    assert!(chisel_again.len() == 1, "Made {} paths ({:?})", chisel_again.len(), chisel_again);
    let no_nans         = chisel_again.into_iter().map(|subpath| subpath.map_points::<SimpleBezierPath>(|point| {
        assert!(!point.x().is_nan() && !point.y().is_nan());
        point
    })).collect::<Vec<_>>();

    for path in no_nans {
        assert!(path.points().count() < 20, "Generated {} points", path.points().count());
    }
}

#[test]
fn chisel_no_very_close_points() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();
    let chisel_field = slow_distance_field_from_path(vec![chisel.clone()]);

    let chisel_points = trace_contours_from_distance_field::<Coord2>(&chisel_field);
    assert!(chisel_points.len() > 0);

    for subpath in chisel_points {
        for (p1, p2) in subpath.iter().tuple_windows() {
            let distance = p1.distance_to(p2);

            assert!(distance > 0.1, "{:?} {:?} are very close", p1, p2);
        }
    }
}

#[test]
fn chisel_from_distance_field() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();
    let chisel_field = slow_distance_field_from_path(vec![chisel.clone()]);

    let chisel_again    = trace_paths_from_distance_field::<SimpleBezierPath>(&chisel_field, 0.1);
    assert!(chisel_again.len() == 1, "Made {} paths ({:?})", chisel_again.len(), chisel_again);
    let no_nans         = chisel_again.into_iter().map(|subpath| subpath.map_points::<SimpleBezierPath>(|point| {
        assert!(!point.x().is_nan() && !point.y().is_nan());
        point
    })).collect::<Vec<_>>();

    for path in no_nans {
        assert!(path.points().count() < 20, "Generated {} points", path.points().count());
    }
}
