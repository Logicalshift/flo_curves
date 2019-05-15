use flo_curves::*;
use flo_curves::bezier::path::*;

#[test]
fn crossing_figure_of_8_intersection_from_inside() {
    //
    // +       +
    // | \   / |
    // |   +   | <--- RAY
    // | /  \  |
    // +      +
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

    assert!(collisions.len() == 4 || collisions.len() == 2);

    // The intersection point should be an actual intersection
    assert!((0..(graph_path.num_points())).into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).count())
        .filter(|num_edges_for_point| num_edges_for_point == &2)
        .count() == 1);
    assert!((0..(graph_path.num_points())).into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).count())
        .filter(|num_edges_for_point| num_edges_for_point == &1)
        .count() == 4);
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
fn ray_entering_triangle_through_apex() {
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
