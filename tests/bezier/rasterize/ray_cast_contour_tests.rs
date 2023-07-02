use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;
use flo_curves::bezier::rasterize::*;

use itertools::*;
use smallvec::*;

use std::collections::{HashMap};

fn create_circle(radius: f64) -> DynRayCastContour {
    let radius      = if radius < 0.0 { 0.0 } else { radius };
    let center      = radius.ceil() + 1.0;
    let diameter    = (center as usize) * 2 + 1;

    DynRayCastContour::new(Box::new(move |y| {
        let y = y - center;

        if y.abs() <= radius {
            let intercept   = ((radius*radius) - (y*y)).sqrt();
            let min_x       = center - intercept;
            let max_x       = center + intercept;

            smallvec![min_x..max_x]
        } else {
            smallvec![]
        }
    }), ContourSize(diameter, diameter))
}

fn draw<TContour: SampledContour>(contour: TContour) {
    let bitmap = (0..(contour.contour_size().0 * contour.contour_size().1)).into_iter()
        .map(|pos| (pos % contour.contour_size().1, pos / contour.contour_size().1))
        .map(|(x, y)| contour_point_is_inside(contour, ContourPosition(x, y)))
        .collect::<Vec<_>>();

    for p in 0..bitmap.len() {
        print!("{}", if bitmap[p] { '#' } else { '.' });
        if ((p+1)%contour.contour_size().1) == 0 { println!() };
    }
    println!();
}

fn check_contour_against_bitmap<TContour: SampledContour>(contour: TContour, draw_circle: bool) {
    // Use point_is_inside to generate a bitmap version of the contour
    let bitmap = (0..(contour.contour_size().0 * contour.contour_size().1)).into_iter()
        .map(|pos| (pos % contour.contour_size().0, pos / contour.contour_size().0))
        .map(|(x, y)| contour_point_is_inside(contour, ContourPosition(x, y)))
        .collect::<Vec<_>>();

    if draw_circle {
        draw(contour);
    }

    let bitmap = BoolSampledContour(contour.contour_size(), bitmap);

    // Get the edges from both
    let bitmap_edges    = bitmap.edge_cell_iterator().collect::<Vec<_>>();
    let contour_edges   = contour.edge_cell_iterator().collect::<Vec<_>>();

    // Should generate identical results
    let edges_for_y_bitmap  = bitmap_edges.iter().cloned().group_by(|(pos, _)| pos.1).into_iter().map(|(ypos, group)| (ypos, group.count())).collect::<HashMap<_, _>>();
    let edges_for_y_contour  = contour_edges.iter().cloned().group_by(|(pos, _)| pos.1).into_iter().map(|(ypos, group)| (ypos, group.count())).collect::<HashMap<_, _>>();

    assert!(edges_for_y_bitmap.len() == edges_for_y_contour.len(), "Returned different number of lines (bitmap has {} vs contour with {})\n{:?}\n\n{:?}", edges_for_y_bitmap.len(), edges_for_y_contour.len(), bitmap_edges, contour_edges);
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

    check_intercepts(contour);
}

fn check_intercepts<TContour: SampledContour>(contour: TContour) {
    let mut num_empty = 0;

    for y in 0..contour.contour_size().height() {
        let intercepts  = contour.rounded_intercepts_on_line(y as _);
        let mut row     = vec![false; contour.contour_size().width()];

        if intercepts.len() == 0 {
            num_empty += 1;
        }

        for intercept in intercepts.iter() {
            for x in (intercept.start as usize)..(intercept.end as usize) {
                assert!(row[x] == false, "Overlapping intercept at {}, {}", x, y);
                row[x] = true;
            }
        }

        for x in 0..contour.contour_size().width() {
            assert!(row[x] == contour_point_is_inside(contour, ContourPosition(x, y)), "Row content mismatch at y={} (intercepts look like:\n  {} but should be:\n  {})", y,
                row.iter().map(|p| if *p { '#' } else { '.' }).collect::<String>(),
                (0..contour.contour_size().width()).into_iter().map(|x| if contour_point_is_inside(contour, ContourPosition(x, y)) { '#' } else { '.' }).collect::<String>());
        }
    }

    assert!(num_empty < 8, "{:?} empty rows", num_empty);
}

#[test]
fn zero_size_circle() {
    let contour = create_circle(0.0);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn teeny_circle() {
    let contour = create_circle(0.5);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn medium_circle_1() {
    let contour = create_circle(32.0);
    check_contour_against_bitmap(&contour, false);
}

#[test]
fn medium_circle_2() {
    let contour = create_circle(31.4);
    check_contour_against_bitmap(&contour, false);
}

#[test]
fn medium_circle_3() {
    let contour = create_circle(5.0);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn big_circle() {
    let contour = create_circle(2345.1234);
    check_contour_against_bitmap(&contour, false);
}

#[test]
fn even_radius_circular_contour() {
    let contour = create_circle(16.0);

    assert!(contour.contour_size().width() == 35, "{:?}", contour.contour_size());

    check_contour_against_bitmap(&contour, true);
}

#[test]
fn odd_radius_circular_contour() {
    let contour = create_circle(15.0);

    assert!(contour.contour_size().width() == 33, "{:?}", contour.contour_size());

    check_contour_against_bitmap(&contour, true);
}

#[test]
fn non_grid_aligned_circular_contour() {
    let contour = create_circle(16.1);
    check_contour_against_bitmap(&contour, true);
}

#[test]
fn many_circles() {
    // All circles up to a radius of 100 in steps of 0.1
    for radius in 0..1000 {
        let radius  = (radius as f64) / 10.0;
        let contour = create_circle(radius);
        check_contour_against_bitmap(&contour, false);
    }
}

#[test]
fn many_circles_small_increments() {
    // All circles up to a radius of 10 in steps of 0.01
    for radius in 0..1000 {
        let radius  = (radius as f64) / 100.0;
        let contour = create_circle(radius);
        check_contour_against_bitmap(&contour, false);
    }
}

#[test]
fn circle_path_from_contours() {
    // Create a contour containing a circle in the middle, using the circular distance field
    let radius  = 30.0;
    let contour = create_circle(radius);

    let size    = contour.contour_size().0;
    let center  = ((size as f64)/2.0).floor();

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
