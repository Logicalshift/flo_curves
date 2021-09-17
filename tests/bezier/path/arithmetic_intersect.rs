use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;

use std::f64;
use std::iter;

#[test]
fn intersect_two_doughnuts() {
    // Two overlapping circles
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle1   = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(9.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle2   = Circle::new(Coord2(9.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_intersect::<_, _, SimpleBezierPath>(&vec![circle1, inner_circle1], &vec![circle2, inner_circle2], 0.1);

    println!("{:?}", combined_circles.len());
    println!("{:?}", combined_circles);
    assert!(combined_circles.len() == 2);
}

#[test]
fn full_intersect_two_doughnuts() {
    // Two overlapping circles
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle1   = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(9.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle2   = Circle::new(Coord2(9.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    // Combine them
    let intersection    = path_full_intersect::<_, _, SimpleBezierPath>(&vec![circle1, inner_circle1], &vec![circle2, inner_circle2], 0.1);

    let combined_circles = &intersection.intersecting_path;
    println!("{:?}", combined_circles.len());
    println!("{:?}", combined_circles);
    assert!(combined_circles.len() == 2);
}

#[test]
fn full_intersect_two_partially_overlapping_circles() {
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(7.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    let intersection    = path_full_intersect::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.1);

    assert!(intersection.intersecting_path.len() == 1);
    assert!(intersection.exterior_paths[0].len() == 1);
    assert!(intersection.exterior_paths[1].len() == 1);
}

#[test]
fn full_intersect_two_non_overlapping_circles() {
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(15.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    let intersection    = path_full_intersect::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.1);

    assert!(intersection.intersecting_path.len() == 0);
    assert!(intersection.exterior_paths[0].len() == 1);
    assert!(intersection.exterior_paths[1].len() == 1);
}

#[test]
fn full_intersect_interior_circles_1() {
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(5.0, 5.0), 3.5).to_path::<SimpleBezierPath>();

    let intersection    = path_full_intersect::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.1);

    assert!(intersection.intersecting_path.len() == 1);
    assert!(intersection.exterior_paths[0].len() == 2);
    assert!(intersection.exterior_paths[1].len() == 0);
}

#[test]
fn full_intersect_interior_circles_2() {
    let circle1         = Circle::new(Coord2(5.0, 5.0), 3.5).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    let intersection    = path_full_intersect::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.1);

    assert!(intersection.intersecting_path.len() == 1);
    assert!(intersection.exterior_paths[0].len() == 0);
    assert!(intersection.exterior_paths[1].len() == 2);
}

#[test]
fn fintersect_two_fully_overlapping_circles() {
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    let intersection    = path_intersect::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.1);

    assert!(intersection.len() == 1);
}

#[test]
fn full_intersect_two_fully_overlapping_circles() {
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    let intersection    = path_full_intersect::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.1);

    println!("{:?}", intersection);

    assert!(intersection.intersecting_path.len() == 1);
    assert!(intersection.exterior_paths[0].len() == 0);
    assert!(intersection.exterior_paths[1].len() == 0);
}

#[test]
fn repeatedly_full_intersect_circle() {
    // Start with a circle
    let circle          = Circle::new(Coord2(500.0, 500.0), 116.0).to_path::<SimpleBezierPath>();

    // Cut 16 triangular slices from it
    let mut remaining   = vec![circle];
    let mut slices      = vec![];

    for slice_idx in 0..16 {
        // Angle in radians of this slice
        let middle_angle            = f64::consts::PI*2.0 / 16.0 * (slice_idx as f64);
        let start_angle             = middle_angle - (f64::consts::PI*2.0 / 32.0);
        let end_angle               = middle_angle + (f64::consts::PI*2.0 / 32.0);

        // Create a triangle slice
        let (center_x, center_y)    = (500.0, 500.0);
        let (x1, y1)                = (center_x + (f64::sin(start_angle) * 300.0),  center_y + (f64::cos(start_angle) * 300.0));
        let (x2, y2)                = (center_x + (f64::sin(end_angle) * 300.0),    center_y + (f64::cos(end_angle) * 300.0));
        let (x3, y3)                = (center_x + (f64::sin(start_angle) * 16.0),    center_y + (f64::cos(start_angle) * 16.0));
        let (x4, y4)                = (center_x + (f64::sin(end_angle) * 16.0),      center_y + (f64::cos(end_angle) * 16.0));

        let fragment                = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(x3, y3))
            .line_to(Coord2(x1, y1))
            .line_to(Coord2(x2, y2))
            .line_to(Coord2(x4, y4))
            .line_to(Coord2(x3, y3))
            .build();

        // Cut the circle via the fragment
        let cut_circle              = path_full_intersect::<_, _, SimpleBezierPath>(&vec![fragment], &remaining, 0.01);

        // Add the slice and the remaining part of the circle
        slices.push(cut_circle.intersecting_path);
        remaining = cut_circle.exterior_paths[1].clone();
    }

    // Each fragment should consist of points that are either at the origin or on the circle
    for circle_fragment in slices {
        assert!(circle_fragment.len() == 1);

        let start_point = circle_fragment[0].start_point();
        let points      = circle_fragment[0].points().map(|(_, _, p)| p);
        let all_points  = iter::once(start_point).chain(points);

        for circle_point in all_points {
            let distance_to_center = circle_point.distance_to(&Coord2(500.0, 500.0));
            println!("{:?}", distance_to_center);
            assert!((distance_to_center-16.0).abs() < 0.1 || (distance_to_center-116.0).abs() < 1.0);
        }
    }

    // Should be a 16x16 polygon left over for the circle
    assert!(remaining.len() == 1);

    let start_point = remaining[0].start_point();
    let points      = remaining[0].points().map(|(_, _, p)| p);
    let all_points  = iter::once(start_point).chain(points);

    for circle_point in all_points {
        let distance_to_center = circle_point.distance_to(&Coord2(500.0, 500.0));
        println!("{:?}", distance_to_center);
        assert!((distance_to_center-0.0).abs() < 0.1 || (distance_to_center-16.0).abs() < 1.0);
    }
}

