use flo_curves::*;
use flo_curves::line::*;

#[test]
fn nearest_x_axis_aligned_1() {
    let x_line = (Coord2(0.0, 0.0), Coord2(1.0, 0.0));

    assert!(x_line.nearest_point(&Coord2(100.0, 20.0)).distance_to(&Coord2(100.0, 0.0)) < 0.01);
}

#[test]
fn nearest_x_axis_aligned_2() {
    let x_line = (Coord2(0.0, 30.0), Coord2(1.0, 30.0));

    assert!(x_line.nearest_point(&Coord2(100.0, 20.0)).distance_to(&Coord2(100.0, 30.0)) < 0.01);
}

#[test]
fn nearest_y_axis_aligned_1() {
    let y_line = (Coord2(0.0, 0.0), Coord2(0.0, 1.0));

    assert!(y_line.nearest_point(&Coord2(100.0, 20.0)).distance_to(&Coord2(0.0, 20.0)) < 0.01);
}

#[test]
fn nearest_y_axis_aligned_2() {
    let y_line = (Coord2(30.0, 0.0), Coord2(30.0, 1.0));

    assert!(y_line.nearest_point(&Coord2(100.0, 20.0)).distance_to(&Coord2(30.0, 20.0)) < 0.01);
}

#[test]
fn nearest_on_line() {
    // Use a point known to be on the line
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let origin_nearest      = line.nearest_point(&Coord2(4.0, 5.0));

    assert!(origin_nearest.distance_to(&Coord2(4.0, 5.0)) < 0.01);
}

#[test]
fn nearest_origin() {
    // Create a line and a perpendicular line that goes through the origin
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let perpendicular_line  = (Coord2(4.0, 5.0), Coord2(4.0 - (9.0-5.0), 5.0 - (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_origin         = perpendicular_line.point_at_pos(2.0);
    let origin_nearest      = line.nearest_point(&from_origin);

    assert!(origin_nearest.distance_to(&Coord2(4.0, 5.0)) < 0.01);
}

#[test]
fn nearest_other_point() {
    // Create a line and a perpendicular line that goes through the origin
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let point               = line.point_at_pos(0.3);
    let perpendicular_line  = (point, Coord2(point.x() - (9.0-5.0), point.y() - (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_point          = perpendicular_line.point_at_pos(2.0);
    let point_nearest       = line.nearest_point(&from_point);

    assert!(point_nearest.distance_to(&point) < 0.01);
}

#[test]
fn nearest_origin_t() {
    // Create a line and a perpendicular line that goes through the origin
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let perpendicular_line  = (Coord2(4.0, 5.0), Coord2(4.0 - (9.0-5.0), 5.0 - (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_origin         = perpendicular_line.point_at_pos(2.0);
    let origin_nearest      = line.nearest_pos(&from_origin);

    assert!(origin_nearest.abs() < 0.01);
}

#[test]
fn nearest_other_point_t() {
    // Create a line and a perpendicular line that goes through the origin
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let point               = line.point_at_pos(0.3);
    let perpendicular_line  = (point, Coord2(point.x() - (9.0-5.0), point.y() - (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_point          = perpendicular_line.point_at_pos(2.0);
    let point_nearest       = line.nearest_pos(&from_point);

    assert!((point_nearest-0.3).abs() < 0.01);
}
