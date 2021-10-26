use flo_curves::*;
use flo_curves::line;
use flo_curves::bezier;

#[test]
fn find_intersection_on_straight_line_not_middle() {
    // Cross that intersects at (5.0, 5.0)
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(0.0, 0.0), Coord2(13.0, 13.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(9.0, 1.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    println!("{:?} {:?}", intersections, intersections.iter().map(|(t1, t2)| (curve1.point_at_pos(*t1), curve2.point_at_pos(*t2))).collect::<Vec<_>>());
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn find_intersection_on_straight_line_middle() {
    // Cross that intersects at (5.0, 5.0)
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(0.0, 0.0), Coord2(10.0, 10.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 0.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    println!("{:?} {:?}", intersections, intersections.iter().map(|(t1, t2)| (curve1.point_at_pos(*t1), curve2.point_at_pos(*t2))).collect::<Vec<_>>());
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn find_intersection_on_straight_line_start() {
    // Intersection at the start of two curves
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(5.0, 5.0), Coord2(10.0, 10.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(5.0, 5.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersections[0].0 < 0.01);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersections[0].1 < 0.01);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn find_intersection_on_straight_line_end_1() {
    // Intersection at the start of two curves
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 10.0), Coord2(5.0, 5.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(0.0, 10.0), Coord2(5.0, 5.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersections[0].0 > 0.99);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersections[0].1 > 0.99);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn find_intersection_on_straight_line_end_to_start_1() {
    // Intersection at the start of two curves
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 10.0), Coord2(5.0, 5.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(5.0, 5.0), Coord2(0.0, 10.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersections[0].0 > 0.99);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersections[0].1 < 0.01);
    assert!(intersect_point.distance_to(&Coord2(5.0, 5.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn find_intersection_on_line_end_to_end_2() {
    // Intersection that should be found in self_collide_removes_shared_point_2 in the graph_path tests
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(1.0, 5.0), Coord2(3.0, 3.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(5.0, 5.0), Coord2(3.0, 3.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(3.0, 3.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersect_point.distance_to(&Coord2(3.0, 3.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn find_intersection_on_line_end_to_end_3() {
    // TODO: this fails at the moment (the issue is that as the ray is collinear we get the wrong t values for the intersection point)

    // Intersection that should be found in self_collide_removes_shared_point_1 in the graph_path tests
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(1.0, 5.0), Coord2(3.0, 3.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(5.0, 1.0), Coord2(3.0, 3.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(3.0, 3.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersect_point.distance_to(&Coord2(3.0, 3.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn solve_for_end_1() {
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(1.0, 5.0), Coord2(3.0, 3.0)));
    let end_pos = bezier::solve_curve_for_t(&curve1, &Coord2(3.0, 3.0));

    assert!(end_pos.is_some());
    assert!((end_pos.unwrap() - 1.0).abs() < 0.01);
}

#[test]
fn solve_for_end_2() {
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(5.0, 1.0), Coord2(3.0, 3.0)));
    let end_pos = bezier::solve_curve_for_t(&curve1, &Coord2(3.0, 3.0));

    assert!(end_pos.is_some());
    assert!((end_pos.unwrap() - 1.0).abs() < 0.01);
}

#[test]
fn find_intersection_on_line_end_to_start_2() {
    // Reverse of the intersection that should be found in self_collide_removes_shared_point_2 in the graph_path tests
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(1.0, 5.0), Coord2(3.0, 3.0)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(3.0, 3.0), Coord2(5.0, 5.0)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    assert!(intersections.len() != 0);

    let intersect_point = curve1.point_at_pos(intersections[0].0);
    assert!(intersect_point.distance_to(&Coord2(3.0, 3.0)) < 0.1);

    let intersect_point = curve2.point_at_pos(intersections[0].1);
    assert!(intersect_point.distance_to(&Coord2(3.0, 3.0)) < 0.1);

    assert!(intersections.len() == 1);
}

#[test]
fn find_intersection_on_straight_line_near_end() {
    // Intersection at the start of two curves
    let curve1  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(10.0, 10.0), Coord2(4.9, 5.1)));
    let curve2  = line::line_to_bezier::<_, bezier::Curve<_>>(&(Coord2(0.0, 10.0), Coord2(5.1, 4.9)));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?} {:?}", intersections, intersections.iter().map(|(t1, t2)| (curve1.point_at_pos(*t1), curve2.point_at_pos(*t2))).collect::<Vec<_>>());

    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
}

#[test]
fn find_intersections_on_curve() {
    //
    // Two curves with three intersections
    //
    // Intersection points approx:
    //
    // Coord2(81.78, 109.88)
    // Coord2(133.16, 167.13)
    // Coord2(179.87, 199.67)
    //
    let curve1  = bezier::Curve::from_points(Coord2(10.0, 100.0), (Coord2(90.0, 30.0), Coord2(40.0, 140.0)), Coord2(220.0, 220.0));
    let curve2  = bezier::Curve::from_points(Coord2(5.0, 150.0), (Coord2(180.0, 20.0), Coord2(80.0, 250.0)), Coord2(210.0, 190.0));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.1);
    println!("{:?} {:?}", intersections, intersections.iter().map(|(t1, t2)| (curve1.point_at_pos(*t1), curve2.point_at_pos(*t2))).collect::<Vec<_>>());

    // All intersections should be approximately the same location
    for intersect in intersections.iter() {
        let point1 = curve1.point_at_pos(intersect.0);
        let point2 = curve2.point_at_pos(intersect.1);

        assert!(point1.distance_to(&point2) < 1.0);
        assert!(point1.distance_to(&point2) < 0.1);
    }

    // Three intersections
    assert!(intersections.len() == 3);
}

#[test]
fn intersections_with_overlapping_curves_1() {
    let curve1 = bezier::Curve::from_points(Coord2(346.69864, 710.2048), (Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306));
    let curve2 = bezier::Curve::from_points(Coord2(346.69864, 710.2048), (Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);

    println!("{:?}", intersections);

    assert!(intersections.len() == 2);
}

#[test]
fn intersections_with_overlapping_curves_2() {
    let curve1 = bezier::Curve::from_points(Coord2(346.69864, 710.2048), (Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306));
    let curve2 = bezier::Curve::from_points(Coord2(346.69864, 710.2048), (Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306));
    let curve2 = bezier::Curve::from_curve(&curve2.section(0.2, 0.6));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);

    println!("{:?}", intersections);

    assert!(intersections.len() == 2);
}

#[test]
fn intersections_with_overlapping_curves_3() {
    let curve1 = bezier::Curve::from_points(Coord2(346.69864, 710.2048), (Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306));
    let curve2 = bezier::Curve::from_points(Coord2(346.69864, 710.2048), (Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306));
    let curve1 = bezier::Curve::from_curve(&curve1.section(0.2, 0.6));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);

    println!("{:?}", intersections);

    assert!(intersections.len() == 2);
}

#[test]
fn intersections_with_nearby_curves_1() {
    let curve1 = bezier::Curve::from_points(Coord2(346.69864, 710.2048), (Coord2(350.41446, 706.8076), Coord2(353.61026, 702.4266)), Coord2(356.28525, 698.20306));
    let curve2 = bezier::Curve::from_points(Coord2(350.22574, 706.551), (Coord2(354.72943, 701.2933), Coord2(358.0882, 695.26)), Coord2(361.0284, 690.2511));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);

    println!("{:?}", intersections);

    assert!(intersections.len() <= 9);
}

#[test]
fn intersections_with_nearby_curves_2() {
    let curve1 = bezier::Curve::from_points(Coord2(305.86907958984375, 882.2529296875), (Coord2(305.41015625, 880.7345581054688), Coord2(303.0707092285156, 879.744140625)), Coord2(298.0640869140625, 875.537353515625));
    let curve2 = bezier::Curve::from_points(Coord2(302.7962341308594, 879.1681518554688), (Coord2(299.5769348144531, 876.8582763671875), Coord2(297.1976318359375, 874.7939453125)), Coord2(301.4282531738281, 878.26220703125));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);

    assert!(intersections.len() <= 9);
}

#[test]
fn intersections_with_nearby_curves_3() {
    let curve1 = bezier::Curve::from_points(Coord2(304.6919250488281, 880.6288452148438), (Coord2(304.2330017089844, 879.1104736328125), Coord2(301.8935546875, 878.1200561523438)), Coord2(296.8869323730469, 873.9132690429688));
    let curve2 = bezier::Curve::from_points(Coord2(301.61907958984375, 877.5440673828125), (Coord2(300.2510986328125, 876.6381225585938), Coord2(298.3997802734375, 875.2341918945313)), Coord2(296.0204772949219, 873.1698608398438));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);

    // assert!(intersections.len() <= 9);
}

#[test]
fn intersections_with_nearby_curves_4() {
    let curve1 = bezier::Curve::from_points(Coord2(436.15716552734375, 869.3236083984375), (Coord2(444.5263671875, 869.2921752929688), Coord2(480.9628601074219, 854.3709106445313)), Coord2(490.6786804199219, 849.5614624023438));
    let curve2 = bezier::Curve::from_points(Coord2(462.5539855957031, 861.322021484375), (Coord2(462.4580078125, 861.4293823242188), Coord2(462.3710021972656, 861.5908813476563)), Coord2(462.3448486328125, 861.8137817382813));

    let intersections   = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);

    assert!(intersections.len() <= 9);
}

#[test]
fn intersection_curve_1() {
    let curve1 = bezier::Curve::from_points(Coord2(252.08901977539063, 676.4180908203125), (Coord2(244.0195770263672, 679.6658935546875), Coord2(244.11508178710938, 682.8816528320313)), Coord2(244.31190490722656, 686.1041259765625));
    let curve2 = bezier::Curve::from_points(Coord2(244.31190490722656, 686.1041259765625), (Coord2(250.65411376953125, 661.4817504882813), Coord2(255.51109313964844, 635.5418701171875)), Coord2(265.2398376464844, 618.4223022460938));

    let intersections = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() != 1);
    assert!(intersections.len() == 2);

    assert!(curve1.point_at_pos(intersections[0].0).distance_to(&curve2.point_at_pos(intersections[0].1)) < 0.01);
    assert!(curve1.point_at_pos(intersections[1].0).distance_to(&curve2.point_at_pos(intersections[1].1)) < 0.01);

    let intersections = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 2);

    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&curve1.point_at_pos(intersections[0].1)) < 0.01);
    assert!(curve2.point_at_pos(intersections[1].0).distance_to(&curve1.point_at_pos(intersections[1].1)) < 0.01);
}

#[test]
fn intersection_curve_2() {
    let curve1 = bezier::Curve::from_points(Coord2(248.42221069335938, 678.5138549804688), (Coord2(240.33773803710938, 703.49462890625), Coord2(246.20928955078125, 728.5226440429688)), Coord2(258.2634582519531, 745.7745361328125));
    let curve2 = bezier::Curve::from_points(Coord2(240.6450958251953, 688.1998901367188), (Coord2(248.51101684570313, 684.6644897460938), Coord2(248.41787719726563, 681.5728759765625)), Coord2(248.42221069335938, 678.5138549804688));

    let intersections = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() != 1);
    assert!(intersections.len() == 2);

    assert!(curve1.point_at_pos(intersections[0].0).distance_to(&curve2.point_at_pos(intersections[0].1)) < 0.01);
    assert!(curve1.point_at_pos(intersections[1].0).distance_to(&curve2.point_at_pos(intersections[1].1)) < 0.01);

    let intersections = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 2);

    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&curve1.point_at_pos(intersections[0].1)) < 0.01);
    assert!(curve2.point_at_pos(intersections[1].0).distance_to(&curve1.point_at_pos(intersections[1].1)) < 0.01);
}

