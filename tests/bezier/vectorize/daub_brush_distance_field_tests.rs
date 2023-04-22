use flo_curves::bezier::vectorize::*;

#[test]
fn overlapping_circles_point_inside_first() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let circle_2        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(5, 5)),
        (&circle_2, ContourPosition(5, 12)),
    ]);

    assert!(distance_field.as_contour().point_is_inside(ContourPosition(15, 8)));
}

#[test]
fn overlapping_circles_point_inside_second() {
    let circle_1        = CircularDistanceField::with_radius(10.0);
    let circle_2        = CircularDistanceField::with_radius(10.0);
    let distance_field  = DaubBrushDistanceField::from_daubs(vec![
        (&circle_1, ContourPosition(5, 5)),
        (&circle_2, ContourPosition(5, 12)),
    ]);

    assert!(distance_field.as_contour().point_is_inside(ContourPosition(15, 15)));
}