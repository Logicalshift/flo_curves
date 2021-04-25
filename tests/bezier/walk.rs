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
    let mut last_t          = 0.0;
    for section in sections.iter() {
        let (_, t_max) = section.original_curve_t_values();
        assert!(t_max > last_t);
        last_t = t_max;

        assert!((chord_length(section)-1.0).abs() <= 0.1 || (t_max >= 1.0 && chord_length(section)-1.0 <= 0.0));
        total_length += chord_length(section);
    }

    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

#[test]
fn even_walk_2() {
    let c                   = Curve::from_points(Coord2(170.83203, 534.28906), (Coord2(140.99219, 492.1289), Coord2(0.52734375, 478.67188)), Coord2(262.95313, 533.2656));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    let mut last_t          = 0.0;
    for section in sections.iter() {
        let (_, t_max) = section.original_curve_t_values();
        assert!(t_max > last_t);
        last_t = t_max;

        assert!((chord_length(section)-1.0).abs() <= 0.1 || (t_max >= 1.0 && chord_length(section)-1.0 <= 0.0));
        total_length += chord_length(section);
    }

    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

#[test]
fn even_walk_3() {
    let c                   = Curve::from_points(Coord2(987.7637, 993.9645), (Coord2(991.1699, 994.0231), Coord2(1043.5605, 853.44885)), Coord2(1064.9473, 994.277));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    let mut last_t          = 0.0;
    for section in sections.iter() {
        let (_, t_max) = section.original_curve_t_values();
        assert!(t_max > last_t);
        last_t = t_max;

        assert!((chord_length(section)-1.0).abs() <= 0.1 || (t_max >= 1.0 && chord_length(section)-1.0 <= 0.0));
        total_length += chord_length(section);
    }

    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

#[test]
fn even_walk_4() {
    let c                   = Curve::from_points(Coord2(222.37538991853827, 99.16540392815092), (Coord2(224.47523575883392, 100.31557953334229), Coord2(223.19303980237945, 101.8075327562316)), Coord2(225.42363518033414, 99.716688142193));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    let mut last_t          = 0.0;
    for section in sections.iter() {
        let (_, t_max) = section.original_curve_t_values();
        assert!(t_max > last_t);
        last_t = t_max;

        assert!((chord_length(section)-1.0).abs() <= 0.1 || (t_max >= 1.0 && chord_length(section)-1.0 <= 0.0));
        total_length += chord_length(section);
    }

    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

#[test]
fn even_walk_5() {
    let c                   = Curve::from_points(Coord2(128.51366414207797, 100.43540868606826), (Coord2(128.8517120419268, 100.53996562501626), Coord2(131.79687993559304, 99.36123524249854)), Coord2(131.8239019605053, 99.36980615298116));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    let mut last_t          = 0.0;
    for section in sections.iter() {
        println!("{:?} {:?}", chord_length(section)-1.0, section.original_curve_t_values());

        let (_, t_max) = section.original_curve_t_values();
        assert!(t_max > last_t);
        last_t = t_max;

        assert!((chord_length(section)-1.0).abs() <= 0.1 || (t_max >= 1.0 && chord_length(section)-1.0 <= 0.0));
        total_length += chord_length(section);
    }

    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}

#[test]
fn even_walk_6() {
    let c                   = Curve::from_points(Coord2(771.375, 195.0959930419922), (Coord2(771.375, 195.0959930419922), Coord2(629.2169799804688, 161.80499267578125)), Coord2(622.0430297851563, 160.3459930419922));
    let sections            = walk_curve_evenly(&c, 2.0, 0.5).collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    let mut last_t          = 0.0;
    for section in sections.iter() {
        println!("{:?} {:?}", chord_length(section)-2.0, section.original_curve_t_values());

        let (_, t_max) = section.original_curve_t_values();
        assert!(t_max > last_t);
        last_t = t_max;

        assert!((chord_length(section)-2.0).abs() <= 0.5 || (t_max >= 1.0 && chord_length(section)-2.0 <= 0.0));
        total_length += chord_length(section);
    }

    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 16.0);
}

#[test]
fn even_walk_point() {
    let c           = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(412.0, 500.0)), Coord2(412.0, 500.0));
    let sections    = walk_curve_evenly(&c, 1.0, 0.1).collect::<Vec<_>>();

    assert!(sections.len() == 1);
    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);
}

#[test]
fn varying_walk_1() {
    let c                   = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let sections            = walk_curve_evenly(&c, 1.0, 0.1)
        .vary_by(vec![1.0, 2.0, 3.0].into_iter().cycle())
        .collect::<Vec<_>>();
    let actual_length       = curve_length(&c, 0.1);

    let mut total_length    = 0.0;
    let mut last_t          = 0.0;
    let mut expected_length = vec![1.0, 2.0, 3.0].into_iter().cycle();
    for section in sections.iter().take(sections.len()-1) {
        let (_, t_max) = section.original_curve_t_values();
        assert!(t_max > last_t);
        last_t = t_max;

        let expected_length = expected_length.next().unwrap();
        assert!((chord_length(section)-expected_length).abs() <= 0.1);
        total_length += chord_length(section);
    }

    assert!(sections[sections.len()-1].original_curve_t_values().1 == 1.0);

    println!("{:?}", (total_length-actual_length).abs());
    assert!((total_length-actual_length).abs() < 4.0);
}