#[test]
fn intersection_curve_3() {
    let curve1 = bezier::Curve::from_points(Coord2(377.8294677734375, 495.076904296875), (Coord2(380.0453796386719, 492.69927978515625), Coord2(381.98138427734375, 489.805419921875)), Coord2(383.61865234375, 486.40106201171875));
    let curve2 = bezier::Curve::from_points(Coord2(379.064697265625, 493.7556457519531), (Coord2(371.90069580078125, 491.9415588378906), Coord2(368.96783447265625, 493.451171875)), Coord2(366.3587951660156, 494.5915832519531));

    let intersections = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);

    assert!(curve1.point_at_pos(intersections[0].0).distance_to(&curve2.point_at_pos(intersections[0].1)) < 0.01);

    let intersections = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);

    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&curve1.point_at_pos(intersections[0].1)) < 0.01);
}

#[test]
fn intersection_curve_4() {
    let curve1 = bezier::Curve::from_points(Coord2(377.8294677734375, 495.076904296875), (Coord2(380.0453796386719, 492.69927978515625), Coord2(381.98138427734375, 489.805419921875)), Coord2(383.61865234375, 486.40106201171875));
    let curve2 = bezier::Curve::from_points(Coord2(379.064697265625, 493.7556457519531), (Coord2(371.3619079589844, 493.8326110839844), Coord2(366.50872802734375, 495.2229919433594)), Coord2(362.0657958984375, 496.14581298828125));

    let intersections = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);

    assert!(curve1.point_at_pos(intersections[0].0).distance_to(&curve2.point_at_pos(intersections[0].1)) < 0.01);

    let intersections = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);

    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&curve1.point_at_pos(intersections[0].1)) < 0.01);
}

