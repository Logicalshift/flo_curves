use flo_curves::*;
use flo_curves::arc::*;
use flo_curves::bezier::path::*;

#[test]
fn subtract_circles() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(7.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Combine them
    let combined_circles = path_sub::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    assert!(combined_circles.len() == 1);

    // All points should be on either circle, and two should be on both
    let points = combined_circles[0].points().map(|(_, _, end_point)| end_point);

    let mut num_points_on_circle1   = 0;
    let mut num_points_on_circle2   = 0;
    let mut num_points_on_both      = 0;

    for point in points {
        let distance_to_circle1 = Coord2(5.0, 5.0).distance_to(&point);
        let distance_to_circle2 = Coord2(7.0, 5.0).distance_to(&point);

        // Must be on either circle
        assert!((distance_to_circle1-4.0).abs() < 0.01 || (distance_to_circle2-4.0).abs() < 0.01);

        println!("{:?} {:?} {:?}", point, distance_to_circle1, distance_to_circle2);

        if (distance_to_circle1-4.0).abs() < 0.01 && (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_both += 1 }
        else if (distance_to_circle1-4.0).abs() < 0.01 { num_points_on_circle1 += 1 }
        else if (distance_to_circle2-4.0).abs() < 0.01 { num_points_on_circle2 += 1 }
    }

    println!("{:?} {:?} {:?}", num_points_on_circle1, num_points_on_circle2, num_points_on_both);

    assert!(num_points_on_circle1 == 2);
    assert!(num_points_on_circle2 == 2);
    assert!(num_points_on_both == 2);
}

#[test]
fn create_doughnut() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    // Create a hole in the larger circle
    let combined_circles = path_sub::<_, _, SimpleBezierPath>(&vec![circle1], &vec![circle2], 0.01);

    assert!(combined_circles.len() == 2);
}

#[test]
fn erase_all() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    // Create a hole in the larger circle
    let combined_circles = path_sub::<_, _, SimpleBezierPath>(&vec![circle2], &vec![circle1], 0.01);

    assert!(combined_circles.len() == 0);
}

#[test]
fn subtract_from_self_rectangles_1() {
    // Two overlapping/identical rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = rectangle1.clone();

    // Create a hole in the larger circle
    let combined_rectangles = path_sub::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);
    println!("{:?}", combined_rectangles);

    assert!(combined_rectangles.len() != 1);
    assert!(combined_rectangles.len() == 0);
}

#[test]
fn subtract_from_self_rectangles_2() {
    // Two overlapping/identical rectangles (reverse direction to the other test)
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = rectangle1.clone();

    // Create a hole in the larger circle
    let combined_rectangles = path_sub::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);
    println!("{:?}", combined_rectangles);

    assert!(combined_rectangles.len() != 1);
    assert!(combined_rectangles.len() == 0);
}

#[test]
fn subtract_from_self_circles() {
    // Two overlapping/identical circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Create a hole in the larger circle
    let combined_circles = path_sub::<_, _, SimpleBezierPath>(&vec![circle2], &vec![circle1], 0.01);

    assert!(combined_circles.len() == 0);
}

#[test]
fn cut_corners() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 4.0))
        .line_to(Coord2(6.0, 4.0))
        .line_to(Coord2(6.0, 6.0))
        .line_to(Coord2(4.0, 6.0))
        .line_to(Coord2(4.0, 4.0))
        .build();

    // Subtract them
    let cut_corner = path_sub::<_, _, SimpleBezierPath>(&vec![rectangle1], &vec![rectangle2], 0.01);

    assert!(cut_corner.len() == 1);

    let cut_corner  = &cut_corner[0];
    let points      = cut_corner.points().collect::<Vec<_>>();

    assert!(cut_corner.start_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
    assert!(points[0].2.distance_to(&Coord2(5.0, 1.0)) < 0.1);
    assert!(points[1].2.distance_to(&Coord2(5.0, 4.0)) < 0.1);
    assert!(points[2].2.distance_to(&Coord2(4.0, 4.0)) < 0.1);
    assert!(points[3].2.distance_to(&Coord2(4.0, 5.0)) < 0.1);
    assert!(points[4].2.distance_to(&Coord2(1.0, 5.0)) < 0.1);
    assert!(points[5].2.distance_to(&Coord2(1.0, 1.0)) < 0.1);
}

