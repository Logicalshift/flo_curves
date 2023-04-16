use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;

use itertools::*;

use std::collections::{HashMap};

fn check_contour_against_bitmap<TContour: SampledContour>(contour: TContour, draw_circle: bool) {
    // Use point_is_inside to generate a bitmap version of the contour
    let bitmap = (0..(contour.size().0 * contour.size().1)).into_iter()
        .map(|pos| (pos % contour.size().1, pos / contour.size().1))
        .map(|(x, y)| contour.point_is_inside(ContourPosition(x, y)))
        .collect::<Vec<_>>();

    if draw_circle {
        for p in 0..bitmap.len() {
            print!("{}", if bitmap[p] { '#' } else { '.' });
            if ((p+1)%contour.size().1) == 0 { println!() };
        }
        println!();
    }

    let bitmap = BoolSampledContour(contour.size(), bitmap);

    // Get the edges from both
    let bitmap_edges    = bitmap.edge_cell_iterator().collect::<Vec<_>>();
    let contour_edges   = contour.edge_cell_iterator().collect::<Vec<_>>();

    // Should generate identical results
    let edges_for_y_bitmap  = bitmap_edges.iter().cloned().group_by(|(pos, _)| pos.1).into_iter().map(|(ypos, group)| (ypos, group.count())).collect::<HashMap<_, _>>();
    let edges_for_y_contour  = contour_edges.iter().cloned().group_by(|(pos, _)| pos.1).into_iter().map(|(ypos, group)| (ypos, group.count())).collect::<HashMap<_, _>>();

    assert!(edges_for_y_bitmap.len() == edges_for_y_contour.len(), "Returned different number of lines ({} vs {})", edges_for_y_bitmap.len(), edges_for_y_contour.len());
    assert!(contour_edges.len() == bitmap_edges.len(), "Returned different number of edges ({} vs {}). Edges counts were: \n  {}\n\nBitmap edges were \n  {}\n\nContour edges were \n  {}",
        bitmap_edges.len(),
        contour_edges.len(),
        edges_for_y_bitmap.keys().map(|ypos| format!("{} {:?} {:?}", ypos, edges_for_y_bitmap.get(ypos), edges_for_y_contour.get(ypos))).collect::<Vec<_>>().join("\n  "),
        bitmap_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "),
        contour_edges.iter().map(|edge| format!("{:?}", edge)).collect::<Vec<_>>().join("\n  "));

    assert!(contour_edges == bitmap_edges, "Edges were \n  {}", 
        bitmap_edges.iter().zip(contour_edges.iter())
            .map(|(bitmap_edge, contour_edge)| format!("({:?}) {:?}    {:?}", bitmap_edge == contour_edge, bitmap_edge, contour_edge))
            .collect::<Vec<_>>()
            .join("\n  "));
}

#[test]
fn zero_size_circle() {
    let contour = CircularDistanceField::with_radius(0.0);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn teeny_circle() {
    let contour = CircularDistanceField::with_radius(0.5);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn even_radius_circular_contour() {
    let contour = CircularDistanceField::with_radius(16.0);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn odd_radius_circular_contour() {
    let contour = CircularDistanceField::with_radius(15.0);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn non_grid_aligned_circular_contour() {
    let contour = CircularDistanceField::with_radius(16.1);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn many_circles() {
    // All circles up to a radius of 100 in steps of 0.1
    for radius in 0..1000 {
        let radius  = (radius as f64) / 10.0;
        let contour = CircularDistanceField::with_radius(radius);
        check_contour_against_bitmap(&contour, false);
    }
}

#[test]
fn many_circles_small_increments() {
    // All circles up to a radius of 10 in steps of 0.01
    for radius in 0..1000 {
        let radius  = (radius as f64) / 100.0;
        let contour = CircularDistanceField::with_radius(radius);
        check_contour_against_bitmap(&contour, false);
    }
}

#[test]
fn circle_path_from_contours() {
    // Create a contour containing a circle in the middle, using the circular distance field
    let radius  = 30.0;
    let contour = CircularDistanceField::with_radius(radius);

    let size    = contour.size().0;
    let center  = (size as f64)/2.0 - 1.0;

    // Trace the samples to generate a vector
    let circle = trace_paths_from_samples::<SimpleBezierPath>(&contour, 1.5);

    // Should contain a single path
    assert!(circle.len() == 1, "{:?}", circle);

    // Allow 1.5px of error (between the fitting algorithm and the sampled circle itself)
    let mut max_error = 0.0;

    for curve in circle[0].to_curves::<Curve<Coord2>>() {
        for t in 0..100 {
            let t           = (t as f64)/100.0;
            let point       = curve.point_at_pos(t);
            let distance    = point.distance_to(&Coord2(center+1.0, center+1.0));
            let offset      = (distance-radius).abs();

            max_error = f64::max(max_error, offset);
        }
    }

    // The error here is semi-random due to the hash table used to store the edge graph
    assert!(max_error <= 1.5, "Max error {:?} > 1.5. Path generated was {:?}", max_error, circle);
}