#[test]
fn intersection_curve_5() {
    let curve1 = bezier::Curve::from_points(Coord2(379.064697265625, 493.7556457519531), (Coord2(371.90069580078125, 491.9415588378906), Coord2(368.96783447265625, 493.451171875)), Coord2(366.3587951660156, 494.5915832519531));
    let curve2 = bezier::Curve::from_points(Coord2(379.064697265625, 493.7556457519531), (Coord2(371.3619079589844, 493.8326110839844), Coord2(366.50872802734375, 495.2229919433594)), Coord2(362.0657958984375, 496.14581298828125));
    
    let intersections = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);

    assert!(curve1.point_at_pos(intersections[0].0).distance_to(&curve2.point_at_pos(intersections[0].1)) < 0.01);

    let intersections = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);

    assert!(curve2.point_at_pos(intersections[0].0).distance_to(&curve1.point_at_pos(intersections[0].1)) < 0.01);
}

#[test]
fn intersection_curve_6() {
    let curve1 = bezier::Curve::from_points(Coord2(608.7642211914063, 855.5934448242188), (Coord2(608.6810302734375, 855.288330078125), Coord2(608.5828857421875, 855.0850830078125)), Coord2(608.47265625, 855.011962890625));
    let curve2 = bezier::Curve::from_points(Coord2(608.81689453125, 855.5904541015625), (Coord2(608.7009887695313, 855.386474609375), Coord2(608.5858154296875, 855.193115234375)), Coord2(608.47265625, 855.011962890625));
    
    let intersections = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 1);
    
    let intersections = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);
    println!("{:?}", intersections);
    assert!(intersections.len() != 0);
    assert!(intersections.len() == 2);
}

