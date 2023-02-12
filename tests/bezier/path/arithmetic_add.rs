use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;
use flo_curves::debug::*;

use super::svg::*;
use super::checks::*;

#[test]
fn add_two_overlapping_circles() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(7.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    let mut num_points_on_circle1   = 0;
    let mut num_points_on_circle2   = 0;
    let mut num_points_on_both      = 0;

    for point in points {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);
        let distance_to_circle2 = Coord2(7.0, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-4.0).abs() < 0.01 || (distance_to_circle2-4.0).abs() < 0.01);

        println!("{:?} {:?} {:?}", point, distance_to_circle1, distance_to_circle2);

        if (distance_to_circle1-4.0).abs() < 0.01 && (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_both += 1 }
        else if (distance_to_circle1-4.0).abs() < 0.01 { num_points_on_circle1 += 1 }
        else if (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_circle2 += 1 }
    }

    println!("{:?} {:?} {:?}", num_points_on_circle1, num_points_on_circle2, num_points_on_both);

    assert!(num_points_on_circle1 == 2);
    assert!(num_points_on_circle2 == 2);
    assert!(num_points_on_both == 2);
}

#[test]
fn add_two_identical_circles() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point).collect::<Vec<_>>();

    for point in points.iter() {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-4.0).abs() < 0.01);

        println!("{:?} {:?}", point, distance_to_circle1);
    }

    assert!(points.len() == 4);
}

#[test]
fn add_two_very_close_circles() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(5.01, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    println!("{:?}", combined_circles.len());
    assert!(combined_circles.len() != 0);
    assert!(combined_circles.len() != 2);
    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    for point in points {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-4.0).abs() < 0.1);

        println!("{:?} {:?}", point, distance_to_circle1);
    }
}

#[test]
fn add_two_close_circles() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(496.9997044935593, 5.0), 300.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(503.0002955064407, 5.0), 300.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    assert!(combined_circles.len() != 0);
    assert!(combined_circles.len() != 2);
    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    for point in points {
        let distance_to_circle1 = Coord2(500.0, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-300.0).abs() < 10.0);

        println!("{:?} {:?}", point, distance_to_circle1);
    }
}

#[test]
fn add_two_overlapping_circles_via_combination_chain() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(7.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_combine::<SimpleBezierPath>(PathCombine::Add(vec![PathCombine::Path(vec![circle1]), PathCombine::Path(vec![circle2])]), 0.01);

    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    let mut num_points_on_circle1   = 0;
    let mut num_points_on_circle2   = 0;
    let mut num_points_on_both      = 0;

    for point in points {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);
        let distance_to_circle2 = Coord2(7.0, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-4.0).abs() < 0.01 || (distance_to_circle2-4.0).abs() < 0.01);

        println!("{:?} {:?} {:?}", point, distance_to_circle1, distance_to_circle2);

        if (distance_to_circle1-4.0).abs() < 0.01 && (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_both += 1 }
        else if (distance_to_circle1-4.0).abs() < 0.01 { num_points_on_circle1 += 1 }
        else if (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_circle2 += 1 }
    }

    println!("{:?} {:?} {:?}", num_points_on_circle1, num_points_on_circle2, num_points_on_both);

    assert!(num_points_on_circle1 == 2);
    assert!(num_points_on_circle2 == 2);
    assert!(num_points_on_both == 2);
}

#[test]
fn add_series_of_circles_via_combination_chain() {
    // Two overlapping circles
    let circles             = (0..4).into_iter()
        .map(|idx| Circle::new(Coord2(5.0 + (idx as f64)*2.0, 4.0), 4.0).to_path::<SimpleBezierPath>())
        .map(|circle| PathCombine::Path(vec![circle]));
    let combine             = PathCombine::Add(circles.collect());
    let combined_circles    = path_combine::<SimpleBezierPath>(combine, 0.01);

    assert!(combined_circles.len() == 1);
}

#[test]
fn add_circle_inside_circle() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    let mut num_points_on_circle1   = 0;

    for point in points {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);

        // Must be on the circle
        assert!((distance_to_circle1-4.0).abs() < 0.01);
        if (distance_to_circle1-4.0).abs() < 0.01 { num_points_on_circle1 += 1 }
    }

    assert!(num_points_on_circle1 == 4);
}

