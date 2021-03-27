use flo_curves::bezier::*;

///
/// Estimates a curve's length by subdividing it a lot
///
fn subdivide_length<Curve: BezierCurve>(curve: &Curve) -> f64 {
    let mut length = 0.0;

    for division in 0..1000 {
        let division    = (division as f64)/1000.0;
        let subsection  = curve.section(division, division + (1.0/1000.0));
        length          += chord_length(&subsection);
    }

    length
}

#[test]
fn measure_length_1() {
    let c               = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let by_subdivision  = subdivide_length(&c);
    let by_measuring    = curve_length(&c, 0.5);

    assert!((by_measuring - by_subdivision).abs() < 1.0);
}

#[test]
fn measure_length_2() {
    let c               = Curve::from_points(Coord2(987.7637, 993.9645), (Coord2(991.1699, 994.0231), Coord2(1043.5605, 853.44885)), Coord2(1064.9473, 994.277));
    let by_subdivision  = subdivide_length(&c);
    let by_measuring    = curve_length(&c, 0.5);

    assert!((by_measuring - by_subdivision).abs() < 1.0);
}

#[test]
fn measure_length_3() {
    let c               = Curve::from_points(Coord2(170.83203, 534.28906), (Coord2(140.99219, 492.1289), Coord2(0.52734375, 478.67188)), Coord2(262.95313, 533.2656));
    let by_subdivision  = subdivide_length(&c);
    let by_measuring    = curve_length(&c, 0.5);

    assert!((by_measuring - by_subdivision).abs() < 1.0);
}

#[test]
fn measure_length_4() {
    let c               = Curve::from_points(Coord2(170.83203, 534.28906), (Coord2(35.15625, 502.65625), Coord2(0.52734375, 478.67188)), Coord2(262.95313, 533.2656));
    let by_subdivision  = subdivide_length(&c);
    let by_measuring    = curve_length(&c, 0.5);

    assert!((by_measuring - by_subdivision).abs() < 1.0);
}