#[test]
fn intersection_curve_7() {
    // Three curves that should intersect in two places, seems to be generating a missed collision when doing a self-collide
    // See `remove_interior_points_complex_1`: one of these curves seems to be producing only one collision, at least for the case that prompted that test
    let curve1 = bezier::Curve::from_points(Coord2(602.1428833007813, 859.0895385742188), (Coord2(607.4638061523438, 858.4710693359375), Coord2(614.4444580078125, 855.14404296875)), Coord2(608.3931884765625, 855.6187133789063));
    let curve2 = bezier::Curve::from_points(Coord2(608.3871459960938, 864.779541015625), (Coord2(609.6311645507813, 860.09716796875), Coord2(608.9767456054688, 852.512939453125)), Coord2(607.8642578125, 855.7709350585938));
    let curve3 = bezier::Curve::from_points(Coord2(611.2799682617188, 862.292236328125), (Coord2(610.607666015625, 857.74365234375), Coord2(606.7666625976563, 851.5074462890625)), Coord2(606.7361450195313, 853.9833984375));

    let intersections1 = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    let intersections2 = bezier::curve_intersects_curve_clip(&curve2, &curve3, 0.01);
    let intersections3 = bezier::curve_intersects_curve_clip(&curve1, &curve3, 0.01);

    println!("{:?}\n{:?}\n{:?}\n", intersections1, intersections2, intersections3);

    assert!(intersections1.len() == 2);
    assert!(intersections2.len() == 2);
    assert!(intersections3.len() == 2);

    let intersections1 = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);
    let intersections2 = bezier::curve_intersects_curve_clip(&curve3, &curve2, 0.01);
    let intersections3 = bezier::curve_intersects_curve_clip(&curve3, &curve1, 0.01);

    println!("{:?}\n{:?}\n{:?}\n", intersections1, intersections2, intersections3);

    assert!(intersections1.len() == 2);
    assert!(intersections2.len() == 2);
    assert!(intersections3.len() == 2);
}

