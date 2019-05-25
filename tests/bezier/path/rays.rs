use flo_curves::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;

#[test]
fn crossing_figure_of_8_intersection_from_inside() {
    //
    // +       +
    // | \   / |
    // |   +   | <--- RAY
    // | /   \ |
    // +       +
    // 
    // This ray hits a corner but it should generate either 0 or 2 collisions at this point, and particularly not 1.
    // (0 intersections implies the ray never leaves the shape and 2 intersections indicates it leaves and immediately 
    // re-enters)
    // 
    // (Interestingly, either behaviour is correct: if there are 0 collisions we won't categorise the edges and if there
    // are 2 we'll mark the edges as exterior when the ray is used to set edge kinds)
    // 
    // This is interesting because of this case:
    // 
    // +
    // | \
    // |   +  <--- RAY
    // | /
    // +
    // 
    // As the 'same' point here should always generate 1 intersection as the ray enters the shape at this point (or leaves
    // in the reverse direction)
    //

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let right_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(6.0, 1.0))
        .line_to(Coord2(6.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(6.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());
    let graph_path = graph_path.collide(GraphPath::from_path(&right_triangle, ()), 0.01);

    let collisions = graph_path.ray_collisions(&(Coord2(8.0, 2.0), Coord2(7.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len()&1) == 0);

    assert!(collisions.len() == 4);

    // The intersection point should be an actual intersection
    assert!((0..(graph_path.num_points())).into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).count())
        .filter(|num_edges_for_point| num_edges_for_point == &2)
        .count() == 1);
    assert!((0..(graph_path.num_points())).into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).count())
        .filter(|num_edges_for_point| num_edges_for_point == &1)
        .count() == 4);

    // Also test the ray travelling the other way
    let collisions = graph_path.ray_collisions(&(Coord2(-2.0, 2.0), Coord2(-1.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len()&1) == 0);

    assert!(collisions.len() == 4);
}

#[test]
fn crossing_figure_of_8_intersection_from_inside_reversed() {
    //
    // +       +
    // | \   / |
    // |   +   | <--- RAY
    // | /   \ |
    // +       +
    // 

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let right_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(6.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(6.0, 1.0))
        .line_to(Coord2(6.0, 3.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());
    let graph_path = graph_path.collide(GraphPath::from_path(&right_triangle, ()), 0.01);

    let collisions = graph_path.ray_collisions(&(Coord2(8.0, 2.0), Coord2(7.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len()&1) == 0);

    assert!(collisions.len() == 4);

    // The intersection point should be an actual intersection
    assert!((0..(graph_path.num_points())).into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).count())
        .filter(|num_edges_for_point| num_edges_for_point == &2)
        .count() == 1);
    assert!((0..(graph_path.num_points())).into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).count())
        .filter(|num_edges_for_point| num_edges_for_point == &1)
        .count() == 4);

    // Also test the ray travelling the other way
    let collisions = graph_path.ray_collisions(&(Coord2(-2.0, 2.0), Coord2(-1.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len()&1) == 0);

    assert!(collisions.len() == 4);
}