#[test]
fn subtract_triangle_from_partial_circle_graph() {
    use flo_curves::debug::*;
    use std::collections::{HashMap};

    // This regenerates a failing test from arithmetic_intersection: problem seems to be that there are overlapping (or near-overlapping lines) that cause two outer edges when subtracting
    let remaining           = vec![(Coord2(477.3671569824219, 613.7830200195313), vec![(Coord2(483.87042236328125, 581.0888671875), Coord2(490.3741455078125, 548.3924560546875), Coord2(496.8785400390625, 515.6925659179688)), (Coord2(498.9593200683594, 515.6925659179688), Coord2(501.0400695800781, 515.6925659179688), Coord2(503.1199951171875, 515.6900024414063)), (Coord2(505.0438232421875, 514.8963012695313), Coord2(506.9661865234375, 514.1000366210938), Coord2(508.8900146484375, 513.2999877929688)), (Coord2(510.3604431152344, 511.8321838378906), Coord2(511.8317565917969, 510.3608703613281), Coord2(513.2999877929688, 508.8900146484375)), (Coord2(514.0997924804688, 506.9667663574219), Coord2(514.8960571289063, 505.0444030761719), Coord2(515.6900024414063, 503.1199951171875)), (Coord2(515.6925659179688, 501.0406799316406), Coord2(515.6925659179688, 498.9599304199219), Coord2(515.6900024414063, 496.8800048828125)), (Coord2(514.8963012695313, 494.9561767578125), Coord2(514.1000366210938, 493.0338134765625), Coord2(513.2999877929688, 491.1099853515625)), (Coord2(511.8321838378906, 489.6395568847656), Coord2(510.3608703613281, 488.1682434082031), Coord2(508.8900146484375, 486.70001220703125)), (Coord2(506.9667663574219, 485.90020751953125), Coord2(505.0444030761719, 485.10394287109375), Coord2(503.1199951171875, 484.30999755859375)), (Coord2(501.0406799316406, 484.30743408203125), Coord2(498.9599304199219, 484.30743408203125), Coord2(496.8800048828125, 484.30999755859375)), (Coord2(494.9561767578125, 485.10369873046875), Coord2(493.0338134765625, 485.89996337890625), Coord2(491.1099853515625, 486.70001220703125)), (Coord2(489.6395568847656, 488.1678161621094), Coord2(488.1682434082031, 489.6391296386719), Coord2(486.70001220703125, 491.1099853515625)), (Coord2(485.90020751953125, 493.0332336425781), Coord2(485.10394287109375, 494.9555969238281), Coord2(484.30999755859375, 496.8800048828125)), (Coord2(484.30743408203125, 498.9593200683594), Coord2(484.30743408203125, 501.0400695800781), Coord2(484.30999755859375, 503.1199951171875)), (Coord2(485.10369873046875, 505.0438232421875), Coord2(485.89996337890625, 506.9661865234375), Coord2(486.70001220703125, 508.8900146484375)), (Coord2(488.1678161621094, 510.3604431152344), Coord2(489.6391296386719, 511.8317565917969), Coord2(491.1108703613281, 513.3035278320313)), (Coord2(472.5879821777344, 541.0249633789063), Coord2(454.0650939941406, 568.7463989257813), Coord2(435.5415344238281, 596.4689331054688)), (Coord2(448.4329833984375, 605.102783203125), Coord2(462.67291259765625, 610.8741455078125), Coord2(477.3671569824219, 613.7830200195313))])];
    let fragment            = vec![(Coord2(491.1108762716864, 513.3035137968407), vec![(Coord2(438.50637541608546, 592.0317129194746), Coord2(385.91765275510227, 670.736298305119), Coord2(333.3289300941191, 749.4408836907635)), (Coord2(369.38413079268656, 764.375436814194), Coord2(405.428517093924, 779.3055104675816), Coord2(441.4729033951614, 794.2355841209692)), (Coord2(459.9451475894517, 701.369341374821), Coord2(478.41185121859684, 608.5309529306364), Coord2(496.87855484774195, 515.6925644864517)), (Coord2(494.95561081048504, 514.8960549865354), Coord2(493.0332435410857, 514.099784391688), Coord2(491.1108762716864, 513.3035137968407))])];

    // Contains points that are very close and not the same - for exmaple:
    // 491.11087 03613281, 513.3035 278320313
    // 491.11087 62716864, 513.3035 137968407

    // Merge the two paths
    let mut merged_path     = GraphPath::new();
    merged_path             = merged_path.merge(GraphPath::from_merged_paths(remaining.iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));
    merged_path             = merged_path.collide(GraphPath::from_merged_paths(fragment.iter().map(|path| (path, PathLabel(1, PathDirection::from(path))))), 0.01);

    // Subtract fragment from remaining
    merged_path.set_exterior_by_subtracting();
    println!("{}", graph_path_svg_string(&merged_path, vec![]));
    merged_path.heal_exterior_gaps();

    // No points with any edges leaving or arriving at them should be close to each other
    let mut point_positions = HashMap::new();
    for edge in merged_path.all_edges() {
        let start_idx   = edge.start_point_index();
        let end_idx     = edge.end_point_index();

        let start_pos   = edge.start_point();
        let end_pos     = edge.end_point();

        point_positions.insert(start_idx, start_pos);
        point_positions.insert(end_idx, end_pos);
    }

    println!();
    for (idx, pos) in point_positions.iter() {
        for (cmp_idx, cmp_pos) in point_positions.iter() {
            if cmp_idx == idx { continue; }

            if pos.distance_to(cmp_pos) < 1.0 {
                println!("Overlapping points: {} {}", idx, cmp_idx);
            }
        }
    }

    // Extract the resulting path
    let subtracted_path     = merged_path.exterior_paths::<SimpleBezierPath>();

    // This should entirely subtract the triangle from the remaining path
    assert!(subtracted_path.len() == 1);
}