#[test]
fn add_two_overlapping_circles_further_apart() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(12.9, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    let mut num_points_on_circle1   = 0;
    let mut num_points_on_circle2   = 0;
    let mut num_points_on_both      = 0;

    for point in points {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);
        let distance_to_circle2 = Coord2(12.9, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-4.0).abs() < 0.01 || (distance_to_circle2-4.0).abs() < 0.01);

        println!("{:?} {:?} {:?}", point, distance_to_circle1, distance_to_circle2);

        if (distance_to_circle1-4.0).abs() < 0.01 && (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_both += 1 }
        else if (distance_to_circle1-4.0).abs() < 0.01 { num_points_on_circle1 += 1 }
        else if (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_circle2 += 1 }
    }

    println!("{:?} {:?} {:?}", num_points_on_circle1, num_points_on_circle2, num_points_on_both);

    assert!(num_points_on_circle1 == 4);
    assert!(num_points_on_circle2 == 4);
    assert!(num_points_on_both == 2);
}

#[test]
fn add_two_overlapping_circles_with_one_reversed() {
    // Two overlapping circles (one clockwise, one anti-clockwise)
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(7.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = circle2.reversed::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    println!("{:?}", combined_circles);
    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    let mut num_points_on_circle1   = 0;
    let mut num_points_on_circle2   = 0;
    let mut num_points_on_both      = 0;

    for point in points {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);
        let distance_to_circle2 = Coord2(7.0, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-4.0).abs() < 0.01 || (distance_to_circle2-4.0).abs() < 0.01);

        println!("{:?} {:?} {:?}", point, distance_to_circle1, distance_to_circle2);

        if (distance_to_circle1-4.0).abs() < 0.01 && (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_both += 1 }
        else if (distance_to_circle1-4.0).abs() < 0.01 { num_points_on_circle1 += 1 }
        else if (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_circle2 += 1 }
    }

    println!("{:?} {:?} {:?}", num_points_on_circle1, num_points_on_circle2, num_points_on_both);

    assert!(num_points_on_circle1 == 2);
    assert!(num_points_on_circle2 == 2);
    assert!(num_points_on_both == 2);
}

#[test]
fn add_two_non_overlapping_circles() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(20.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.1);

    println!("{:?}", combined_circles);
    assert!(combined_circles.len() == 2);
}

#[test]
fn add_two_doughnuts() {
    // Two overlapping circles
    let circle1         = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle1   = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();
    let circle2         = Circle::new(Coord2(9.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle2   = Circle::new(Coord2(9.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    println!("{}", svg_path_string(&circle1));
    println!("{}", svg_path_string(&inner_circle1));
    println!("{}", svg_path_string(&circle2));
    println!("{}", svg_path_string(&inner_circle2));

    // Combine them
    let combined_circles = path_add::<_, _, SimpleBezierPath>(&vec![circle1, inner_circle1], &vec![circle2, inner_circle2], 0.09);

    println!("{:?}", combined_circles.len());
    println!("{:?}", combined_circles);
    println!("{:?}", combined_circles.iter().map(|path| svg_path_string(path)).collect::<Vec<_>>());
    assert!(combined_circles.len() == 4);
}

#[test]
fn remove_interior_from_circle_removes_nothing() {
    let circle      = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let removed     = path_remove_interior_points::<_, SimpleBezierPath>(&vec![circle.clone()], 0.1);

    assert!(removed.len() == 1);
    assert!(removed[0].1.len() == circle.1.len());

    assert!(path_has_points_in_order(removed[0].clone(), circle.1.iter().cloned().collect(), 0.01));
}

#[test]
fn remove_interior_for_ring_removes_center() {
    let ring1   = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2   = Circle::new(Coord2(2.0, 2.0), 1.5).to_path::<SimpleBezierPath>();

    let removed = path_remove_interior_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone()], 0.01);

    assert!(removed.len() == 1);

    assert!(path_has_points_in_order(removed[0].clone(), ring1.1.iter().cloned().collect(), 0.01));
}

#[test]
fn remove_interior_for_ring_with_crossbar_removes_center() {
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2       = Circle::new(Coord2(2.0, 2.0), 1.5).to_path::<SimpleBezierPath>();
    let crossbar1   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.2, 1.9))
        .line_to(Coord2(0.2, 2.1))
        .line_to(Coord2(3.8, 2.1))
        .line_to(Coord2(3.8, 1.9))
        .line_to(Coord2(0.2, 1.9))
        .build();

    let removed     = path_remove_interior_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone(), crossbar1.clone()], 0.01);

    assert!(removed.len() == 1);

    assert!(path_has_points_in_order(removed[0].clone(), ring1.1.iter().cloned().collect(), 0.01));
}

