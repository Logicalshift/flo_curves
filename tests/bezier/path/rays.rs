use flo_curves::bezier::path::{
    BezierPath, BezierPathBuilder, BezierPathFactory, GraphPath, SimpleBezierPath,
};
use flo_curves::bezier::{
    curve_intersects_ray, BezierCurveFactory, BoundingBox, Coord2, Coordinate, Curve,
};

use std::collections::HashMap;

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
    assert!((collisions.len() & 1) == 0);

    assert!(collisions.len() == 4);

    // The intersection point should be an actual intersection
    assert!(
        (0..(graph_path.num_points()))
            .into_iter()
            .map(|point_idx| graph_path.edges_for_point(point_idx).count())
            .filter(|num_edges_for_point| num_edges_for_point == &2)
            .count()
            == 1
    );
    assert!(
        (0..(graph_path.num_points()))
            .into_iter()
            .map(|point_idx| graph_path.edges_for_point(point_idx).count())
            .filter(|num_edges_for_point| num_edges_for_point == &1)
            .count()
            == 4
    );

    // Also test the ray travelling the other way
    let collisions = graph_path.ray_collisions(&(Coord2(-2.0, 2.0), Coord2(-1.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len() & 1) == 0);

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
    assert!((collisions.len() & 1) == 0);

    assert!(collisions.len() == 4);

    // The intersection point should be an actual intersection
    assert!(
        (0..(graph_path.num_points()))
            .into_iter()
            .map(|point_idx| graph_path.edges_for_point(point_idx).count())
            .filter(|num_edges_for_point| num_edges_for_point == &2)
            .count()
            == 1
    );
    assert!(
        (0..(graph_path.num_points()))
            .into_iter()
            .map(|point_idx| graph_path.edges_for_point(point_idx).count())
            .filter(|num_edges_for_point| num_edges_for_point == &1)
            .count()
            == 4
    );

    // Also test the ray travelling the other way
    let collisions = graph_path.ray_collisions(&(Coord2(-2.0, 2.0), Coord2(-1.0, 2.0)));

    assert!(collisions.len() != 3);
    assert!((collisions.len() & 1) == 0);

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

    for y in [
        1.9, 1.99, 1.999, 1.9999, 1.99999, 1.99999, 2.1, 2.01, 2.001, 2.0001, 2.00001, 2.000001,
        2.0000001,
    ]
    .iter()
    {
        let collisions = graph_path.ray_collisions(&(Coord2(8.0, *y), Coord2(7.0, *y)));

        assert!(collisions.len() != 3);
        assert!((collisions.len() & 1) == 0);

        assert!(collisions.len() == 4 || collisions.len() == 2);

        // Also test the ray travelling the other way
        let collisions = graph_path.ray_collisions(&(Coord2(-2.0, *y), Coord2(-1.0, *y)));

        assert!(collisions.len() != 3);
        assert!((collisions.len() & 1) == 0);

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
    assert!((collisions.len() & 1) == 0);

    assert!(collisions.is_empty());
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
    assert!((collisions.len() & 1) == 0);

    assert!(collisions.is_empty() || collisions.len() == 2);
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
    assert!((collisions.len() & 1) == 0);

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

    assert!((collisions.len() & 1) == 0);
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

    assert!((collisions.len() & 1) == 0);
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

    for y in [
        1.9, 1.99, 1.999, 1.9999, 1.99999, 1.99999, 2.1, 2.01, 2.001, 2.0001, 2.00001, 2.000001,
        2.0000001,
    ]
    .iter()
    {
        let collisions = graph_path.ray_collisions(&(Coord2(0.0, *y), Coord2(1.0, *y)));

        assert!((collisions.len() & 1) == 0);
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

    assert!((collisions.len() & 1) == 0);
    assert!(collisions.is_empty());
}

#[test]
fn ray_hitting_intersection_bad() {
    // These three edges form an intersection that has a known bad intersection with the specified ray
    // edge2 here generates 2 collisions at the intersection for some reason, which seems to be what's causing a bug
    let ray = (
        Coord2(614.1064453125, 904.1033935546875),
        Coord2(614.3379516601563, 903.910888671875),
    );
    let edge1 = Curve::from_points(
        Coord2(612.35302734375, 902.1972045898438),
        (
            Coord2(611.9544677734375, 904.4937744140625),
            Coord2(612.1427001953125, 905.798828125),
        ),
        Coord2(613.4901123046875, 904.6159057617188),
    );
    let edge2 = Curve::from_points(
        Coord2(613.4901123046875, 904.6159057617188),
        (
            Coord2(613.6087646484375, 904.5118408203125),
            Coord2(613.736328125, 904.388427734375),
        ),
        Coord2(613.873291015625, 904.2447509765625),
    );
    let edge3 = Curve::from_points(
        Coord2(613.1998901367188, 904.267822265625),
        (
            Coord2(613.2864379882813, 904.4163818359375),
            Coord2(613.3829956054688, 904.5339965820313),
        ),
        Coord2(613.4901123046875, 904.6159057617188),
    );

    let ray1 = curve_intersects_ray(&edge1, &ray);
    let ray2 = curve_intersects_ray(&edge2, &ray);
    let ray3 = curve_intersects_ray(&edge3, &ray);

    println!("{:?}", ray1);
    println!("{:?}", ray2);
    println!("{:?}", ray3);

    // edge2 should generate 1 collision
    assert!(ray2.len() == 1);
    assert!(ray3.len() == 1);

    // Ray1 produces a collision at the end
    assert!(ray1.len() == 2);
}

#[test]
fn ray_hitting_start_and_end_of_line_1() {
    //
    //     + --- +
    //     |     |
    //     |     |     Should produce two collisions, even though the ray effectively hits both the start and end
    // +---+     |     points.
    // |         |
    // |         |
    // +---------+
    //     ^
    //     |
    //    Ray
    //
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(2.0, 3.0))
        .line_to(Coord2(2.0, 2.0))
        .line_to(Coord2(1.0, 2.0))
        .build();
    let ray = (Coord2(2.0, 0.0), Coord2(2.0, 1.0));
    let graph_path = GraphPath::from_path(&path, ());

    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() == 2);
}

