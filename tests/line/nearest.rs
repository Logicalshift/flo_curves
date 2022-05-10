use flo_curves::*;
use flo_curves::line::*;

fn nearest_t_value_iteration<L>(line: &L, point: &L::Point) -> f64
where
    L: Line,
{
    let mut min_distance    = f64::MAX;
    let mut min_t           = 0.0;

    // Walk the curve in increments of .1 pixel
    for t in -200..=300 {
        let t = (t as f64) / 100.0;
        let distance = line.point_at_pos(t).distance_to(point);

        if distance < min_distance {
            min_distance = distance;
            min_t = t;
        }
    }

    min_t
}

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
    let perpendicular_line  = (Coord2(4.0, 5.0), Coord2(4.0 - (9.0-5.0), 5.0 + (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_origin         = perpendicular_line.point_at_pos(2.0);
    let origin_nearest      = line.nearest_point(&from_origin);

    assert!(origin_nearest.distance_to(&line.point_at_pos(nearest_t_value_iteration(&line, &from_origin))) < 0.1);
    assert!(origin_nearest.distance_to(&Coord2(4.0, 5.0)) < 0.01);
}

#[test]
fn nearest_1() {
    let line            = (Coord2(0.0, 0.0), Coord2(10.0, 7.0));
    let point           = Coord2(1.0, 5.0);
    let line_near       = line.nearest_point(&point);
    let iter_t          = nearest_t_value_iteration(&line, &point);
    let iter_point      = line.point_at_pos(iter_t);

    assert!(line_near.distance_to(&iter_point) < 0.1);
}

#[test]
fn nearest_2() {
    // Create a line and a perpendicular line that goes a point at t=0.3
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let point               = line.point_at_pos(0.3);
    let perpendicular_line  = (point, Coord2(point.x() - (9.0-5.0), point.y() + (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_point          = perpendicular_line.point_at_pos(2.0);
    let point_nearest       = line.nearest_point(&from_point);

    assert!((nearest_t_value_iteration(&line, &point) - 0.3).abs() < 0.01);
    assert!(point_nearest.distance_to(&line.point_at_pos(nearest_t_value_iteration(&line, &from_point))) < 0.1);
    assert!(point_nearest.distance_to(&point) < 0.01);
}

#[test]
fn nearest_t_2() {
    // Create a line and a perpendicular line that goes through the origin
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let point               = line.point_at_pos(0.3);
    let perpendicular_line  = (point, Coord2(point.x() - (9.0-5.0), point.y() + (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_point          = perpendicular_line.point_at_pos(2.0);
    let point_nearest       = line.nearest_pos(&from_point);

    assert!((point_nearest-0.3).abs() < 0.01);
}

#[test]
fn nearest_origin_t() {
    // Create a line and a perpendicular line that goes through the origin
    let line                = (Coord2(4.0, 5.0), Coord2(7.0, 9.0));
    let perpendicular_line  = (Coord2(4.0, 5.0), Coord2(4.0 - (9.0-5.0), 5.0 + (7.0-4.0)));

    // For any point on the perpendicular line, the nearest point should be the origin point
    let from_origin         = perpendicular_line.point_at_pos(2.0);
    let origin_nearest      = line.nearest_pos(&from_origin);

    assert!((origin_nearest - nearest_t_value_iteration(&line, &from_origin)).abs() < 0.01);
    assert!(origin_nearest.abs() < 0.01);
}
