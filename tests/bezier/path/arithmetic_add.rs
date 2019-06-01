use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;

use super::svg::*;

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
    let points          = shared_point.points().collect::<Vec<_>>();

    assert!(shared_point.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(7.0, 3.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(7.0, 7.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(3.0, 7.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(3.0, 5.0)) < 0.1);
    assert!(points[6].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[7].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
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
    let points          = shared_point.points().collect::<Vec<_>>();

    assert!(shared_point.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(7.0, 3.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(7.0, 7.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(3.0, 7.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(3.0, 5.0)) < 0.1);
    assert!(points[6].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[7].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
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
    let points          = shared_point.points().collect::<Vec<_>>();

    assert!(shared_point.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(7.0, 3.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(7.0, 7.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(3.0, 7.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(3.0, 5.0)) < 0.1);
    assert!(points[6].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[7].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
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
    let points          = shared_point.points().collect::<Vec<_>>();

    assert!(shared_point.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(7.0, 3.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(7.0, 7.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(3.0, 7.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(3.0, 5.0)) < 0.1);
    assert!(points[6].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[7].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
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
    let points          = shared_point.points().collect::<Vec<_>>();

    assert!(shared_point.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(7.0, 3.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(7.0, 7.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(3.0, 7.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(3.0, 5.0)) < 0.1);
    assert!(points[6].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[7].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
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
    let points          = shared_point.points().collect::<Vec<_>>();

    assert!(shared_point.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(3.0, 5.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(3.0, 7.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(7.0, 7.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(7.0, 3.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(points[6].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[7].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
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
    let points          = shared_point.points().collect::<Vec<_>>();

    assert!(shared_point.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(3.0, 5.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(3.0, 7.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(7.0, 7.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(7.0, 3.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(points[6].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[7].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
}

#[test]
fn remove_interior_points_complex_1() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(589.8298950195313, 841.699951171875))
        .curve_to((Coord2(589.8298950195313, 841.699951171875), Coord2(589.8298950195313, 841.699951171875)), Coord2(589.8298950195313, 841.699951171875))
        .curve_to((Coord2(585.0781860351563, 841.545166015625), Coord2(588.116943359375, 846.1569213867188)), Coord2(589.9508056640625, 846.92041015625))
        .curve_to((Coord2(593.9074096679688, 850.3338623046875), Coord2(596.3680419921875, 855.8639526367188)), Coord2(600.2550048828125, 860.024169921875))
        .curve_to((Coord2(602.3019409179688, 864.72900390625), Coord2(603.487060546875, 861.721435546875)), Coord2(602.1428833007813, 859.0895385742188))
        .curve_to((Coord2(607.4638061523438, 858.4710693359375), Coord2(614.4444580078125, 855.14404296875)), Coord2(608.3931884765625, 855.6187133789063))
        .curve_to((Coord2(604.7843627929688, 851.9526977539063), Coord2(601.4735107421875, 847.9655151367188)), Coord2(597.78515625, 843.8760986328125))
        .curve_to((Coord2(601.0536499023438, 837.7391357421875), Coord2(590.90966796875, 841.439453125)), Coord2(587.8450927734375, 847.3414916992188))
        .curve_to((Coord2(592.2240600585938, 850.6311645507813), Coord2(595.8001098632813, 856.1324462890625)), Coord2(599.6971435546875, 861.4691772460938))
        .curve_to((Coord2(599.6600952148438, 866.2685546875), Coord2(601.5029907226563, 861.010498046875)), Coord2(601.408447265625, 857.6356811523438))
        .curve_to((Coord2(605.051025390625, 858.197509765625), Coord2(608.0866088867188, 854.1636352539063)), Coord2(597.3378295898438, 846.8604125976563))
        .curve_to((Coord2(597.2238159179688, 836.9576416015625), Coord2(590.7571411132813, 843.5430297851563)), Coord2(587.1199340820313, 848.599365234375))
        .curve_to((Coord2(588.7532348632813, 853.0540161132813), Coord2(591.633544921875, 856.119873046875)), Coord2(594.626708984375, 853.6188354492188))
        .curve_to((Coord2(596.7156982421875, 852.8362426757813), Coord2(595.0059814453125, 845.878662109375)), Coord2(591.52490234375, 845.5113525390625))
        .curve_to((Coord2(585.76171875, 847.6647338867188), Coord2(580.7750244140625, 855.853759765625)), Coord2(586.7627563476563, 853.3876342773438))
        .curve_to((Coord2(588.5208129882813, 859.3195190429688), Coord2(594.2566528320313, 860.6160278320313)), Coord2(592.3621826171875, 860.9254760742188))
        .curve_to((Coord2(594.9733276367188, 864.4375), Coord2(593.3421020507813, 848.7232055664063)), Coord2(586.76220703125, 847.8418579101563))
        .curve_to((Coord2(589.7845458984375, 841.6835327148438), Coord2(583.6079711914063, 848.498046875)), Coord2(580.9037475585938, 853.9146118164063))
        .curve_to((Coord2(580.701904296875, 853.186767578125), Coord2(578.50439453125, 857.2315063476563)), Coord2(581.5901489257813, 860.4940795898438))
        .curve_to((Coord2(585.6346435546875, 863.285400390625), Coord2(589.900146484375, 854.3807373046875)), Coord2(584.1525268554688, 856.2511596679688))
        .curve_to((Coord2(590.3831787109375, 852.05712890625), Coord2(578.9157104492188, 850.2012329101563)), Coord2(574.5430297851563, 856.5203247070313))
        .curve_to((Coord2(573.6943969726563, 863.1355590820313), Coord2(580.0052490234375, 871.26220703125)), Coord2(575.3004760742188, 871.1060791015625))
        .curve_to((Coord2(576.81103515625, 870.624267578125), Coord2(572.30712890625, 859.2913818359375)), Coord2(570.9198608398438, 861.718994140625))
        .curve_to((Coord2(572.5287475585938, 864.7382202148438), Coord2(581.41259765625, 882.9050903320313)), Coord2(580.4722900390625, 881.7498779296875))
        .curve_to((Coord2(580.0606689453125, 880.2344970703125), Coord2(575.6553955078125, 869.0311889648438)), Coord2(573.716552734375, 868.6065673828125))
        .curve_to((Coord2(570.4192504882813, 866.5391845703125), Coord2(572.1432495117188, 889.7837524414063)), Coord2(575.9349365234375, 889.2540893554688))
        .curve_to((Coord2(579.9112548828125, 889.1182250976563), Coord2(573.3362426757813, 870.1537475585938)), Coord2(570.325439453125, 872.933349609375))
        .curve_to((Coord2(566.7039184570313, 872.4866333007813), Coord2(575.889892578125, 896.3516845703125)), Coord2(580.193359375, 885.1004028320313))
        .curve_to((Coord2(578.9361572265625, 882.8379516601563), Coord2(578.29638671875, 880.9623413085938)), Coord2(577.2049560546875, 878.0570678710938))
        .curve_to((Coord2(576.3244018554688, 875.5227661132813), Coord2(575.8396606445313, 874.0106811523438)), Coord2(575.3523559570313, 871.5857543945313))
        .curve_to((Coord2(567.6146240234375, 879.8153076171875), Coord2(569.26904296875, 890.168212890625)), Coord2(572.8831176757813, 890.166259765625))
        .curve_to((Coord2(580.7759399414063, 887.835693359375), Coord2(580.0247802734375, 885.56103515625)), Coord2(572.6173095703125, 889.1515502929688))
        .curve_to((Coord2(572.6390991210938, 889.1546020507813), Coord2(572.2571411132813, 889.167724609375)), Coord2(572.2820434570313, 889.1630249023438))
        .curve_to((Coord2(570.7896728515625, 887.8728637695313), Coord2(567.4065551757813, 888.0462036132813)), Coord2(572.1813354492188, 892.5457763671875))
        .curve_to((Coord2(570.9942016601563, 894.4725341796875), Coord2(577.9598999023438, 900.7188720703125)), Coord2(582.4383544921875, 902.0015258789063))
        .curve_to((Coord2(582.8182373046875, 902.308349609375), Coord2(586.3283081054688, 901.2371826171875)), Coord2(586.35205078125, 900.798583984375))
        .curve_to((Coord2(588.947998046875, 898.2053833007813), Coord2(592.195068359375, 891.016845703125)), Coord2(591.5047607421875, 889.0786743164063))
        .curve_to((Coord2(592.836669921875, 884.303955078125), Coord2(592.759033203125, 882.3919677734375)), Coord2(593.544921875, 881.51806640625))
        .curve_to((Coord2(594.1064453125, 880.7155151367188), Coord2(593.8582153320313, 881.4864501953125)), Coord2(596.4064331054688, 879.8722534179688))
        .curve_to((Coord2(597.3624877929688, 879.4691162109375), Coord2(597.849365234375, 879.2901611328125)), Coord2(598.5863037109375, 879.035400390625))
        .curve_to((Coord2(598.9070434570313, 878.928466796875), Coord2(599.0929565429688, 878.8623657226563)), Coord2(599.3098754882813, 878.7935180664063))
        .curve_to((Coord2(596.8707275390625, 882.4271240234375), Coord2(601.0760498046875, 876.7950439453125)), Coord2(603.7096557617188, 873.0940551757813))
        .curve_to((Coord2(603.7099609375, 873.09423828125), Coord2(603.7090454101563, 872.9913940429688)), Coord2(603.708740234375, 872.9912109375))
        .curve_to((Coord2(602.2408447265625, 869.050048828125), Coord2(594.5162353515625, 860.6947021484375)), Coord2(590.5521850585938, 859.4874267578125))
        .curve_to((Coord2(584.2527465820313, 852.785888671875), Coord2(581.061279296875, 852.8796997070313)), Coord2(581.9452514648438, 853.1057739257813))
        .curve_to((Coord2(582.593017578125, 852.9660034179688), Coord2(580.7717895507813, 856.9833984375)), Coord2(580.3909301757813, 856.538818359375))
        .curve_to((Coord2(580.433837890625, 856.8338623046875), Coord2(577.17236328125, 858.1321411132813)), Coord2(578.2269897460938, 857.4826049804688))
        .curve_to((Coord2(579.6019897460938, 857.3278198242188), Coord2(581.242431640625, 858.7636108398438)), Coord2(586.4614868164063, 860.9908447265625))
        .curve_to((Coord2(589.0213012695313, 862.7692260742188), Coord2(595.4768676757813, 865.4718017578125)), Coord2(598.6329345703125, 865.7658081054688))
        .curve_to((Coord2(598.567138671875, 866.820556640625), Coord2(603.420166015625, 864.4375610351563)), Coord2(603.9707641601563, 863.19189453125))
        .curve_to((Coord2(604.5771484375, 862.9888916015625), Coord2(605.8325805664063, 859.209716796875)), Coord2(605.4430541992188, 858.7909545898438))
        .curve_to((Coord2(604.993408203125, 855.49951171875), Coord2(601.7562866210938, 849.7847900390625)), Coord2(600.6087036132813, 850.0838623046875))
        .curve_to((Coord2(598.4024047851563, 846.3101196289063), Coord2(598.4458618164063, 848.7549438476563)), Coord2(599.054931640625, 849.7504272460938))
        .curve_to((Coord2(599.3753051757813, 849.5570678710938), Coord2(598.5122680664063, 852.48095703125)), Coord2(598.3043212890625, 852.2940063476563))
        .curve_to((Coord2(594.026123046875, 853.1077270507813), Coord2(590.7466430664063, 857.636474609375)), Coord2(597.5106811523438, 857.97314453125))
        .curve_to((Coord2(599.0369262695313, 859.3222045898438), Coord2(599.9699096679688, 856.8018798828125)), Coord2(600.7046508789063, 857.8336181640625))
        .curve_to((Coord2(602.2219848632813, 857.0648803710938), Coord2(604.2450561523438, 856.0399780273438)), Coord2(605.7623901367188, 855.271240234375))
        .curve_to((Coord2(606.0952758789063, 855.509765625), Coord2(607.6958618164063, 852.7109985351563)), Coord2(605.1021118164063, 850.0916748046875))
        .curve_to((Coord2(607.8820190429688, 846.5908203125), Coord2(593.875244140625, 843.7130737304688)), Coord2(588.6094360351563, 846.0635986328125))
        .curve_to((Coord2(588.1766357421875, 846.2265625), Coord2(587.21044921875, 849.5330200195313)), Coord2(587.5308227539063, 849.7504272460938))
        .curve_to((Coord2(588.139892578125, 853.0325317382813), Coord2(591.3778076171875, 858.6402587890625)), Coord2(592.365966796875, 858.1364135742188))
        .curve_to((Coord2(594.4129028320313, 861.7054443359375), Coord2(594.3702392578125, 859.3676147460938)), Coord2(593.9205932617188, 858.7909545898438))
        .curve_to((Coord2(593.531005859375, 859.0397338867188), Coord2(594.5933837890625, 855.8880004882813)), Coord2(594.7660522460938, 856.260986328125))
        .curve_to((Coord2(595.0291137695313, 855.397216796875), Coord2(599.3807983398438, 853.1875)), Coord2(598.6329345703125, 854.2422485351563))
        .curve_to((Coord2(598.4532470703125, 854.5361938476563), Coord2(597.2518920898438, 853.0940551757813)), Coord2(591.7156982421875, 850.7276611328125))
        .curve_to((Coord2(588.8387451171875, 848.8101196289063), Coord2(581.9440307617188, 846.1011962890625)), Coord2(578.2269897460938, 845.9464111328125))
        .curve_to((Coord2(577.8850708007813, 845.296875), Coord2(573.4860229492188, 846.9068603515625)), Coord2(572.7304077148438, 847.910888671875))
        .curve_to((Coord2(571.8158569335938, 847.940185546875), Coord2(569.74658203125, 852.5507202148438)), Coord2(570.394287109375, 853.1057739257813))
        .curve_to((Coord2(571.2783203125, 857.921142578125), Coord2(578.909423828125, 867.0386962890625)), Coord2(583.4408569335938, 868.6987915039063))
        .curve_to((Coord2(590.3077392578125, 875.8531494140625), Coord2(593.4223022460938, 875.197265625)), Coord2(591.9874877929688, 873.194091796875))
        .curve_to((Coord2(591.9872436523438, 873.1967163085938), Coord2(591.986328125, 873.0939331054688)), Coord2(591.9866333007813, 873.0940551757813))
        .curve_to((Coord2(594.6202392578125, 869.4052734375), Coord2(598.6082153320313, 863.8417358398438)), Coord2(595.7800903320313, 867.597900390625))
        .curve_to((Coord2(595.6080322265625, 867.6517333984375), Coord2(595.2333374023438, 867.7623901367188)), Coord2(594.86767578125, 867.8843994140625))
        .curve_to((Coord2(594.2318115234375, 868.0874633789063), Coord2(592.8426513671875, 868.5746459960938)), Coord2(591.7855224609375, 869.0294189453125))
        .curve_to((Coord2(590.3074340820313, 869.1311645507813), Coord2(585.4846801757813, 872.3850708007813)), Coord2(583.8530883789063, 874.7000122070313))
        .curve_to((Coord2(582.0348510742188, 877.52783203125), Coord2(580.2197875976563, 883.7698364257813)), Coord2(579.9202880859375, 886.5905151367188))
        .curve_to((Coord2(577.5986328125, 892.2477416992188), Coord2(579.3203735351563, 892.0960083007813)), Coord2(579.736328125, 890.97021484375))
        .curve_to((Coord2(579.3685302734375, 890.795166015625), Coord2(582.47412109375, 889.8472900390625)), Coord2(582.4383544921875, 890.154052734375))
        .curve_to((Coord2(583.9168090820313, 891.4367065429688), Coord2(587.2955322265625, 891.2642211914063)), Coord2(582.5258178710938, 886.7721557617188))
        .curve_to((Coord2(583.718017578125, 884.8529663085938), Coord2(576.7567138671875, 878.6074829101563)), Coord2(572.2820434570313, 877.3173217773438))
        .curve_to((Coord2(572.260009765625, 877.3126220703125), Coord2(571.8314208984375, 877.3272705078125)), Coord2(571.8067016601563, 877.3335571289063))
        .curve_to((Coord2(560.010498046875, 881.22509765625), Coord2(569.1023559570313, 904.441650390625)), Coord2(580.4130249023438, 899.290283203125))
        .curve_to((Coord2(587.358642578125, 896.5389404296875), Coord2(577.4598388671875, 865.2567138671875)), Coord2(569.1909790039063, 866.8143920898438))
        .curve_to((Coord2(564.2433471679688, 876.4852905273438), Coord2(565.2119750976563, 879.7418823242188)), Coord2(566.0805053710938, 882.063720703125))
        .curve_to((Coord2(566.91650390625, 884.5101928710938), Coord2(568.3486328125, 888.1217651367188)), Coord2(569.8612060546875, 890.8428344726563))
        .curve_to((Coord2(587.60546875, 903.775146484375), Coord2(589.3789672851563, 862.3728637695313)), Coord2(570.9526977539063, 861.138671875))
        .curve_to((Coord2(555.2870483398438, 863.2453002929688), Coord2(565.8951416015625, 904.8345336914063)), Coord2(580.59521484375, 900.0840454101563))
        .curve_to((Coord2(594.0852661132813, 895.3810424804688), Coord2(579.9393920898438, 852.4515380859375)), Coord2(565.9549560546875, 859.7515869140625))
        .curve_to((Coord2(557.6361083984375, 864.9191284179688), Coord2(571.5972900390625, 896.5897216796875)), Coord2(582.2666625976563, 893.362060546875))
        .curve_to((Coord2(596.2720947265625, 889.8972778320313), Coord2(586.6115112304688, 848.4343872070313)), Coord2(571.9376220703125, 850.0352172851563))
        .curve_to((Coord2(556.0201416015625, 851.1971435546875), Coord2(561.6185302734375, 885.7030029296875)), Coord2(577.4126586914063, 882.59521484375))
        .curve_to((Coord2(589.552978515625, 879.34228515625), Coord2(591.1780395507813, 853.8557739257813)), Coord2(584.5911254882813, 850.6495971679688))
        .curve_to((Coord2(573.9320678710938, 846.2090454101563), Coord2(564.6778564453125, 858.427490234375)), Coord2(573.91796875, 861.5853881835938))
        .curve_to((Coord2(572.7061767578125, 870.7952880859375), Coord2(589.696533203125, 873.0230712890625)), Coord2(593.0504760742188, 859.9937133789063))
        .curve_to((Coord2(595.9219970703125, 858.3511352539063), Coord2(586.99267578125, 843.552978515625)), Coord2(581.2372436523438, 842.6605834960938))
        .curve_to((Coord2(576.9470825195313, 848.0301513671875), Coord2(573.3963623046875, 858.2985229492188)), Coord2(577.3682861328125, 853.6705322265625))
        .curve_to((Coord2(573.3412475585938, 856.9037475585938), Coord2(593.6327514648438, 872.11083984375)), Coord2(601.6283569335938, 866.2468872070313))
        .curve_to((Coord2(603.8673706054688, 859.3587036132813), Coord2(597.4412231445313, 846.4760131835938)), Coord2(595.3387451171875, 847.1231689453125))
        .curve_to((Coord2(600.8814697265625, 844.0478515625), Coord2(586.74169921875, 842.0896606445313)), Coord2(580.974609375, 845.4783935546875))
        .curve_to((Coord2(577.481201171875, 849.10498046875), Coord2(595.04345703125, 864.8500366210938)), Coord2(599.9298095703125, 862.3732299804688))
        .curve_to((Coord2(603.3001708984375, 859.6436767578125), Coord2(598.7106323242188, 838.4157104492188)), Coord2(590.7146606445313, 839.1911010742188))
        .curve_to((Coord2(586.1773071289063, 843.9035034179688), Coord2(581.3427124023438, 853.1783447265625)), Coord2(589.6311645507813, 853.3121948242188))
        .curve_to((Coord2(591.4857788085938, 861.0636596679688), Coord2(603.1357421875, 865.7894287109375)), Coord2(608.3871459960938, 864.779541015625))
        .curve_to((Coord2(609.6311645507813, 860.09716796875), Coord2(608.9767456054688, 852.512939453125)), Coord2(607.8642578125, 855.7709350585938))
        .curve_to((Coord2(604.8263549804688, 851.1680908203125), Coord2(599.0707397460938, 843.4915161132813)), Coord2(593.83251953125, 839.4678955078125))
        .curve_to((Coord2(588.12841796875, 843.3626708984375), Coord2(584.9580688476563, 854.13720703125)), Coord2(590.4581909179688, 850.477294921875))
        .curve_to((Coord2(593.902099609375, 854.3041381835938), Coord2(597.5442504882813, 858.5637817382813)), Coord2(601.3976440429688, 862.492431640625))
        .curve_to((Coord2(596.497314453125, 864.138427734375), Coord2(605.6493530273438, 863.7943115234375)), Coord2(611.2799682617188, 862.292236328125))
        .curve_to((Coord2(610.607666015625, 857.74365234375), Coord2(606.7666625976563, 851.5074462890625)), Coord2(606.7361450195313, 853.9833984375))
        .curve_to((Coord2(602.8890380859375, 849.8455200195313), Coord2(597.0514526367188, 846.7515869140625)), Coord2(593.0549926757813, 843.3157958984375))
        .curve_to((Coord2(591.688232421875, 841.3230590820313), Coord2(585.3775024414063, 841.3017578125)), Coord2(589.6620483398438, 842.685791015625))
        .build();

    // This path has generated an error that indicates that no result path was generated (unfortunately it seems this version does not produce the error)
    let without_interior_points = path_remove_interior_points::<_, SimpleBezierPath>(&vec![path.clone()], 0.01);
    assert!(without_interior_points.len() != 0);
    assert!(without_interior_points.len() == 1);

    // Bug appears to be that not all collisions are generated (so two self-collides in a row will generate more points)
    let mut graph_path = GraphPath::from_path(&path, ());
    graph_path.self_collide(0.01);

    let initial_num_points  = graph_path.num_points();
    let initial_num_edges   = graph_path.all_edges().count();
    graph_path.self_collide(0.01);

    // Self-colliding twice in a row should not produce any new edges (or points, though the same number of edges but a different number of points probably indicates that the result is fine)
    println!("{} -> {}, {} -> {}", initial_num_edges, graph_path.all_edges().count(), initial_num_points, graph_path.num_points());

    // Extra collisions show up as extra points
    for p in initial_num_points..graph_path.num_points() {
        println!("Extra point: {:?}", graph_path.point_position(p));
    }

    assert!(graph_path.all_edges().count() == initial_num_edges);
    assert!(graph_path.num_points() == initial_num_points);
}
