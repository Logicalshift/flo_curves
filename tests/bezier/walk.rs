use flo_curves::bezier::*;

#[test]
fn uneven_walk_1() {
    let c           = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let sections    = walk_curve_unevenly(&c, 10).collect::<Vec<_>>();

    assert!(sections.len() == 10);
    assert!(sections[0].original_curve_t_values() == (0.0, 0.1));

    for section_num in 0..10 {
        let expected_t_min = (section_num as f64)/10.0;
        let expected_t_max = (section_num as f64)/10.0 + 0.1;

        let (actual_t_min, actual_t_max) = sections[section_num].original_curve_t_values();

        assert!((actual_t_min - expected_t_min).abs() < 0.0001);
        assert!((actual_t_max - expected_t_max).abs() < 0.0001);
    }
}

#[test]
fn even_walk_1() {
    let c                   = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    for section in sections.iter() {
        assert!((chord_length(section)-1.0).abs() <= 0.1);
        total_length += chord_length(section);
    }

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

#[test]
fn even_walk_2() {
    let c                   = Curve::from_points(Coord2(170.83203, 534.28906), (Coord2(140.99219, 492.1289), Coord2(0.52734375, 478.67188)), Coord2(262.95313, 533.2656));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    for section in sections.iter() {
        assert!((chord_length(section)-1.0).abs() <= 0.1);
        total_length += chord_length(section);
    }

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

#[test]
fn even_walk_3() {
    let c                   = Curve::from_points(Coord2(987.7637, 993.9645), (Coord2(991.1699, 994.0231), Coord2(1043.5605, 853.44885)), Coord2(1064.9473, 994.277));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    for section in sections.iter() {
        assert!((chord_length(section)-1.0).abs() <= 0.1);
        total_length += chord_length(section);
    }

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

/*
#[test]
fn even_walk_4() {
    let c                   = Curve::from_points(Coord2(222.37538991853827, 99.16540392815092), (Coord2(224.47523575883392, 100.31557953334229), Coord2(223.19303980237945, 101.8075327562316)), Coord2(225.42363518033414, 99.716688142193));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    for section in sections.iter() {
        assert!((chord_length(section)-1.0).abs() <= 0.1);
        total_length += chord_length(section);
    }

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}
*/