#[test]
fn ray_hitting_start_and_end_of_line_2() {
    //
    //     + --- +
    //     |     |
    //     |     |     Should produce two collisions, even though the ray effectively hits both the start and end
    // +---+     |     points. Here we move the point very slightly so the ray crosses the line instead of exactly
    // |         |     hitting the center point
    // |         |
    // +---------+
    //     ^
    //     |
    //    Ray
    //
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(2.0, 3.0))
        .line_to(Coord2(2.001, 2.0))
        .line_to(Coord2(1.0, 2.0))
        .build();
    let ray = (Coord2(2.0, 0.0), Coord2(2.0, 1.0));
    let graph_path = GraphPath::from_path(&path, ());

    let collisions = graph_path.ray_collisions(&ray);

    println!("{:?}", collisions);
    assert!(collisions.len() & 1 == 0);
    assert!(collisions.len() == 2);
}

#[test]
fn ray_hitting_start_and_end_of_line_3() {
    // As above but crossing even closer
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(2.0, 3.0))
        .line_to(Coord2(2.000999, 2.0))
        .line_to(Coord2(1.0, 2.0))
        .build();
    let ray = (Coord2(2.0, 0.0), Coord2(2.0, 1.0));
    let graph_path = GraphPath::from_path(&path, ());

    let collisions = graph_path.ray_collisions(&ray);

    println!("{:?}", collisions);
    assert!(collisions.len() & 1 == 0);
    assert!(collisions.len() == 2);
}

#[test]
fn ray_hitting_start_and_end_of_line_4() {
    // Moving one of the other points
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(1.999, 3.0))
        .line_to(Coord2(2.0, 2.0))
        .line_to(Coord2(1.0, 2.0))
        .build();
    let ray = (Coord2(2.0, 0.0), Coord2(2.0, 1.0));
    let graph_path = GraphPath::from_path(&path, ());

    let collisions = graph_path.ray_collisions(&ray);

    println!("{:?}", collisions);
    assert!(collisions.len() & 1 == 0);
    assert!(collisions.len() == 2);
}

#[test]
fn ray_hitting_start_and_end_of_curve_1() {
    // As above, but curve bowing outwards
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(2.0, 3.0))
        .curve_to((Coord2(1.0, 3.0), Coord2(1.0, 2.0)), Coord2(2.0, 2.0))
        .line_to(Coord2(1.0, 2.0))
        .build();
    let ray = (Coord2(2.0, 0.0), Coord2(2.0, 1.0));
    let graph_path = GraphPath::from_path(&path, ());

    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() == 2);
}

#[test]
fn ray_hitting_start_and_end_of_curve_2() {
    // As above, but curve bowing inwards
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(3.0, 1.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(2.0, 3.0))
        .curve_to((Coord2(3.0, 3.0), Coord2(3.0, 2.0)), Coord2(2.0, 2.0))
        .line_to(Coord2(1.0, 2.0))
        .build();
    let ray = (Coord2(2.0, 0.0), Coord2(2.0, 1.0));
    let graph_path = GraphPath::from_path(&path, ());

    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() == 2);
}