#[test]
#[ignore]   // TODO: this is failing due to an odd issue (generates a weird extra path, probably due to snapping)
fn remove_interior_for_ring_with_offset_crossbar_removes_center_1() {
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2       = Circle::new(Coord2(2.0, 2.0), 1.7).to_path::<SimpleBezierPath>();
    let crossbar1   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.2, 0.9))
        .line_to(Coord2(0.2, 1.1))
        .line_to(Coord2(3.8, 1.1))
        .line_to(Coord2(3.8, 0.9))
        .line_to(Coord2(0.2, 0.9))
        .build();

    // Create the graph path from the source side
    let path = vec![ring1.clone(), ring2.clone(), crossbar1.clone()];
    let mut merged_path = GraphPath::new();
    merged_path         = merged_path.merge(GraphPath::from_merged_paths(path.iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));

    // Collide the path with itself to find the intersections
    merged_path.self_collide(0.01);
    merged_path.round(0.01);

    merged_path.set_exterior_by_removing_interior_points();

    println!("{}", graph_path_svg_string(&merged_path, vec![]));

    // Try the actual removing operation
    let removed     = path_remove_interior_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone(), crossbar1.clone()], 0.01);

    println!("{:?}", removed.len());
    assert!(removed.len() == 1);
}

#[test]
fn remove_interior_for_ring_with_offset_crossbar_removes_center_2() {
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2       = Circle::new(Coord2(2.0, 2.0), 1.7).to_path::<SimpleBezierPath>();
    let crossbar1   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.2, 0.9))
        .line_to(Coord2(0.2, 1.1))
        .line_to(Coord2(4.0, 1.1))
        .line_to(Coord2(4.0, 0.9))
        .line_to(Coord2(0.2, 0.9))
        .build();

    // Create the graph path from the source side
    let path = vec![ring1.clone(), ring2.clone(), crossbar1.clone()];
    let mut merged_path = GraphPath::new();
    merged_path         = merged_path.merge(GraphPath::from_merged_paths(path.iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));

    // Collide the path with itself to find the intersections
    merged_path.self_collide(0.01);
    merged_path.round(0.01);

    merged_path.set_exterior_by_removing_interior_points();

    println!("{}", graph_path_svg_string(&merged_path, vec![]));

    // Try the actual removing operation
    let removed     = path_remove_interior_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone(), crossbar1.clone()], 0.01);

    println!("{:?}", removed.len());
    assert!(removed.len() == 1);
}

#[test]
fn ring_with_offset_crossbar_ray_casting_issue() {
    // Hit a bug where the ray (Coord2(0.3853378796624052, 0.7560017173290998), Coord2(0.385337879662404, 1.0999999999999999)) seems to be missing intersections
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2       = Circle::new(Coord2(2.0, 2.0), 1.7).to_path::<SimpleBezierPath>();
    let crossbar1   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.2, 0.9))
        .line_to(Coord2(0.2, 1.1))
        .line_to(Coord2(3.8, 1.1))
        .line_to(Coord2(3.8, 0.9))
        .line_to(Coord2(0.2, 0.9))
        .build();

    let path = vec![ring1.clone(), ring2.clone(), crossbar1.clone()];
    let mut merged_path = GraphPath::new();
    merged_path         = merged_path.merge(GraphPath::from_merged_paths(path.iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));

    merged_path.self_collide(0.01);
    merged_path.round(0.01);

    let ray_cast        = merged_path.ray_collisions(&(Coord2(0.3853378796624052, 0.7560017173290998), Coord2(0.385337879662404, 1.0999999999999999)));
    println!("{}: {:?}", ray_cast.len(), ray_cast);
    assert!(ray_cast.len() == 6);
}

#[test]
fn remove_interior_for_ring_with_cross_removes_center() {
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2       = Circle::new(Coord2(2.0, 2.0), 1.5).to_path::<SimpleBezierPath>();
    let crossbar1   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.2, 1.9))
        .line_to(Coord2(0.2, 2.1))
        .line_to(Coord2(3.8, 2.1))
        .line_to(Coord2(3.8, 1.9))
        .build();
    let crossbar2   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.9, 0.2))
        .line_to(Coord2(2.1, 0.2))
        .line_to(Coord2(2.1, 3.8))
        .line_to(Coord2(1.9, 3.8))
        .build();

    let removed     = path_remove_interior_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone(), crossbar1.clone(), crossbar2.clone()], 0.01);

    // Check the removed result
    assert!(removed.len() == 1);

    assert!(path_has_points_in_order(removed[0].clone(), ring1.1.iter().cloned().collect(), 0.01));
}

