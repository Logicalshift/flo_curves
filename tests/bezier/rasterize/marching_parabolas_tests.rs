use flo_curves::bezier::{rasterize::*, vectorize::{ContourPosition, SampledSignedDistanceField}};

use std::ops::{Range};

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
    assert!((rectangle.distance_at_point(ContourPosition(50, 20)) - 0.0).abs() < 0.01, "{} != 0.0", rectangle.distance_at_point(ContourPosition(20, 20)));
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
    assert!((rectangle.distance_at_point(ContourPosition(50, 20)) - 0.0).abs() < 0.01, "{} != 0.0", rectangle.distance_at_point(ContourPosition(20, 20)));
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
