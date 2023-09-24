use flo_curves::line::*;

use std::f64;

#[test]
fn angle_on_x_axis_is_0() {
    let angle = (Coord2(1.0, 1.0), Coord2(2.0, 1.0)).angle();

    assert!((angle-0.0).abs() < 0.01, "Angle should be 0.0 but is {}", angle);
}

#[test]
fn angle_on_x_axis_reversed_is_pi() {
    let angle = (Coord2(2.0, 1.0), Coord2(1.0, 1.0)).angle();

    assert!((angle-f64::consts::PI).abs() < 0.01, "Angle should be 0.0 but is {}", angle);
}

#[test]
fn angle_on_y_axis_is_half_pi() {
    let angle = (Coord2(1.0, 1.0), Coord2(1.0, 2.0)).angle();

    assert!((angle-f64::consts::PI/2.0).abs() < 0.01, "Angle should be pi/2 but is {}", angle);
}

#[test]
fn angle_on_y_axis_reversed_is_three_halfs_pi() {
    let angle = (Coord2(1.0, 2.0), Coord2(1.0, 1.0)).angle();

    assert!((angle-3.0*f64::consts::PI/2.0).abs() < 0.01, "Angle should be 3*pi/2 but is {}", angle);
}

#[test]
fn angle_between_45_degree_lines_1() {
    let line1 = (Coord2(1.0, 1.0), Coord2(0.0, 2.0));
    let line2 = (Coord2(1.0, 1.0), Coord2(2.0, 2.0));

    let angle_between = line1.angle_to(&line2);

    assert!((angle_between-f64::consts::PI/2.0).abs() < 0.01, "Angle should be pi/2 but is {}", angle_between);
}

#[test]
fn angle_between_45_degree_lines_2() {
    let line1 = (Coord2(1.0, 1.0), Coord2(2.0, 2.0));
    let line2 = (Coord2(1.0, 1.0), Coord2(0.0, 2.0));

    let angle_between = line1.angle_to(&line2);

    assert!((angle_between-3.0*f64::consts::PI/2.0).abs() < 0.01, "Angle should be 3*pi/2 but is {}", angle_between);
}
