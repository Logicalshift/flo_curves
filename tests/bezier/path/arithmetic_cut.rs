use flo_curves::bezier::path::*;
use flo_curves::*;

#[test]
fn cut_square() {
    let square_1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(5.0, 5.0))
        .line_to(Coord2(10.0, 5.0))
        .line_to(Coord2(10.0, 10.0))
        .line_to(Coord2(5.0, 10.0))
        .line_to(Coord2(5.0, 5.0))
        .build();

    let square_2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(7.5, 7.5))
        .line_to(Coord2(15.0, 7.5))
        .line_to(Coord2(15.0, 15.0))
        .line_to(Coord2(7.5, 15.0))
        .line_to(Coord2(7.5, 7.5))
        .build();

    let cut_square = path_cut::<_, _, SimpleBezierPath>(&vec![square_1], &vec![square_2], 0.01);

    assert!(cut_square.exterior_path.len() == 1);
    assert!(cut_square.interior_path.len() == 1);

    assert!(cut_square.interior_path[0].points().len() == 4);
    assert!(cut_square.exterior_path[0].points().len() == 6);
}

#[test]
fn cut_square_entirely_interior() {
    let square_1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(5.0, 5.0))
        .line_to(Coord2(10.0, 5.0))
        .line_to(Coord2(10.0, 10.0))
        .line_to(Coord2(5.0, 10.0))
        .line_to(Coord2(5.0, 5.0))
        .build();

    let square_2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(2.0, 2.0))
        .line_to(Coord2(15.0, 2.0))
        .line_to(Coord2(15.0, 15.0))
        .line_to(Coord2(2.0, 15.0))
        .line_to(Coord2(2.0, 2.0))
        .build();

    let cut_square = path_cut::<_, _, SimpleBezierPath>(&vec![square_1], &vec![square_2], 0.01);

    assert!(cut_square.exterior_path.len() == 0);
    assert!(cut_square.interior_path.len() == 1);

    assert!(cut_square.interior_path[0].points().len() == 4);
}

#[test]
fn cut_square_entirely_exterior() {
    let square_1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(5.0, 5.0))
        .line_to(Coord2(10.0, 5.0))
        .line_to(Coord2(10.0, 10.0))
        .line_to(Coord2(5.0, 10.0))
        .line_to(Coord2(5.0, 5.0))
        .build();

    let square_2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(20.0, 20.0))
        .line_to(Coord2(15.0, 20.0))
        .line_to(Coord2(15.0, 15.0))
        .line_to(Coord2(20.0, 15.0))
        .line_to(Coord2(20.0, 20.0))
        .build();

    let cut_square = path_cut::<_, _, SimpleBezierPath>(&vec![square_1], &vec![square_2], 0.01);

    assert!(cut_square.exterior_path.len() == 1);
    assert!(cut_square.interior_path.len() == 0);

    assert!(cut_square.exterior_path[0].points().len() == 4);
}

#[test]
fn cut_square_center() {
    let square_1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(5.0, 5.0))
        .line_to(Coord2(10.0, 5.0))
        .line_to(Coord2(10.0, 10.0))
        .line_to(Coord2(5.0, 10.0))
        .line_to(Coord2(5.0, 5.0))
        .build();

    let square_2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(6.0, 6.0))
        .line_to(Coord2(9.0, 6.0))
        .line_to(Coord2(9.0, 9.0))
        .line_to(Coord2(6.0, 9.0))
        .line_to(Coord2(6.0, 6.0))
        .build();

    let cut_square = path_cut::<_, _, SimpleBezierPath>(&vec![square_1], &vec![square_2], 0.01);

    assert!(cut_square.exterior_path.len() == 2);
    assert!(cut_square.interior_path.len() == 1);

    assert!(cut_square.interior_path[0].points().len() == 4);
    assert!(cut_square.exterior_path[0].points().len() == 4);
    assert!(cut_square.exterior_path[1].points().len() == 4);
}
