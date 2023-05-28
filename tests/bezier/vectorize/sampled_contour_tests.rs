use flo_curves::bezier::vectorize::*;

#[test]
fn single_sample_iterator() {
    // Single 'inside' sample, should produce 4 edge cells
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            0, 0, 0,
            0, 1, 0,
            0, 0, 0,
        ]);

    let edge_cell_positions = contour.edge_cell_iterator().map(|(pos, _cell)| pos).collect::<Vec<_>>();

    assert!(edge_cell_positions == vec![
            ContourPosition(1, 1),
            ContourPosition(2, 1),
            ContourPosition(1, 2),
            ContourPosition(2, 2),
        ], "Cell positions are {:?}", edge_cell_positions);
}

#[test]
fn shift_left_1() {
    assert!(ContourCell::from_corners(true, true, true, true).shift_left() == ContourCell::from_corners(true, false, true, false), "{:?}", ContourCell::from_corners(true, true, true, true).shift_left());
}

#[test]
fn shift_left_2() {
    assert!(ContourCell::from_corners(true, true, true, false).shift_left() == ContourCell::from_corners(true, false, false, false), "{:?}", ContourCell::from_corners(true, true, true, false).shift_left());
}

#[test]
fn shift_left_3() {
    assert!(ContourCell::from_corners(true, false, true, true).shift_left() == ContourCell::from_corners(false, false, true, false), "{:?}", ContourCell::from_corners(true, true, false, true).shift_left());
}

#[test]
fn shift_left_4() {
    assert!(ContourCell::from_corners(true, false, true, false).shift_left() == ContourCell::from_corners(false, false, false, false), "{:?}", ContourCell::from_corners(true, true, false, false).shift_left());
}

#[test]
fn shift_left_5() {
    assert!(ContourCell::from_corners(false, true, false, true).shift_left() == ContourCell::from_corners(true, false, true, false), "{:?}", ContourCell::from_corners(true, true, true, true).shift_left());
}

#[test]
fn merge() {
    assert!(ContourCell::from_corners(true, false, true, false).merge(ContourCell::from_corners(true, true, false, false)) == ContourCell::from_corners(true, true, true, false));
}

#[test]
fn filled_iterator() {
    // Fully filled contour, we should detect all of the 'outermost' edges
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

    let edge_cell_positions = contour.edge_cell_iterator().collect::<Vec<_>>();

    assert!(edge_cell_positions == vec![
            (ContourPosition(0, 0), ContourCell::from_corners(false, false, false, true)),
            (ContourPosition(1, 0), ContourCell::from_corners(false, false, true, true)),
            (ContourPosition(2, 0), ContourCell::from_corners(false, false, true, true)),
            (ContourPosition(3, 0), ContourCell::from_corners(false, false, true, false)),

            (ContourPosition(0, 1), ContourCell::from_corners(false, true, false, true)),
            (ContourPosition(3, 1), ContourCell::from_corners(true, false, true, false)),

            (ContourPosition(0, 2), ContourCell::from_corners(false, true, false, true)),
            (ContourPosition(3, 2), ContourCell::from_corners(true, false, true, false)),

            (ContourPosition(0, 3), ContourCell::from_corners(false, true, false, false)),
            (ContourPosition(1, 3), ContourCell::from_corners(true, true, false, false)),
            (ContourPosition(2, 3), ContourCell::from_corners(true, true, false, false)),
            (ContourPosition(3, 3), ContourCell::from_corners(true, false, false, false)),
        ], "Cell positions are {:?}", edge_cell_positions);
}

#[test]
fn filled_with_border_iterator() {
    // Fully filled contour, we should detect all of the 'outermost' edges
    let contour = U8SampledContour(ContourSize(5, 5), vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 0, 0, 0, 0,
        ]);

    let edge_cell_positions = contour.edge_cell_iterator().collect::<Vec<_>>();

    assert!(edge_cell_positions == vec![
            (ContourPosition(1, 1), ContourCell::from_corners(false, false, false, true)),
            (ContourPosition(2, 1), ContourCell::from_corners(false, false, true, true)),
            (ContourPosition(3, 1), ContourCell::from_corners(false, false, true, true)),
            (ContourPosition(4, 1), ContourCell::from_corners(false, false, true, false)),

            (ContourPosition(1, 2), ContourCell::from_corners(false, true, false, true)),
            (ContourPosition(4, 2), ContourCell::from_corners(true, false, true, false)),

            (ContourPosition(1, 3), ContourCell::from_corners(false, true, false, true)),
            (ContourPosition(4, 3), ContourCell::from_corners(true, false, true, false)),

            (ContourPosition(1, 4), ContourCell::from_corners(false, true, false, false)),
            (ContourPosition(2, 4), ContourCell::from_corners(true, true, false, false)),
            (ContourPosition(3, 4), ContourCell::from_corners(true, true, false, false)),
            (ContourPosition(4, 4), ContourCell::from_corners(true, false, false, false)),
        ], "Cell positions are {:?}", edge_cell_positions);
}

#[test]
fn perimeter_iterator() {
    // Two loops, one inner, one outer
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            1, 1, 1,
            1, 0, 1,
            1, 1, 1,
        ]);

    let edge_cell_positions = contour.edge_cell_iterator().map(|(pos, _cell)| pos).collect::<Vec<_>>();

    assert!(edge_cell_positions == vec![
            ContourPosition(0, 0),
            ContourPosition(1, 0),
            ContourPosition(2, 0),
            ContourPosition(3, 0),

            ContourPosition(0, 1),
            ContourPosition(1, 1),
            ContourPosition(2, 1),
            ContourPosition(3, 1),

            ContourPosition(0, 2),
            ContourPosition(1, 2),
            ContourPosition(2, 2),
            ContourPosition(3, 2),

            ContourPosition(0, 3),
            ContourPosition(1, 3),
            ContourPosition(2, 3),
            ContourPosition(3, 3),
        ], "Cell positions are {:?}", edge_cell_positions);
}
