use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;

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
