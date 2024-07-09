use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

use itertools::*;

use std::ops::{Range};

fn distance_field_as_string(field: &impl SampledSignedDistanceField) -> String {
    let mut result = String::new();

    for y in 0..field.field_size().0 {
        for x in 0..field.field_size().1 {
            let distance = field.distance_at_point(ContourPosition(x, y));

            let symbol = if distance <= 0.0 {
                '#'
            } else if distance < 1.0 {
                '@'
            } else if distance < 2.0 {
                '*'
            } else if distance < 3.0 {
                '|'
            } else if distance < 4.0 {
                '-'
            } else if distance < 10.0 {
                '.'
            } else {
                ' '
            };

            result.push(symbol);
        }

        result.push('\n');
    }

    result
}

fn circle_intercepts(center_x: f64, center_y: f64, radius: f64) -> impl Fn(f64) -> Vec<Range<f64>> {
    move |xpos| {
        let x    = xpos - center_x;

        if x.abs() <= radius {
            let intercept = ((radius*radius) - (x*x)).sqrt();
            let min_y     = center_y - intercept;
            let max_y     = center_y + intercept;

            vec![min_y..max_y]
        } else {
            vec![]
        }
    }
}

#[test]
pub fn circular_path_outside_1() {
    let width    = 256;
    let height   = 256;

    let center_x = 128.0;
    let center_y = 128.0;
    let radius   = 80.0;

    // Create a circular distance field
    let circle = MarchingParabolaDistanceField::from_intercepts(width, height, circle_intercepts(center_y, center_x, radius), circle_intercepts(center_x, center_y, radius));

    // Check the outside distances are accurate to within 1 pixel
    let mut num_greater_than_1   = 0;
    let mut num_greater_than_0_5 = 0;
    let mut num_greater_than_0_1 = 0;

    for y in 0..height {
        for x in 0..width {
            let val = circle.distance_at_point(ContourPosition(x, y));

            let x = x as f64;
            let y = y as f64;
            let x = x - center_x;
            let y = y - center_y;
            let to_center = (x*x + y*y).sqrt();

            if to_center > radius {
                let expected = to_center - radius;

                if (expected - val).abs() >= 1.0 || val.is_nan() {
                    num_greater_than_1 += 1;
                }

                if (expected - val).abs() >= 0.5 {
                    num_greater_than_0_5 += 1;
                }

                if (expected - val).abs() >= 0.1 {
                    num_greater_than_0_1 += 1;
                }
            }
        }
    }

    println!("> 0.1 = {}", num_greater_than_0_1);
    println!("> 0.5 = {}", num_greater_than_0_5);
    println!("> 1.0 = {}", num_greater_than_1);
    assert!(num_greater_than_1 == 0);
    assert!(num_greater_than_0_5 == 0);
}

#[test]
pub fn circular_path_inside_1() {
    let width    = 256;
    let height   = 256;

    let center_x = 128.0;
    let center_y = 128.0;
    let radius   = 80.0;

    // Create a circular distance field
    let circle = MarchingParabolaDistanceField::from_intercepts(width, height, circle_intercepts(center_y, center_x, radius), circle_intercepts(center_x, center_y, radius));

    // Check the outside distances are accurate to within 1 pixel
    let mut num_greater_than_1   = 0;
    let mut num_greater_than_0_5 = 0;
    let mut num_greater_than_0_1 = 0;
    let mut num_wrong_sign       = 0;

    for y in 0..height {
        for x in 0..width {
            let val = circle.distance_at_point(ContourPosition(x, y));

            let x = x as f64;
            let y = y as f64;
            let x = x - center_x;
            let y = y - center_y;
            let to_center = (x*x + y*y).sqrt();

            if to_center < radius {
                let expected = (to_center - radius).abs();

                if val > 0.0 || (val == 0.0 && expected != 0.0) {
                    num_wrong_sign += 1;
                }

                if (expected - val.abs()).abs() >= 1.0 || val.is_nan() {
                    num_greater_than_1 += 1;
                }

                if (expected - val.abs()).abs() >= 0.5 {
                    num_greater_than_0_5 += 1;
                }

                if (expected - val.abs()).abs() >= 0.1 {
                    num_greater_than_0_1 += 1;
                }
            }
        }
    }

    println!("> 0.1 = {}", num_greater_than_0_1);
    println!("> 0.5 = {}", num_greater_than_0_5);
    println!("> 1.0 = {}", num_greater_than_1);
    println!("wrong sign = {}", num_wrong_sign);
    assert!(num_greater_than_1 == 0);
    assert!(num_greater_than_0_5 == 0);
    assert!(num_wrong_sign == 0);
}

