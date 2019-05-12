extern crate flo_curves;

use flo_curves::*;

#[test]
fn can_get_distance_between_points() {
    assert!(Coord2(1.0, 1.0).distance_to(&Coord2(1.0, 8.0)) == 7.0);
}

#[test]
fn can_find_unit_vector() {
    assert!(Coord2(0.0, 1.0).to_unit_vector() == Coord2(0.0, 1.0));
    assert!(Coord2(0.0, 2.0).to_unit_vector() == Coord2(0.0, 1.0));

    assert!(f64::abs(Coord2(4.0, 2.0).to_unit_vector().distance_to(&Coord2(0.0, 0.0))-1.0) < 0.01);
}

#[test]
fn unit_vector_of_0_0_is_0_0() {
    assert!(Coord2(0.0, 0.0).to_unit_vector() == Coord2(0.0, 0.0));
}

#[test]
fn can_get_dot_product() {
    assert!(Coord2(2.0,1.0).dot(&Coord2(3.0, 4.0)) == 10.0);
}

#[test]
fn round_to_hundredths() {
    assert!(Coord2(1.1111, 2.2222).round(0.01) == Coord2(1.11, 2.22));
}

#[test]
fn round_to_units() {
    assert!(Coord2(1.1111, 2.2222).round(1.0) == Coord2(1.0, 2.0));
}

#[test]
fn round_up_to_units() {
    assert!(Coord2(1.1111, 2.5555).round(1.0) == Coord2(1.0, 3.0));
}
