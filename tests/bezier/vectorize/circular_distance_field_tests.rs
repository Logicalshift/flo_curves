use flo_curves::bezier::vectorize::*;

#[test]
fn contour_vs_bitmap_1() {
    // Create a distance field contour
    let contour = CircularDistanceField::with_radius(16.0);

    // Use point_is_inside to generate a bitmap version of the contour
    let bitmap = (0..(contour.size().0 * contour.size().1)).into_iter()
        .map(|pos| (pos % contour.size().1, pos / contour.size().1))
        .map(|(x, y)| contour.point_is_inside(ContourPosition(x, y)))
        .collect::<Vec<_>>();
    let bitmap = BoolSampledContour(contour.size(), bitmap);

    // Get the edges from both
    let contour_edges   = contour.edge_cell_iterator().collect::<Vec<_>>();
    let bitmap_edges    = bitmap.edge_cell_iterator().collect::<Vec<_>>();

    // Should generate identical results
    assert!(contour_edges.len() == bitmap_edges.len(), "Returned different number of edges. Bitmap edges were \n  {}\n\nContour edges were \n  {}",
        bitmap_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "),
        contour_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "));

    assert!(contour_edges == bitmap_edges, "Edges were \n  {}", 
        bitmap_edges.iter().zip(contour_edges.iter())
            .map(|(bitmap_edge, contour_edge)| format!("({:?}) {:?}    {:?}", bitmap_edge == contour_edge, bitmap_edge, contour_edge))
            .collect::<Vec<_>>()
            .join("\n  "));
}

#[test]
fn contour_vs_bitmap_2() {
    // Create a distance field contour
    let contour = CircularDistanceField::with_radius(15.0);

    // Use point_is_inside to generate a bitmap version of the contour
    let bitmap = (0..(contour.size().0 * contour.size().1)).into_iter()
        .map(|pos| (pos % contour.size().1, pos / contour.size().1))
        .map(|(x, y)| contour.point_is_inside(ContourPosition(x, y)))
        .collect::<Vec<_>>();
    let bitmap = BoolSampledContour(contour.size(), bitmap);

    // Get the edges from both
    let contour_edges   = contour.edge_cell_iterator().collect::<Vec<_>>();
    let bitmap_edges    = bitmap.edge_cell_iterator().collect::<Vec<_>>();

    // Should generate identical results
    assert!(contour_edges.len() == bitmap_edges.len(), "Returned different number of edges. Bitmap edges were \n  {}\n\nContour edges were \n  {}",
        bitmap_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "),
        contour_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "));

    assert!(contour_edges == bitmap_edges, "Edges were \n  {}", 
        bitmap_edges.iter().zip(contour_edges.iter())
            .map(|(bitmap_edge, contour_edge)| format!("({:?}) {:?}    {:?}", bitmap_edge == contour_edge, bitmap_edge, contour_edge))
            .collect::<Vec<_>>()
            .join("\n  "));
}

#[test]
fn contour_vs_bitmap_3() {
    // Create a distance field contour
    let contour = CircularDistanceField::with_radius(16.1);

    // Use point_is_inside to generate a bitmap version of the contour
    let bitmap = (0..(contour.size().0 * contour.size().1)).into_iter()
        .map(|pos| (pos % contour.size().1, pos / contour.size().1))
        .map(|(x, y)| contour.point_is_inside(ContourPosition(x, y)))
        .collect::<Vec<_>>();
    let bitmap = BoolSampledContour(contour.size(), bitmap);

    // Get the edges from both
    let contour_edges   = contour.edge_cell_iterator().collect::<Vec<_>>();
    let bitmap_edges    = bitmap.edge_cell_iterator().collect::<Vec<_>>();

    // Should generate identical results
    assert!(contour_edges.len() == bitmap_edges.len(), "Returned different number of edges. Bitmap edges were \n  {}\n\nContour edges were \n  {}",
        bitmap_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "),
        contour_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "));

    assert!(contour_edges == bitmap_edges, "Edges were \n  {}", 
        bitmap_edges.iter().zip(contour_edges.iter())
            .map(|(bitmap_edge, contour_edge)| format!("({:?}) {:?}    {:?}", bitmap_edge == contour_edge, bitmap_edge, contour_edge))
            .collect::<Vec<_>>()
            .join("\n  "));
}
