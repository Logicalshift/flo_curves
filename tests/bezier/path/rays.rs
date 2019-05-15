use flo_curves::*;
use flo_curves::bezier::path::*;

#[test]
fn crossing_figure_of_8_intersection() {
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
}