#[test]
fn ray_crossing_and_glancing() {
    // This is a ray that scores a crossing collision followed by a glancing collision along a very short curve that is almost but not quite
    // collinear. We should remove the glancing collision but we do not for some reason
    let ray = (
        Coord2(694.1428833007813, 884.8551025390625),
        Coord2(692.2153930664063, 883.697509765625),
    );
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(632.3152465820313, 838.1866455078125))
        .curve_to(
            (
                Coord2(634.3826904296875, 844.4097290039063),
                Coord2(635.91357421875, 849.9752807617188),
            ),
            Coord2(635.8788452148438, 849.8710327148438),
        )
        .curve_to(
            (
                Coord2(635.9103393554688, 849.8935546875),
                Coord2(635.9293212890625, 849.9025268554688),
            ),
            Coord2(635.9400634765625, 849.9002075195313),
        )
        .curve_to(
            (
                Coord2(635.97216796875, 850.158447265625),
                Coord2(635.9955444335938, 850.357666015625),
            ),
            Coord2(636.0320434570313, 850.4244384765625),
        )
        .line_to(Coord2(635.97216796875, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&path, ());
    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() & 1 == 0);
}

#[test]
fn ray_glancing_1() {
    let ray = (
        Coord2(798.2357788085938, 783.7974853515625),
        Coord2(798.6553344726563, 782.6351928710938),
    );
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 677.4369506835938))
        .line_to(Coord2(839.3995361328125, 677.4369506835938))
        .curve_to(
            (
                Coord2(839.6207275390625, 674.3713989257813),
                Coord2(838.3349609375, 674.1100463867188),
            ),
            Coord2(837.8158569335938, 674.1475830078125),
        )
        .curve_to(
            (
                Coord2(838.1270141601563, 674.0364990234375),
                Coord2(838.5419311523438, 673.8883056640625),
            ),
            Coord2(838.8530883789063, 673.7772216796875),
        )
        .line_to(Coord2(838.8530883789063, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&path, ());
    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() & 1 == 0);
}

#[test]
fn ray_glancing_1_reversed() {
    let ray = (
        Coord2(798.2357788085938, 783.7974853515625),
        Coord2(798.6553344726563, 782.6351928710938),
    );
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 677.4369506835938))
        .line_to(Coord2(839.3995361328125, 677.4369506835938))
        .curve_to(
            (
                Coord2(839.6207275390625, 674.3713989257813),
                Coord2(838.3349609375, 674.1100463867188),
            ),
            Coord2(837.8158569335938, 674.1475830078125),
        )
        .curve_to(
            (
                Coord2(838.1270141601563, 674.0364990234375),
                Coord2(838.5419311523438, 673.8883056640625),
            ),
            Coord2(838.8530883789063, 673.7772216796875),
        )
        .line_to(Coord2(838.8530883789063, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = path.reversed::<SimpleBezierPath>();

    let graph_path = GraphPath::from_path(&path, ());
    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() & 1 == 0);
}

#[test]
fn ray_glancing_2() {
    let ray = (
        Coord2(576.3092041015625, 854.6082153320313),
        Coord2(576.0201416015625, 854.5975341796875),
    );
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 854.7772216796875))
        .line_to(Coord2(580.010986328125, 854.7772216796875))
        .curve_to(
            (
                Coord2(580.0106201171875, 854.7659301757813),
                Coord2(580.01025390625, 854.7548828125),
            ),
            Coord2(580.0098266601563, 854.7442016601563),
        )
        .curve_to(
            (
                Coord2(580.0198974609375, 854.7529296875),
                Coord2(580.0298461914063, 854.761474609375),
            ),
            Coord2(580.0394897460938, 854.7695922851563),
        )
        .line_to(Coord2(580.0394897460938, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let graph_path = GraphPath::from_path(&path, ());
    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() & 1 == 0);

    let mut edge_collisions = HashMap::new();
    for (collision, _curve_t, _line_t, _position) in collisions {
        let edge = collision.edge();

        *(edge_collisions.entry(edge).or_insert(0)) += 1;
    }

    assert!(edge_collisions.into_iter().all(|(_, count)| count == 1));
}

#[test]
fn ray_glancing_2_reversed() {
    let ray = (
        Coord2(576.3092041015625, 854.6082153320313),
        Coord2(576.0201416015625, 854.5975341796875),
    );
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 854.7772216796875))
        .line_to(Coord2(580.010986328125, 854.7772216796875))
        .curve_to(
            (
                Coord2(580.0106201171875, 854.7659301757813),
                Coord2(580.01025390625, 854.7548828125),
            ),
            Coord2(580.0098266601563, 854.7442016601563),
        )
        .curve_to(
            (
                Coord2(580.0198974609375, 854.7529296875),
                Coord2(580.0298461914063, 854.761474609375),
            ),
            Coord2(580.0394897460938, 854.7695922851563),
        )
        .line_to(Coord2(580.0394897460938, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = path.reversed::<SimpleBezierPath>();

    let graph_path = GraphPath::from_path(&path, ());
    let collisions = graph_path.ray_collisions(&ray);

    assert!(collisions.len() & 1 == 0);

    let mut edge_collisions = HashMap::new();
    for (collision, _curve_t, _line_t, _position) in collisions {
        let edge = collision.edge();

        *(edge_collisions.entry(edge).or_insert(0)) += 1;
    }

    assert!(edge_collisions.into_iter().all(|(_, count)| count == 1));
}
