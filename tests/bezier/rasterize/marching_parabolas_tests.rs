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

                if (expected - val).abs() >= 1.0 {
                    num_greater_than_1 += 1;
                }

                if (expected - val).abs() >= 0.5 {
                    num_greater_than_0_5 += 1;
                }
            }
        }
    }

    println!("> 0.5 = {}", num_greater_than_0_5);
    println!("> 1.0 = {}", num_greater_than_1);
    assert!(num_greater_than_1 == 0);
}