#[test]
fn intersection_curve_8() {
    // This curve starts and ends at the same position
    let loop_curve  = bezier::Curve::from_points(Coord2(534.170654296875, 832.8574829101563), (Coord2(534.3781127929688, 832.078369140625), Coord2(534.73828125, 832.8485107421875)), Coord2(534.170654296875, 832.8574829101563));
    let curve2      = bezier::Curve::from_points(Coord2(534.3034057617188, 832.695068359375), (Coord2(534.2012536621094, 832.3760168457031), Coord2(534.2515673828125, 832.5331616210938)), Coord2(534.1509399414063, 832.2188720703125));

    let intersections1 = bezier::curve_intersects_curve_clip(&loop_curve, &curve2, 0.01);
    let intersections2 = bezier::curve_intersects_curve_clip(&curve2, &loop_curve, 0.01);

    println!("{:?}", intersections1);
    println!("{:?}", intersections2);

    assert!(intersections1.len() > 0);
    assert!(intersections2.len() > 0);

    assert!(intersections1.len() == 1);
    assert!(intersections2.len() == 1);

    assert!((intersections1[0].0 - intersections2[0].1).abs() < 0.001);
    assert!((intersections1[0].1 - intersections2[0].0).abs() < 0.001);
}

#[test]
fn intersection_curve_9() {
    let curve1 = bezier::Curve::from_points(Coord2(576.0272827148438, 854.4729614257813), (Coord2(575.8976440429688, 855.9894409179688), Coord2(576.928466796875, 870.5877685546875)), Coord2(577.4629516601563, 873.9253540039063));
    let curve2 = bezier::Curve::from_points(Coord2(576.0455932617188, 854.9674072265625), (Coord2(574.2247924804688, 855.3988037109375), Coord2(577.3884887695313, 863.1025390625)), Coord2(580.003662109375, 863.5904541015625));
    
    let intersections1 = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    let intersections2 = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);

    println!("{:?}", intersections1);
    println!("{:?}", intersections2);

    assert!(intersections1.len() > 0);
    assert!(intersections2.len() > 0);

    assert!(intersections1.len() == 2);
    assert!(intersections2.len() == 2);
}

