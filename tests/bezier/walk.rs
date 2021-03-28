use flo_curves::bezier::*;

#[test]
fn walk_1() {
    let c           = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let sections    = walk_curve_uneven(&c, 10).collect::<Vec<_>>();

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