fn convert_path_to_f32_and_back((start_point, remaining_points): SimpleBezierPath) -> SimpleBezierPath {
    let start_f32       = (start_point.x() as f32, start_point.y() as f32);
    let remaining_f32   = remaining_points.into_iter().map(|(cp1, cp2, p)| {
        ((cp1.x() as f32, cp1.y() as f32), (cp2.x() as f32, cp2.y() as f32), (p.x() as f32, p.y() as f32))
    });

    let path_points = remaining_f32.map(|(cp1, cp2, p)| {
        (Coord2(cp1.0 as f64, cp1.1 as f64), Coord2(cp2.0 as f64, cp2.1 as f64), Coord2(p.0 as f64, p.1 as f64))
    });
    (Coord2(start_f32.0 as _, start_f32.1 as _), path_points.collect())
}

#[test]
fn repeatedly_full_intersect_circle_f32_intermediate_representation() {
    // Converting the remaining curve to f32 and back again results in a failed intersection due to slightly different line positions, which causes the cut to fail on some slices

    // Start with a circle
    let circle          = Circle::new(Coord2(500.0, 500.0), 116.0).to_path::<SimpleBezierPath>();

    // Cut 16 triangular slices from it
    let mut remaining   = vec![circle];
    let mut slices      = vec![];

    for slice_idx in 0..16 {
        // Angle in radians of this slice
        let middle_angle            = f64::consts::PI*2.0 / 16.0 * (slice_idx as f64);
        let start_angle             = middle_angle - (f64::consts::PI*2.0 / 32.0);
        let end_angle               = middle_angle + (f64::consts::PI*2.0 / 32.0);

        // Create a triangle slice
        let (center_x, center_y)    = (500.0, 500.0);
        let (x1, y1)                = (center_x + (f64::sin(start_angle) * 300.0),  center_y + (f64::cos(start_angle) * 300.0));
        let (x2, y2)                = (center_x + (f64::sin(end_angle) * 300.0),    center_y + (f64::cos(end_angle) * 300.0));
        let (x3, y3)                = (center_x + (f64::sin(start_angle) * 16.0),    center_y + (f64::cos(start_angle) * 16.0));
        let (x4, y4)                = (center_x + (f64::sin(end_angle) * 16.0),      center_y + (f64::cos(end_angle) * 16.0));

        let fragment                = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(x3, y3))
            .line_to(Coord2(x1, y1))
            .line_to(Coord2(x2, y2))
            .line_to(Coord2(x4, y4))
            .line_to(Coord2(x3, y3))
            .build();

        // Merge the paths and print out the number of edges
        let mut merged_path = GraphPath::new();
        merged_path         = merged_path.merge(GraphPath::from_merged_paths(vec![fragment.clone()].iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));
        merged_path         = merged_path.collide(GraphPath::from_merged_paths(remaining.iter().map(|path| (path, PathLabel(1, PathDirection::from(path))))), 0.01);
        merged_path.round(0.01);

        println!("Slice {}: {} edges", slice_idx, merged_path.all_edges().count());

        // Cut the circle via the fragment
        let cut_circle              = path_full_intersect::<_, _, SimpleBezierPath>(&vec![fragment], &remaining, 0.01);

        // Add the slice and the remaining part of the circle
        slices.push(cut_circle.intersecting_path);
        remaining = cut_circle.exterior_paths[1].clone();

        // Reduce and re-increase the precision of the remaining path (this happens in FlowBetween: even though the points will be in slightly different positions we should still be able to slice using this curve)
        remaining = remaining.into_iter().map(|path| convert_path_to_f32_and_back(path)).collect();
    }

    // Each fragment should consist of points that are either at the origin or on the circle
    for (idx, circle_fragment) in slices.iter().enumerate() {
        assert!(circle_fragment.len() == 1);

        let start_point = circle_fragment[0].start_point();
        let points      = circle_fragment[0].points().map(|(_, _, p)| p);
        let all_points  = iter::once(start_point).chain(points);

        for circle_point in all_points {
            let distance_to_center = circle_point.distance_to(&Coord2(500.0, 500.0));
            println!("- {} {:?}", idx, distance_to_center);
            assert!((distance_to_center-16.0).abs() < 0.1 || (distance_to_center-116.0).abs() < 1.0);
        }
    }

    // Should be a 16x16 polygon left over for the circle
    assert!(remaining.len() == 1);

    let start_point = remaining[0].start_point();
    let points      = remaining[0].points().map(|(_, _, p)| p);
    let all_points  = iter::once(start_point).chain(points);

    for circle_point in all_points {
        let distance_to_center = circle_point.distance_to(&Coord2(500.0, 500.0));
        println!("{:?}", distance_to_center);
        assert!((distance_to_center-0.0).abs() < 0.1 || (distance_to_center-16.0).abs() < 1.0);
    }
}