#[test]
fn intersection_curve_10() {
    let curve1 = bezier::Curve { 
        start_point: Coord2(284.86767013759504, 712.1320343559642), 
        end_point: Coord2(709.1317388495236, 712.1320343559643), 
        control_points: (Coord2(402.02495766297596, 829.2893218813454), Coord2(591.9744513241425, 829.2893218813454)) 
    };
    let curve2 = bezier::Curve { 
        start_point: Coord2(290.8682611504764, 712.1320343559642), 
        end_point: Coord2(715.132329862405, 712.1320343559643), 
        control_points: (Coord2(408.02554867585735, 829.2893218813454), Coord2(597.9750423370239, 829.2893218813454)) 
    };

    let intersections1 = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    assert!(intersections1.len() > 0);

    let intersections2 = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);

    println!("{:?}", intersections1);
    println!("{:?}", intersections2);

    assert!(intersections1.len() > 0);
    assert!(intersections2.len() > 0);

    assert!(intersections1.len() == 1);
    assert!(intersections2.len() == 1);

    assert!(Coord2::from(intersections1[0]).distance_to(&Coord2::from(intersections2[0])) < 0.02);
}

// (0.49236699497857783, 0.5065132298924669) (0.47428429377321385, 0.4934869656848157
// Intersection at 0.5064950379631311, 0.49346879469352817 (very close to the end of this range)
#[test]
fn intersection_curve_11() {
    // Tries to eliminate the subdivisions from intersection_curve_10 so will more reliably fail if other changes are made to
    // the intersection algorithm
    let curve1 = bezier::Curve { 
        start_point: Coord2(284.86767013759504, 712.1320343559642), 
        end_point: Coord2(709.1317388495236, 712.1320343559643), 
        control_points: (Coord2(402.02495766297596, 829.2893218813454), Coord2(591.9744513241425, 829.2893218813454)) 
    };
    let curve2 = bezier::Curve { 
        start_point: Coord2(290.8682611504764, 712.1320343559642), 
        end_point: Coord2(715.132329862405, 712.1320343559643), 
        control_points: (Coord2(408.02554867585735, 829.2893218813454), Coord2(597.9750423370239, 829.2893218813454)) 
    };

    let curve1 = curve1.section(0.49236699497857783, 0.5065132298924669);
    let curve2 = curve2.section(0.47428429377321385, 0.4934869656848157);

    let intersections1 = bezier::curve_intersects_curve_clip(&curve1, &curve2, 0.01);
    assert!(intersections1.len() > 0);

    let intersections2 = bezier::curve_intersects_curve_clip(&curve2, &curve1, 0.01);

    println!("{:?}", intersections1);
    println!("{:?}", intersections2);

    assert!(intersections1.len() > 0);
    assert!(intersections2.len() > 0);

    assert!(intersections1.len() == 1);
    assert!(intersections2.len() == 1);

    let pos1 = curve1.point_at_pos(intersections1[0].0);
    let pos2 = curve2.point_at_pos(intersections1[0].1);

    let pos3 = curve2.point_at_pos(intersections2[0].0);
    let pos4 = curve1.point_at_pos(intersections2[0].1);

    println!("{:?}", pos1.distance_to(&pos2));
    println!("{:?}", pos3.distance_to(&pos4));
    println!("{:?}", pos3.distance_to(&pos2));
    println!("{:?}", pos1.distance_to(&pos4));

    assert!(pos1.distance_to(&pos2) < 0.02);
    assert!(pos3.distance_to(&pos4) < 0.02);
    assert!(pos2.distance_to(&pos3) < 0.02);
    assert!(pos1.distance_to(&pos4) < 0.02);
}