#[test]
pub fn rectangle_path_1() {
    // Single rectangle path
    let rectangle = MarchingParabolaDistanceField::from_intercepts(128, 128, 
        |x| {
            if x >= 8.0 && x < 60.0 {
                vec![2.0..40.0]
            } else {
                vec![]
            }
        }, 
        |y| {
            if y >= 2.0 && y <= 40.0 {
                vec![8.0..60.0]
            } else {
                vec![]
            }
        });

    assert!((rectangle.distance_at_point(ContourPosition(50, 41)) - 1.0).abs() < 0.01, "{} != 1.0", rectangle.distance_at_point(ContourPosition(20, 41)));
    assert!((rectangle.distance_at_point(ContourPosition(50, 42)) - 2.0).abs() < 0.01, "{} != 2.0", rectangle.distance_at_point(ContourPosition(20, 42)));
    assert!((rectangle.distance_at_point(ContourPosition(50, 2)) - 0.0).abs() < 0.01, "{} != 0.0", rectangle.distance_at_point(ContourPosition(20, 20)));
}

#[test]
pub fn rectangle_path_2() {
    // Multiple rectangle paths
    let rectangle = MarchingParabolaDistanceField::from_intercepts(128, 128, 
        |x| {
            if x >= 8.0 && x < 60.0 {
                vec![2.0..40.0, 80.0..90.0]
            } else {
                vec![]
            }
        }, 
        |y| {
            if (y >= 2.0 && y < 40.0) || (y >= 80.0 && y < 90.0) {
                vec![8.0..60.0]
            } else {
                vec![]
            }
        });

    assert!((rectangle.distance_at_point(ContourPosition(50, 41)) - 1.0).abs() < 0.01, "{} != 1.0", rectangle.distance_at_point(ContourPosition(20, 41)));
    assert!((rectangle.distance_at_point(ContourPosition(50, 42)) - 2.0).abs() < 0.01, "{} != 2.0", rectangle.distance_at_point(ContourPosition(20, 42)));
    assert!((rectangle.distance_at_point(ContourPosition(50, 2)) - 0.0).abs() < 0.01, "{} != 0.0", rectangle.distance_at_point(ContourPosition(20, 20)));
    assert!((rectangle.distance_at_point(ContourPosition(50, 79)) - 1.0).abs() < 0.01, "{} != 1.0", rectangle.distance_at_point(ContourPosition(20, 79)));
    assert!((rectangle.distance_at_point(ContourPosition(50, 92)) - 2.0).abs() < 0.01, "{} != 2.0", rectangle.distance_at_point(ContourPosition(20, 92)));
}

#[test]
pub fn multiple_intercepts_in_one_pixel_1() {
    // Bunch of very thin slivers over single pixels
    let x_ranges = vec![1.0..1.1, 1.2..1.3, 1.4..1.5, 2.0..40.0, 41.9..42.0, 42.1..42.2];

    let weird_rectangle = MarchingParabolaDistanceField::from_intercepts(128, 128, 
        |x| {
            if x >= 8.0 && x < 60.0 {
                x_ranges.clone()
            } else {
                vec![]
            }
        }, 
        |y| {
            if x_ranges.iter().any(|range| range.contains(&y)) {
                vec![8.0..60.0]
            } else {
                vec![]
            }
        });

    assert!((weird_rectangle.distance_at_point(ContourPosition(20, 43)) - 0.8).abs() < 0.01, "{} != 0.8", weird_rectangle.distance_at_point(ContourPosition(20, 43)));
}