#[test]
fn subtract_triangle_from_partial_circle() {
    // This regenerates a failing test from arithmetic_intersection: problem seems to be that there are overlapping (or near-overlapping lines) that cause two outer edges when subtracting
    let remaining           = vec![(Coord2(477.3671569824219, 613.7830200195313), vec![(Coord2(483.87042236328125, 581.0888671875), Coord2(490.3741455078125, 548.3924560546875), Coord2(496.8785400390625, 515.6925659179688)), (Coord2(498.9593200683594, 515.6925659179688), Coord2(501.0400695800781, 515.6925659179688), Coord2(503.1199951171875, 515.6900024414063)), (Coord2(505.0438232421875, 514.8963012695313), Coord2(506.9661865234375, 514.1000366210938), Coord2(508.8900146484375, 513.2999877929688)), (Coord2(510.3604431152344, 511.8321838378906), Coord2(511.8317565917969, 510.3608703613281), Coord2(513.2999877929688, 508.8900146484375)), (Coord2(514.0997924804688, 506.9667663574219), Coord2(514.8960571289063, 505.0444030761719), Coord2(515.6900024414063, 503.1199951171875)), (Coord2(515.6925659179688, 501.0406799316406), Coord2(515.6925659179688, 498.9599304199219), Coord2(515.6900024414063, 496.8800048828125)), (Coord2(514.8963012695313, 494.9561767578125), Coord2(514.1000366210938, 493.0338134765625), Coord2(513.2999877929688, 491.1099853515625)), (Coord2(511.8321838378906, 489.6395568847656), Coord2(510.3608703613281, 488.1682434082031), Coord2(508.8900146484375, 486.70001220703125)), (Coord2(506.9667663574219, 485.90020751953125), Coord2(505.0444030761719, 485.10394287109375), Coord2(503.1199951171875, 484.30999755859375)), (Coord2(501.0406799316406, 484.30743408203125), Coord2(498.9599304199219, 484.30743408203125), Coord2(496.8800048828125, 484.30999755859375)), (Coord2(494.9561767578125, 485.10369873046875), Coord2(493.0338134765625, 485.89996337890625), Coord2(491.1099853515625, 486.70001220703125)), (Coord2(489.6395568847656, 488.1678161621094), Coord2(488.1682434082031, 489.6391296386719), Coord2(486.70001220703125, 491.1099853515625)), (Coord2(485.90020751953125, 493.0332336425781), Coord2(485.10394287109375, 494.9555969238281), Coord2(484.30999755859375, 496.8800048828125)), (Coord2(484.30743408203125, 498.9593200683594), Coord2(484.30743408203125, 501.0400695800781), Coord2(484.30999755859375, 503.1199951171875)), (Coord2(485.10369873046875, 505.0438232421875), Coord2(485.89996337890625, 506.9661865234375), Coord2(486.70001220703125, 508.8900146484375)), (Coord2(488.1678161621094, 510.3604431152344), Coord2(489.6391296386719, 511.8317565917969), Coord2(491.1108703613281, 513.3035278320313)), (Coord2(472.5879821777344, 541.0249633789063), Coord2(454.0650939941406, 568.7463989257813), Coord2(435.5415344238281, 596.4689331054688)), (Coord2(448.4329833984375, 605.102783203125), Coord2(462.67291259765625, 610.8741455078125), Coord2(477.3671569824219, 613.7830200195313))])];
    let fragment            = vec![(Coord2(491.1108762716864, 513.3035137968407), vec![(Coord2(438.50637541608546, 592.0317129194746), Coord2(385.91765275510227, 670.736298305119), Coord2(333.3289300941191, 749.4408836907635)), (Coord2(369.38413079268656, 764.375436814194), Coord2(405.428517093924, 779.3055104675816), Coord2(441.4729033951614, 794.2355841209692)), (Coord2(459.9451475894517, 701.369341374821), Coord2(478.41185121859684, 608.5309529306364), Coord2(496.87855484774195, 515.6925644864517)), (Coord2(494.95561081048504, 514.8960549865354), Coord2(493.0332435410857, 514.099784391688), Coord2(491.1108762716864, 513.3035137968407))])];

    // Contains points that are very close and not the same - for exmaple:
    // 491.11087 03613281, 513.3035 278320313
    // 491.11087 62716864, 513.3035 137968407

    // Merge the two paths
    let subtracted_path     = path_sub::<_, _, SimpleBezierPath>(&remaining, &fragment, 0.01);

    // This should entirely subtract the triangle from the remaining path
    assert!(subtracted_path.len() == 1);
}