#[test]
fn remove_overlapped_for_ring_does_not_remove_center() {
    let ring1   = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2   = Circle::new(Coord2(2.0, 2.0), 1.5).to_path::<SimpleBezierPath>();

    let removed = path_remove_overlapped_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone()], 0.01);

    assert!(removed.len() == 2);

    assert!(path_has_points_in_order(removed[0].clone(), ring1.1.iter().cloned().collect(), 0.01));

    assert!(path_has_points_in_order(removed[1].clone(), ring2.1.iter().cloned().collect(), 0.01));
}

#[test]
fn remove_overlapped_for_ring_with_overlapping_crossbar() {
    // Crossbar overlaps both the main ring and the center
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2       = Circle::new(Coord2(2.0, 2.0), 1.5).to_path::<SimpleBezierPath>();
    let crossbar1   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.2, 1.9))
        .line_to(Coord2(0.2, 2.1))
        .line_to(Coord2(3.8, 2.1))
        .line_to(Coord2(3.8, 1.9))
        .line_to(Coord2(0.2, 1.9))
        .build();

    let removed     = path_remove_overlapped_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone(), crossbar1.clone()], 0.01);

    assert!(removed.len() == 5);
}

#[test]
fn remove_overlapped_for_ring_with_crossbar_in_space() {
    // Crossbar is floating in space here
    let ring1       = Circle::new(Coord2(2.0, 2.0), 2.0).to_path::<SimpleBezierPath>();
    let ring2       = Circle::new(Coord2(2.0, 2.0), 1.5).to_path::<SimpleBezierPath>();
    let crossbar1   = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.6, 0.9))
        .line_to(Coord2(1.6, 1.1))
        .line_to(Coord2(2.4, 1.1))
        .line_to(Coord2(2.4, 0.9))
        .build();

    let removed     = path_remove_overlapped_points::<_, SimpleBezierPath>(&vec![ring1.clone(), ring2.clone(), crossbar1.clone()], 0.01);

    assert!(removed.len() == 3);
}

#[test]
fn remove_interior_points_basic() {
    let with_interior_point = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(2.0, 2.0))
        .line_to(Coord2(4.0, 2.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let with_points_removed: Vec<SimpleBezierPath> = path_remove_interior_points(&vec![with_interior_point], 0.1);

    // Should be 5 points in the path with points removed
    assert!(with_points_removed.len() == 1);
    assert!(with_points_removed[0].points().count() != 6);
    assert!(with_points_removed[0].points().count() == 5);

    let expected_points = vec![
        Coord2(1.0, 1.0),
        Coord2(1.0, 5.0),
        Coord2(5.0, 5.0),
        Coord2(5.0, 1.0),
        Coord2(3.0, 3.0)
    ];

    assert!(expected_points.iter().any(|expected| with_points_removed[0].start_point().distance_to(expected) < 0.1));
    for (_cp1, _cp2, point) in with_points_removed[0].points() {
        assert!(expected_points.iter().any(|expected| point.distance_to(expected) < 0.1));
    }
}

#[test]
fn self_collide_is_stable() {
    let with_interior_point = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(2.0, 2.0))
        .line_to(Coord2(4.0, 2.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path      = GraphPath::from_path(&with_interior_point, ());

    graph_path.self_collide(0.1);
    let initial_num_points  = graph_path.num_points();
    let initial_num_edges   = graph_path.all_edges().count();

    graph_path.self_collide(0.1);
    assert!(graph_path.all_edges().count() == initial_num_edges);
    assert!(graph_path.num_points() == initial_num_points);
}

#[test]
fn rectangle_add_graph_path() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 3.0))
        .build();

    let path = GraphPath::from_path(&rectangle1, ());
    assert!(path.all_edges().count() == 4);
    let path = path.collide(GraphPath::from_path(&rectangle2, ()), 0.01);
    assert!(path.all_edges().count() == 12);
}

#[test]
fn rectangle_add() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Add them
    let shared_point = path_add::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(shared_point.len() == 1);

    let shared_point    = &shared_point[0];

    assert!(path_has_end_points_in_order(shared_point.clone(), vec![
        Coord2(5.0, 1.0),
        Coord2(5.0, 3.0),
        Coord2(7.0, 3.0),
        Coord2(7.0, 7.0),
        Coord2(3.0, 7.0),
        Coord2(3.0, 5.0),
        Coord2(1.0, 5.0),
        Coord2(1.0, 1.0),
    ], 0.1));
}