#[test]
fn trace_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_field    = MarchingParabolaDistanceField::from_path_region(0.0, 0.0, 1000, 1000, vec![circle_path.clone()]);
    let traced_circle   = trace_paths_from_distance_field::<SimpleBezierPath>(&circle_field, 0.1);

    debug_assert!(traced_circle.len() == 1);

    // Test against the ideal circle
    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(501.0, 501.0));

            debug_assert!((distance - radius) < 0.2, "Point #{} at distance {:?}", num_points, distance);
        }
    }

    // Test against the actual path
    let mut num_points = 0;
    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);
            let point       = point - Coord2(1.0, 1.0);

            let nearest_distance = circle_path.to_curves::<Curve<_>>().into_iter()
                .map(|curve| curve.distance_to(&point))
                .reduce(|d1, d2| d1.min(d2))
                .unwrap();

            debug_assert!(nearest_distance.abs() < 0.1, "Point #{} at distance {:?}", num_points, nearest_distance);
        }
    }

    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 32, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());
}

#[test]
fn trace_chisel_contours() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();

    let (chisel_field, ox, oy)  = MarchingParabolaDistanceField::from_path(vec![chisel.clone()]);
    let traced_chisel           = trace_contours_from_distance_field::<Coord2>(&chisel_field);
    let offset                  = Coord2(ox, oy);

    debug_assert!(traced_chisel.len() == 1, "Generated {} paths in the result\n{}", traced_chisel.len(), distance_field_as_string(&chisel_field));

    let mut num_points  = 0;
    let mut max_error   = 0.0f64;
    let mut total_error = 0.0f64;
    let mut error_count = 0;
    for point in traced_chisel[0].iter().copied() {
        num_points += 1;

        let point = point + offset - Coord2(1.0, 1.0);

        let nearest_distance = chisel.to_curves::<Curve<_>>().into_iter()
            .map(|curve| curve.distance_to(&point))
            .reduce(|d1, d2| d1.min(d2))
            .unwrap()
            .abs();
        max_error   = max_error.max(nearest_distance);
        total_error += nearest_distance;

        if nearest_distance > 0.1 {
            error_count += 1;
        }
    }

    let avg_error = total_error / (num_points as f64);

    debug_assert!(max_error < 0.1, "Max error was {} (average {}, num >0.1 {}/{})", max_error, avg_error, error_count, num_points);
}

#[test]
fn chisel_no_very_close_points() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();
    let (chisel_field, _, _) = MarchingParabolaDistanceField::from_path(vec![chisel.clone()]);

    let chisel_points = trace_contours_from_distance_field::<Coord2>(&chisel_field);
    assert!(chisel_points.len() > 0);

    for subpath in chisel_points {
        for (p1, p2) in subpath.iter().tuple_windows() {
            let distance = p1.distance_to(p2);

            assert!(distance > 0.1, "{:?} {:?} are very close", p1, p2);
            assert!(distance < 2.0, "{:?} {:?} are very far apart", p1, p2);
        }
    }
}

#[test]
fn trace_chisel_paths() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();

    let (chisel_field, ox, oy)  = MarchingParabolaDistanceField::from_path(vec![chisel.clone()]);
    let traced_chisel           = trace_paths_from_distance_field::<SimpleBezierPath>(&chisel_field, 0.1);
    let offset                  = Coord2(ox, oy);

    debug_assert!(traced_chisel.len() == 1, "Generated {} paths in the result\n{}", traced_chisel.len(), distance_field_as_string(&chisel_field));

    let mut num_points  = 0;
    let mut max_error   = 0.0f64;
    let mut total_error = 0.0f64;
    for curve in traced_chisel[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);
            let point       = point + offset - Coord2(1.0, 1.0);

            let nearest_distance = chisel.to_curves::<Curve<_>>().into_iter()
                .map(|curve| curve.distance_to(&point))
                .reduce(|d1, d2| d1.min(d2))
                .unwrap();
            max_error   = max_error.max(nearest_distance);
            total_error += nearest_distance;

            debug_assert!(nearest_distance.abs() < 0.4, "Point #{} at distance {:?}\n{}", num_points, nearest_distance, distance_field_as_string(&chisel_field));
        }
    }

    let avg_error = total_error / (num_points as f64);

    debug_assert!(max_error < 0.4, "Max error was {:?} (average {:?})\n{}", max_error, avg_error, distance_field_as_string(&chisel_field));
    debug_assert!(traced_chisel[0].to_curves::<Curve<_>>().len() < 16, "Result has {} curves", traced_chisel[0].to_curves::<Curve<_>>().len());
}