#[test]
fn crossing_figure_of_8_intersection_from_inside_nearby() {
    //
    // +       +
    // | \   / |
    // |   +   | <--- RAY
    // | /   \ |
    // +       +
    // 

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let right_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(6.0, 1.0))
        .line_to(Coord2(6.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(6.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());
    let graph_path = graph_path.collide(GraphPath::from_path(&right_triangle, ()), 0.01);

    for y in [1.9, 1.99, 1.999, 1.9999, 1.99999, 1.99999, 2.1, 2.01, 2.001, 2.0001, 2.00001, 2.000001, 2.0000001].into_iter() {
        let collisions = graph_path.ray_collisions(&(Coord2(8.0, *y), Coord2(7.0, *y)));

        assert!(collisions.len() != 3);
        assert!((collisions.len()&1) == 0);

        assert!(collisions.len() == 4 || collisions.len() == 2);

        // Also test the ray travelling the other way
        let collisions = graph_path.ray_collisions(&(Coord2(-2.0, *y), Coord2(-1.0, *y)));

        assert!(collisions.len() != 3);
        assert!((collisions.len()&1) == 0);

        assert!(collisions.len() == 4);
    }
}

#[test]
fn crossing_figure_of_8_intersection_collinear() {
    //
    //                 Ray
    //               /
    //             /
    //           L
    // +       +
    // | \   / |
    // |   +   |
    // | /   \ |
    // +       +
    // 

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let right_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(5.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());
    let graph_path = graph_path.collide(GraphPath::from_path(&right_triangle, ()), 0.01);

    let collisions = graph_path.ray_collisions(&(Coord2(1.0, 1.0), Coord2(3.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len()&1) == 0);

    assert!(collisions.len() == 0);
}

#[test]
fn crossing_figure_of_8_intersection_from_outside() {
    // As above, but the ray passing vertically through the intersection
    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let right_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(6.0, 1.0))
        .line_to(Coord2(6.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(6.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());
    let graph_path = graph_path.collide(GraphPath::from_path(&right_triangle, ()), 0.01);

    let collisions = graph_path.ray_collisions(&(Coord2(3.0, 0.0), Coord2(3.0, 1.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len()&1) == 0);

    assert!(collisions.len() == 0 || collisions.len() == 2);
}

#[test]
fn crossing_intersection_with_collinear_edge() {
    //
    // +       +
    // | \   /   \
    // |   + ----- + <--- RAY
    // | /  
    // +    
    //

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let right_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 2.0))
        .line_to(Coord2(7.0, 2.0))
        .line_to(Coord2(5.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());
    let graph_path = graph_path.collide(GraphPath::from_path(&right_triangle, ()), 0.01);

    let collisions = graph_path.ray_collisions(&(Coord2(8.0, 2.0), Coord2(7.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len()&1) == 0);

    assert!(collisions.len() == 2);
}

#[test]
fn ray_entering_triangle_through_apex_1() {
    // 
    // +
    // | \
    // |   +  <--- RAY
    // | /
    // +
    // 
    // As the 'same' point here should always generate 1 intersection as the ray enters the shape at this point (or leaves
    // in the reverse direction)
    //

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());

    let collisions = graph_path.ray_collisions(&(Coord2(8.0, 2.0), Coord2(7.0, 2.0)));

    assert!((collisions.len()&1) == 0);
    assert!(collisions.len() == 2);
}

#[test]
fn ray_entering_triangle_through_apex_2() {
    // 
    // +
    // | \
    // |   +  ---> RAY
    // | /
    // +
    // 
    // As the 'same' point here should always generate 1 intersection as the ray enters the shape at this point (or leaves
    // in the reverse direction)
    //

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());

    let collisions = graph_path.ray_collisions(&(Coord2(0.0, 2.0), Coord2(1.0, 2.0)));

    assert!((collisions.len()&1) == 0);
    assert!(collisions.len() == 2);
}

#[test]
fn ray_entering_triangle_through_apex_3() {
    // 
    // +
    // | \
    // |   +  ---> RAY
    // | /
    // +
    // 
    // As the 'same' point here should always generate 1 intersection as the ray enters the shape at this point (or leaves
    // in the reverse direction)
    //

    let left_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .line_to(Coord2(3.0, 2.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&left_triangle, ());

    for y in [1.9, 1.99, 1.999, 1.9999, 1.99999, 1.99999, 2.1, 2.01, 2.001, 2.0001, 2.00001, 2.000001, 2.0000001].into_iter() {
        let collisions = graph_path.ray_collisions(&(Coord2(0.0, *y), Coord2(1.0, *y)));

        assert!((collisions.len()&1) == 0);
        assert!(collisions.len() == 2);
    }
}

#[test]
fn ray_hitting_tangent_at_point() {
    let tangent_triangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 3.0))
        .curve_to((Coord2(3.0, 3.0), Coord2(3.0, 3.0)), Coord2(3.0, 2.0))
        .curve_to((Coord2(3.0, 1.0), Coord2(3.0, 1.0)), Coord2(1.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&tangent_triangle, ());

    let collisions = graph_path.ray_collisions(&(Coord2(3.0, 0.0), Coord2(3.0, 1.0)));

    assert!((collisions.len()&1) == 0);
    assert!(collisions.len() == 0);
}

#[test]
fn ray_hitting_intersection_bad() {
    // These three edges form an intersection that has a known bad intersection with the specified ray
    // edge2 here generates 2 collisions at the intersection for some reason, which seems to be what's causing a bug
    let ray     = (Coord2(614.1064453125, 904.1033935546875), Coord2(614.3379516601563, 903.910888671875));
    let edge1   = Curve::from_points(Coord2(612.35302734375, 902.1972045898438), (Coord2(611.9544677734375, 904.4937744140625), Coord2(612.1427001953125, 905.798828125)), Coord2(613.4901123046875, 904.6159057617188));
    let edge2   = Curve::from_points(Coord2(613.4901123046875, 904.6159057617188), (Coord2(613.6087646484375, 904.5118408203125), Coord2(613.736328125, 904.388427734375)), Coord2(613.873291015625, 904.2447509765625));
    let edge3   = Curve::from_points(Coord2(613.1998901367188, 904.267822265625), (Coord2(613.2864379882813, 904.4163818359375), Coord2(613.3829956054688, 904.5339965820313)), Coord2(613.4901123046875, 904.6159057617188));

    let ray1 = curve_intersects_ray(&edge1, &ray);
    let ray2 = curve_intersects_ray(&edge2, &ray);
    let ray3 = curve_intersects_ray(&edge3, &ray);

    // Ray1 produces a collision at the end that I think doesn't get merged by the appropriate step
    assert!(ray1.len() == 1);
    assert!(ray2.len() == 1);
    assert!(ray3.len() == 1);
}

#[test]
fn ray_misses_end_by_small_margin() {
    let ray         = (Coord2(848.5234985351563, 797.486328125), Coord2(848.4642944335938, 797.4795532226563));
    let edges       = vec![
            Curve::from_points(Coord2(848.4671020507813, 797.462646484375), (Coord2(848.46630859375, 797.459228515625), Coord2(848.4619140625, 797.49755859375)), Coord2(848.4624633789063, 797.5032348632813)),
            Curve::from_points(Coord2(848.4624633789063, 797.5032348632813), (Coord2(848.3275146484375, 797.4852294921875), Coord2(848.57958984375, 797.7412719726563)), Coord2(848.715576171875, 797.7614135742188)),
            Curve::from_points(Coord2(848.715576171875, 797.7614135742188), (Coord2(848.7601928710938, 797.739501953125), Coord2(848.8429565429688, 797.6612548828125)), Coord2(848.9512939453125, 797.535400390625)),
            Curve::from_points(Coord2(850.2943725585938, 795.3328247070313), (Coord2(851.0455932617188, 793.9050903320313), Coord2(852.1541748046875, 792.0202026367188)), Coord2(853.2625122070313, 789.974609375)),
            Curve::from_points(Coord2(853.2625122070313, 789.974609375), (Coord2(855.5187377929688, 785.8296508789063), Coord2(858.1968383789063, 781.510986328125)), Coord2(859.6488037109375, 779.812255859375)),
            Curve::from_points(Coord2(859.6488037109375, 779.812255859375), (Coord2(859.3439331054688, 779.5526123046875), Coord2(858.9374389648438, 779.2064208984375)), Coord2(858.632568359375, 778.94677734375)),
            Curve::from_points(Coord2(858.632568359375, 778.94677734375), (Coord2(857.1051025390625, 780.74658203125), Coord2(854.6322021484375, 785.4913330078125)), Coord2(852.4514770507813, 789.53515625)),
            Curve::from_points(Coord2(852.4514770507813, 789.53515625), (Coord2(851.3413696289063, 791.583984375), Coord2(850.4099731445313, 793.5748291015625)), Coord2(849.6604614257813, 794.999267578125)),
            Curve::from_points(Coord2(849.6604614257813, 794.999267578125), (Coord2(848.7610473632813, 796.5136108398438), Coord2(848.5972290039063, 797.339599609375)), Coord2(848.715576171875, 797.2513427734375)),
            Curve::from_points(Coord2(848.715576171875, 797.2513427734375), (Coord2(848.8153686523438, 797.266357421875), Coord2(848.9803466796875, 797.4114379882813)), Coord2(849.001953125, 797.4755249023438)),
            Curve::from_points(Coord2(848.9655151367188, 797.5032348632813), (Coord2(848.9658203125, 797.50537109375), Coord2(848.9645385742188, 797.5181884765625)), Coord2(848.96337890625, 797.5213623046875)),
            Curve::from_points(Coord2(848.9614868164063, 797.5369873046875), (Coord2(848.9580078125, 797.5364379882813), Coord2(848.95458984375, 797.535888671875)), Coord2(848.9512939453125, 797.535400390625)),
            Curve::from_points(Coord2(848.9512939453125, 797.535400390625), (Coord2(848.9552612304688, 797.53076171875), Coord2(848.959228515625, 797.5260620117188)), Coord2(848.96337890625, 797.5213623046875)),
            Curve::from_points(Coord2(848.9512939453125, 797.535400390625), (Coord2(848.6336059570313, 797.4876708984375), Coord2(848.7899780273438, 797.51123046875)), Coord2(848.4671020507813, 797.462646484375)),
            Curve::from_points(Coord2(848.96337890625, 797.5213623046875), (Coord2(848.9678955078125, 797.5159912109375), Coord2(848.9724731445313, 797.5105590820313)), Coord2(848.9771728515625, 797.5051879882813)),
            Curve::from_points(Coord2(848.96337890625, 797.5213623046875), (Coord2(848.9624633789063, 797.5344848632813), Coord2(848.9616088867188, 797.53955078125)), Coord2(848.9614868164063, 797.5369873046875)),
            Curve::from_points(Coord2(848.9771728515625, 797.5051879882813), (Coord2(848.9853515625, 797.49560546875), Coord2(848.9935913085938, 797.4857177734375)), Coord2(849.001953125, 797.4755249023438)),
            Curve::from_points(Coord2(848.9771728515625, 797.5051879882813), (Coord2(848.9735717773438, 797.5043334960938), Coord2(848.9697265625, 797.50390625)), Coord2(848.9655151367188, 797.5032348632813)),
            Curve::from_points(Coord2(849.001953125, 797.4755249023438), (Coord2(849.3349609375, 797.0731811523438), Coord2(849.86474609375, 796.28857421875)), Coord2(850.2943725585938, 795.3328247070313)),
            Curve::from_points(Coord2(849.001953125, 797.4755249023438), (Coord2(849.00830078125, 797.494384765625), Coord2(849.001953125, 797.5062255859375)), Coord2(848.9771728515625, 797.5051879882813))
        ];
    let collisions = edges.into_iter()
        .map(|edge| {
            (edge, curve_intersects_ray(&edge, &ray))
        })
        .collect::<Vec<_>>();

    println!("{:?} (total {} collisions)", collisions, collisions.iter().map(|(_, c)| c.len()).sum::<usize>());
    assert!(false);
}