#[test]
fn rectangle_add_with_shared_point() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Add them
    let shared_point = path_add::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(shared_point.len() == 1);

    let shared_point    = &shared_point[0];

    assert!(path_has_end_points_in_order(shared_point.clone(), vec![
        Coord2(5.0, 1.0),
        Coord2(5.0, 3.0),
        Coord2(7.0, 3.0),
        Coord2(7.0, 7.0),
        Coord2(3.0, 7.0),
        Coord2(3.0, 5.0),
        Coord2(1.0, 5.0),
        Coord2(1.0, 1.0),
    ], 0.1));
}

#[test]
fn rectangle_add_with_shared_point_2() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Add them
    let shared_point = path_add::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(shared_point.len() == 1);

    let shared_point    = &shared_point[0];

    assert!(path_has_end_points_in_order(shared_point.clone(), vec![
        Coord2(5.0, 1.0),
        Coord2(5.0, 3.0),
        Coord2(7.0, 3.0),
        Coord2(7.0, 7.0),
        Coord2(3.0, 7.0),
        Coord2(3.0, 5.0),
        Coord2(1.0, 5.0),
        Coord2(1.0, 1.0),
    ], 0.1));
}

#[test]
fn rectangle_add_with_shared_point_3() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Add them
    let shared_point = path_add::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(shared_point.len() == 1);

    let shared_point    = &shared_point[0];

    assert!(path_has_end_points_in_order(shared_point.clone(), vec![
        Coord2(5.0, 1.0),
        Coord2(5.0, 3.0),
        Coord2(7.0, 3.0),
        Coord2(7.0, 7.0),
        Coord2(3.0, 7.0),
        Coord2(3.0, 5.0),
        Coord2(1.0, 5.0),
        Coord2(1.0, 1.0),
    ], 0.1));
}

#[test]
fn rectangle_add_with_shared_point_4() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(3.0, 3.0))
        .build()
        .reversed::<SimpleBezierPath>();

    // Print out the graph path generated by adding these two points
    let mut gp = GraphPath::from_path(&rectangle1, PathLabel(0, PathDirection::Clockwise)).collide(GraphPath::from_path(&rectangle2, PathLabel(1, PathDirection::Clockwise)), 0.01);
    gp.set_exterior_by_adding();
    println!("{:?}", gp);

    // Add them
    let shared_point = path_add::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(shared_point.len() == 1);

    let shared_point    = &shared_point[0];

    assert!(path_has_end_points_in_order(shared_point.clone(), vec![
        Coord2(5.0, 1.0),
        Coord2(5.0, 3.0),
        Coord2(7.0, 3.0),
        Coord2(7.0, 7.0),
        Coord2(3.0, 7.0),
        Coord2(3.0, 5.0),
        Coord2(1.0, 5.0),
        Coord2(1.0, 1.0),
    ], 0.1));
}

#[test]
fn rectangle_add_with_shared_point_5() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build()
        .reversed::<SimpleBezierPath>();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Add them
    let shared_point = path_add::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(shared_point.len() == 1);

    let shared_point    = &shared_point[0];

    assert!(path_has_end_points_in_order(shared_point.clone(), vec![
        Coord2(1.0, 5.0),
        Coord2(3.0, 5.0),
        Coord2(3.0, 7.0),
        Coord2(7.0, 7.0),
        Coord2(7.0, 3.0),
        Coord2(5.0, 3.0),
        Coord2(5.0, 1.0),
        Coord2(1.0, 1.0),
    ], 0.1));
}

#[test]
fn rectangle_add_with_shared_point_6() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 3.0)) // Shared point
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build()
        .reversed::<SimpleBezierPath>();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Add them
    let shared_point = path_add::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(shared_point.len() == 1);

    let shared_point    = &shared_point[0];

    assert!(path_has_end_points_in_order(shared_point.clone(), vec![
        Coord2(1.0, 5.0),
        Coord2(3.0, 5.0),
        Coord2(3.0, 7.0),
        Coord2(7.0, 7.0),
        Coord2(7.0, 3.0),
        Coord2(5.0, 3.0),
        Coord2(5.0, 1.0),
        Coord2(1.0, 1.0),
    ], 0.1));
}
