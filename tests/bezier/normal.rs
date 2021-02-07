use flo_curves::*;
use flo_curves::bezier;
use flo_curves::bezier::{NormalCurve};

#[test]
fn normal_for_line_is_straight_up() {
    let line    = bezier::Curve::from_points(Coord2(0.0,0.0), (Coord2(3.0, 0.0), Coord2(7.0, 0.0)), Coord2(10.0, 0.0));
    let normal  = line.normal_at_pos(0.5);

    // Normal should be a line facing up
    assert!(normal.x().abs() < 0.01);
    assert!(normal.y() > 0.01);
}

#[test]
fn normal_for_vert_short_line_is_straight_up() {
    let line    = bezier::Curve::from_points(Coord2(0.0,0.0), (Coord2(0.0000000003, 0.0), Coord2(0.0000000007, 0.0)), Coord2(0.0000000010, 0.0));
    let normal  = line.normal_at_pos(0.5);

    // Normals usually aren't unit vectors, but will produce very small values for very short lines
    let normal = normal.to_unit_vector();

    // Normal should be a line facing up
    assert!(normal.x().abs() < 0.01);
    assert!(normal.y() > 0.01);
}

#[test]
fn normal_for_vert_short_diagonal_line_is_straight_up() {
    let line    = bezier::Curve::from_points(Coord2(0.0,0.0), (Coord2(0.0000000003, 0.0000000003), Coord2(0.0000000007, 0.0000000007)), Coord2(0.0000000010, 0.0000000010));
    let normal  = line.normal_at_pos(0.5);

    // Normals usually aren't unit vectors, but will produce very small values for very short lines
    let normal = normal.to_unit_vector();

    // Normal should be a 45 degree diagonal line
    assert!(normal.x() < -0.01);
    assert!(normal.y() > 0.01);
    assert!((-normal.x()-normal.y()).abs() < 0.01);
}

#[test]
fn normal_for_point() {
    let line    = bezier::Curve::from_points(Coord2(0.0,0.0), (Coord2(0.0, 0.0), Coord2(0.0, 0.0)), Coord2(0.0, 0.0));
    let normal  = line.normal_at_pos(0.5);

    // Normal should be the (0,0) vector (points don't have normals)
    assert!(normal.x().abs() < 0.0001);
    assert!(normal.y() < 0.0001);
}

#[test]
fn normal_at_start_of_curve_matches_control_points() {
    let line    = bezier::Curve::from_points(Coord2(0.0,0.0), (Coord2(0.0, 1.0), Coord2(7.0, 0.0)), Coord2(10.0, 0.0));
    let normal  = line.normal_at_pos(0.0);

    // Normal should be a facing left
    assert!(normal.x() < 0.0);
    assert!(normal.y().abs() < 0.01);
}