// Fragment: Curve { start_point: Coord2(503.12144515225805, 515.6925644864517), end_point: Coord2(558.5270966048384, 794.2355841209692), control_points: (Coord2(521.5881487814031, 608.5309529306364), Coord2(540.0548524105482, 701.369341374821)) }
// Remaining: Curve { start_point: Coord2(522.6328735351563, 613.7830200195313), end_point: Coord2(582.0244140625, 582.0244140625), control_points: (Coord2(544.3945922851563, 609.47509765625), Coord2(565.159912109375, 598.8888549804688)) }
#[test]
fn intersection_very_close_to_start_1() {
    // One section here is a line, so we can find that the start point of the 'remaining' section is very close to that line (enough that it should produce an intersection)
    // Source of this is a case where the 'remaining' section is slightly rounded due to a conversion to f32 and back to f64
    let fragment    = bezier::Curve { start_point: Coord2(503.12144515225805, 515.6925644864517), end_point: Coord2(558.5270966048384, 794.2355841209692), control_points: (Coord2(521.5881487814031, 608.5309529306364), Coord2(540.0548524105482, 701.369341374821)) };
    let remaining   = bezier::Curve { start_point: Coord2(522.6328735351563, 613.7830200195313), end_point: Coord2(582.0244140625, 582.0244140625), control_points: (Coord2(544.3945922851563, 609.47509765625), Coord2(565.159912109375, 598.8888549804688)) };

    let intersections1 = bezier::curve_intersects_curve_clip(&fragment, &remaining, 0.01);
    let intersections2 = bezier::curve_intersects_curve_clip(&remaining, &fragment, 0.01);

    println!("{:?}", intersections1);
    println!("{:?}", intersections2);

    assert!(intersections1.len() > 0);
    assert!(intersections2.len() > 0);

    assert!(intersections1.len() == 1);
    assert!(intersections2.len() == 1);

    let pos1 = fragment.point_at_pos(intersections1[0].0);
    let pos2 = remaining.point_at_pos(intersections1[0].1);

    let pos3 = remaining.point_at_pos(intersections2[0].0);
    let pos4 = fragment.point_at_pos(intersections2[0].1);

    println!("{:?}", pos1.distance_to(&pos2));
    println!("{:?}", pos3.distance_to(&pos4));
    println!("{:?}", pos3.distance_to(&pos2));
    println!("{:?}", pos1.distance_to(&pos4));

    assert!(pos1.distance_to(&pos2) < 0.02);
    assert!(pos3.distance_to(&pos4) < 0.02);
    assert!(pos2.distance_to(&pos3) < 0.02);
    assert!(pos1.distance_to(&pos4) < 0.02);
}

#[test]
fn solve_t_close_to_start() {
    use flo_curves::bezier::*;

    // Same curve as above, but we try to solve the closest point for one curve against another
    let fragment    = bezier::Curve { start_point: Coord2(503.12144515225805, 515.6925644864517), end_point: Coord2(558.5270966048384, 794.2355841209692), control_points: (Coord2(521.5881487814031, 608.5309529306364), Coord2(540.0548524105482, 701.369341374821)) };
    let remaining   = bezier::Curve { start_point: Coord2(522.6328735351563, 613.7830200195313), end_point: Coord2(582.0244140625, 582.0244140625), control_points: (Coord2(544.3945922851563, 609.47509765625), Coord2(565.159912109375, 598.8888549804688)) };

    // In this case, the fragment is linear (however, as we're solving for a point close to a curve, this shouldn't be necessary always)
    assert!(fragment.characteristics() == CurveCategory::Linear);

    // The start point of 'remaining' is very close to fragment (in fact, the distance between the two is down to floating-point imprecision more than anything)
    let t_remaining = 0.0;

    // Should be able to solve for this point on the remaining curve
    let t_fragment  = fragment.t_for_point(&remaining.start_point).expect("t value");
    let t_point     = fragment.point_at_pos(t_fragment);
    let t_distance  = t_point.distance_to(&remaining.point_at_pos(t_remaining));

    // The above test should be able to solve this value to at least this precision level (t_remaining = 0.0, t_fragment = as above)
    assert!(t_distance < 0.02);
}
