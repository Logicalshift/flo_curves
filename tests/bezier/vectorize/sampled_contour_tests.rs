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
fn filled_iterator() {
    // Fully filled contour, we should detect all of the 'outermost' edges
    let contour = U8SampledContour(ContourSize(3, 3), vec![
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

    let edge_cell_positions = contour.edge_cell_iterator().map(|(pos, _cell)| pos).collect::<Vec<_>>();

    assert!(edge_cell_positions == vec![
            ContourPosition(0, 0),
            ContourPosition(1, 0),
            ContourPosition(2, 0),
            ContourPosition(3, 0),

            ContourPosition(0, 1),
            ContourPosition(3, 1),

            ContourPosition(0, 2),
            ContourPosition(3, 2),

            ContourPosition(0, 3),
            ContourPosition(1, 3),
            ContourPosition(2, 3),
            ContourPosition(3, 3),
        ], "Cell positions are {:?}", edge_cell_positions);
}

