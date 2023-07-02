use flo_curves::geo::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

use smallvec::*;

use std::f64;

///
/// Creates a cache containing a circle of the specified radius, with a center at (radius+1, radius+1)
///
fn create_circle_sample(num_points: usize, radius: f64) -> SampledApproxDistanceFieldCache {
    // Size should encompass the whole circle
    let size = (radius+1.0) * 2.0;
    let size = size.ceil() as usize;

    // Create num_points samples around the perimeter of the circle
    let center = Coord2(radius+1.0, radius+1.0);
    let points = (0..num_points)
        .map(|t| {
            let t = (t as f64)/(num_points as f64);
            let t = 2.0 * f64::consts::PI * t;
            let x = radius * t.sin();
            let y = radius * t.cos();

            Coord2(x, y) + center
        })
        .map(|point| (ContourPosition(point.x().round() as _, point.y().round() as _), point));

    // Need an 'is_inside' function
    let intercepts = (0..size).map(|y| {
        let y = y as f64;
        let y = y - center.y();

        if y.abs() <= radius {
            let intercept   = ((radius*radius) - (y*y)).sqrt();
            let min_x       = center.x() - intercept;
            let max_x       = center.x() + intercept;

            smallvec![min_x..max_x]
        } else {
            smallvec![]
        }
    });

    // Generate a cache
    SampledApproxDistanceFieldCache::from_points(points, intercepts, ContourSize(size, size))
}

fn check_circle_distances(cache: &mut SampledApproxDistanceFieldCache, radius: f64, max_error: f64) {
    // Fetch the size and expected center point
    let size    = cache.size();
    let center  = Coord2(radius+1.0, radius+1.0);

    // Iterate over every coordinate
    for y in 0..size.height() {
        for x in 0..size.height() {
            // Request the distance squared
            let distance_squared = cache.distance_squared_at_point(ContourPosition(x, y));

            // Interpret it as inside and convert it to a 'real' distance
            let is_inside   = distance_squared < 0.0;
            let distance    = distance_squared.abs().sqrt();

            // Compute the expected values
            let actual_pos = Coord2(x as _, y as _);
            let circle_pos = actual_pos - center;

            let from_center = circle_pos.dot(&circle_pos).sqrt();
            let from_edge   = from_center - radius;

            debug_assert!((from_edge.abs() - distance.abs()).abs() < max_error, "({}, {}) has distance {} but was expecting {}", x, y, from_edge.abs(), distance.abs());

            debug_assert!(!is_inside || from_edge <= 0.0, "({}, {}) should be outside the circle but isn't. Real distance {} (approximated as {})", x, y, from_edge, distance);
            debug_assert!(is_inside || from_edge >= 0.0, "({}, {}) should be inside the circle but isn't. Real distance {} (approximated as {})", x, y, from_edge, distance);
        }
    } 
}

#[test]
fn circle_100_many_points() {
    check_circle_distances(&mut create_circle_sample(4000, 100.0), 100.0, 2.0);
}

#[test]
fn circle_1000_many_points() {
    // Points further away will accumulate more errors
    check_circle_distances(&mut create_circle_sample(16000, 1000.0), 1000.0, 32.0);
}

#[test]
fn circle_100_few_points() {
    // Fewer points will produce a less accurate distance field
    check_circle_distances(&mut create_circle_sample(40, 100.0), 100.0, 32.0);
}
