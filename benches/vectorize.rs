use criterion::{criterion_group, criterion_main, Criterion};

use flo_curves::geo::*;
use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;
use flo_curves::bezier::rasterize::*;

use smallvec::*;

use std::f64;
use std::ops::{Range};

fn sampled_circle(size: usize, radius: usize) -> BoolSampledContour {
    // Create a contour containing a circle in the middle
    let radius  = radius as f64;
    let center  = (size/2) as f64;
    let contour = (0..(size*size)).into_iter()
        .map(|pos| {
            let x = (pos % size) as f64;
            let y = (pos / size) as f64;
            let x = x - center;
            let y = y - center;

            let r_squared = (x*x) + (y*y);
            if r_squared > radius * radius {
                false
            } else {
                true
            }
        })
        .collect();

    let contour = BoolSampledContour(ContourSize(size, size), contour);

    contour
}

fn start_edge_iteration<'a, TContour: SampledContour>(contour: &'a TContour) -> impl 'a + Iterator<Item=(ContourPosition, ContourCell)> {
    contour.edge_cell_iterator()
}

fn scan_intercepts<TContour: SampledContour>(contour: TContour) -> Vec<SmallVec<[Range<f64>; 4]>> {
    let mut result = vec![];

    for y in 0..contour.contour_size().height() {
        result.push(contour.intercepts_on_line(y as _));
    }

    result
}

fn find_edges<TContour: SampledContour>(contour: TContour) -> Vec<(ContourPosition, ContourCell)> {
    contour.edge_cell_iterator().collect()
}

fn circle_from_contours<TContour: SampledContour>(contour: TContour) {
    // Trace the samples to generate a vector
    trace_paths_from_samples::<SimpleBezierPath>(&contour, 0.1);
}

fn create_brush_stroke(brush_size: f64) -> Curve<Coord3> {
    let pos  = 0.3 * 2.0*f64::consts::PI;
    let pos  = (pos.sin() + 1.0) * 200.0;
    let off1 = 200.0 - pos/2.0;
    let off2 = pos/2.0;

    let off1 = off1 * (brush_size/200.0);
    let off2 = off2 * (brush_size/200.0);

    let t  = 0.4f64;
    let p0 = Coord2(-(t*1.0/2.0).cos() * 400.0, (t*1.0/3.0).sin() * 500.0) + Coord2(500.0, 500.0);
    let p1 = Coord2(-(t*2.0/3.0).cos() * 400.0, (t*1.0/4.0).sin() * 200.0) + Coord2(500.0, 500.0);
    let p2 = Coord2((t*1.0/4.0).cos() * 200.0, -(t*2.0/3.0).sin() * 400.0) + Coord2(500.0, 500.0);
    let p3 = Coord2((t*1.0/3.0).cos() * 500.0, -(t*1.0/2.0).sin() * 200.0) + Coord2(500.0, 500.0);

    let p0_3 = Coord3::from((p0, off1));
    let p1_3 = Coord3::from((p1, (off2-off1)*(1.0/3.0) + off1));
    let p2_3 = Coord3::from((p2, (off2-off1)*(2.0/3.0) + off1));
    let p3_3 = Coord3::from((p3, off2));

    let brush_curve = Curve::from_points(p0_3, (p1_3, p2_3), p3_3);

    brush_curve
}

fn create_lms_offset_curve(curve: Curve<Coord3>) -> Vec<Curve<Coord2>> {
    let (sp, (cp1, cp2), ep)    = curve.all_points();
    let base_curve              = Curve::from_points(Coord2(sp.x(), sp.y()), (Coord2(cp1.x(), cp1.y()), Coord2(cp2.x(), cp2.y())), Coord2(ep.x(), ep.y()));
    let distance_curve          = Curve::from_points(sp.z(), (cp1.z(), cp2.z()), ep.z());

    let offset_1                = offset_lms_sampling(&base_curve, |t| distance_curve.point_at_pos(t), |_| 0.0, 400, 0.1).unwrap();
    let offset_2                = offset_lms_sampling(&base_curve, |t| -distance_curve.point_at_pos(t), |_| 0.0, 400, 0.1).unwrap();

    offset_1.into_iter().chain(offset_2).collect()
}

fn create_brush_stroke_daubs(brush_size: f64) -> Vec<(CircularDistanceField, ContourPosition)> {
    let brush_curve         = create_brush_stroke(brush_size);
    let (daubs, _offset)    = brush_stroke_daubs_from_curve(&CircularBrush, &brush_curve, 0.5, 0.25);

    daubs.collect::<Vec<_>>()
}

fn read_edge_distances(daubs: &DaubBrushDistanceField<CircularDistanceField>, edges: &Vec<(ContourPosition, ContourCell)>) {
    let mut distances = vec![];
    for (pos, _) in edges.iter() {
        distances.push(daubs.distance_at_point(*pos));
    }
}

fn trace_distance_field<TDistanceField>(daubs: &TDistanceField) 
where
    TDistanceField: SampledSignedDistanceField
{
    trace_paths_from_distance_field::<SimpleBezierPath>(daubs, 1.0);
}

fn criterion_benchmark(c: &mut Criterion) {
    let circle_100              = sampled_circle(100, 30);
    let circle_1000             = sampled_circle(1000, 300);
    let circle_100_generated    = CircularDistanceField::with_radius(30.0);
    let circle_1000_generated   = CircularDistanceField::with_radius(300.0);
    let daub_distance_field     = DaubBrushDistanceField::from_daubs(create_brush_stroke_daubs(200.0));
    let distance_field_edges    = find_edges(&daub_distance_field);

    let circle_path_1000        = Circle::new(Coord2(500.0, 500.0), 300.0).to_path::<SimpleBezierPath>();
    let path_contour_1000       = PathContour::from_path(vec![circle_path_1000], ContourSize(1000, 1000));

    c.bench_function("offset_curves", |b| b.iter(|| create_lms_offset_curve(create_brush_stroke(200.0))));

    c.bench_function("find_edges 100", |b| b.iter(|| find_edges(&circle_100)));
    c.bench_function("find_edges 1000", |b| b.iter(|| find_edges(&circle_1000)));
    c.bench_function("find_edges_not_sampled 100", |b| b.iter(|| find_edges(&circle_100_generated)));
    c.bench_function("find_edges_not_sampled 1000", |b| b.iter(|| find_edges(&circle_1000_generated)));

    c.bench_function("circle_from_contours 100", |b| b.iter(|| circle_from_contours(&circle_100)));
    c.bench_function("circle_from_contours 1000", |b| b.iter(|| circle_from_contours(&circle_1000)));
    c.bench_function("circle_intercepts_scan_sampled 100", |b| b.iter(|| scan_intercepts(&circle_100)));
    c.bench_function("circle_intercepts_scan_sampled 1000", |b| b.iter(|| scan_intercepts(&circle_1000)));
    c.bench_function("circle_intercepts_scan 100", |b| b.iter(|| scan_intercepts(&circle_100_generated)));
    c.bench_function("circle_intercepts_scan 1000", |b| b.iter(|| scan_intercepts(&circle_1000_generated)));
    c.bench_function("circle_start_iteration", |b| b.iter(|| start_edge_iteration(&circle_1000_generated)));
    c.bench_function("circle_from_contours_not_sampled 100", |b| b.iter(|| circle_from_contours(&circle_100_generated)));
    c.bench_function("circle_from_contours_not_sampled 1000", |b| b.iter(|| circle_from_contours(&circle_1000_generated)));

    c.bench_function("circle_path_intercepts_scan 1000", |b| b.iter(|| scan_intercepts(&path_contour_1000)));
    c.bench_function("circle_path_trace 1000", |b| b.iter(|| circle_from_contours(&path_contour_1000)));

    c.bench_function("create_brush_stroke_daubs", |b| b.iter(|| create_brush_stroke_daubs(200.0)));
    c.bench_function("create_brush_distance_field", |b| b.iter(|| DaubBrushDistanceField::from_daubs(create_brush_stroke_daubs(200.0))));
    c.bench_function("start_brush_iteration", |b| b.iter(|| start_edge_iteration(&daub_distance_field)));
    c.bench_function("brush_intercepts_scan", |b| b.iter(|| scan_intercepts(&daub_distance_field)));
    c.bench_function("read_brush_stroke_edges", |b| b.iter(|| find_edges(&daub_distance_field)));
    c.bench_function("read_edge_distances", |b| b.iter(|| read_edge_distances(&daub_distance_field, &distance_field_edges)));
    c.bench_function("trace_distance_field", |b| b.iter(|| trace_distance_field(&daub_distance_field)));

    c.bench_function("single_daub", |b| b.iter(|| {
       let distance_field = DaubBrushDistanceField::from_daubs(vec![(CircularDistanceField::with_radius(300.0), ContourPosition(0, 0))]);
       trace_distance_field(&distance_field);  
    }));
    c.bench_function("single_small_daub", |b| b.iter(|| {
       let distance_field = DaubBrushDistanceField::from_daubs(vec![(CircularDistanceField::with_radius(10.0), ContourPosition(0, 0))]);
       trace_distance_field(&distance_field);  
    }));
    c.bench_function("ten_daubs", |b| b.iter(|| {
       let distance_field = DaubBrushDistanceField::from_daubs((0..10).map(|x| (CircularDistanceField::with_radius(300.0), ContourPosition(0, x))));
       trace_distance_field(&distance_field);  
    }));
    c.bench_function("hundred_daubs", |b| b.iter(|| {
       let distance_field = DaubBrushDistanceField::from_daubs((0..100).map(|x| (CircularDistanceField::with_radius(300.0), ContourPosition(0, x))));
       trace_distance_field(&distance_field);  
    }));
    c.bench_function("hundred_small_daubs", |b| b.iter(|| {
       let distance_field = DaubBrushDistanceField::from_daubs((0..100).map(|x| (CircularDistanceField::with_radius(10.0), ContourPosition(0, x))));
       trace_distance_field(&distance_field);  
    }));
    c.bench_function("hundred_daubs_horiz", |b| b.iter(|| {
       let distance_field = DaubBrushDistanceField::from_daubs((0..100).map(|x| (CircularDistanceField::with_radius(300.0), ContourPosition(x, 0))));
       trace_distance_field(&distance_field);  
    }));
    c.bench_function("hundred_small_daubs_horiz", |b| b.iter(|| {
       let distance_field = DaubBrushDistanceField::from_daubs((0..100).map(|x| (CircularDistanceField::with_radius(10.0), ContourPosition(x, 0))));
       trace_distance_field(&distance_field);  
    }));
    c.bench_function("full_distance_field", |b| b.iter(|| {
        let daub_distance_field = DaubBrushDistanceField::from_daubs(create_brush_stroke_daubs(200.0));
        trace_distance_field(&daub_distance_field)
    }));
    c.bench_function("full_distance_field_small_brush", |b| b.iter(|| {
        let daub_distance_field = DaubBrushDistanceField::from_daubs(create_brush_stroke_daubs(20.0));
        trace_distance_field(&daub_distance_field)
    }));
    c.bench_function("full_distance_field_path_brush", |b| b.iter(|| {
        let radius          = 32.0;
        let center          = Coord2(radius+1.0, radius+1.0);
        let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();
        let circle_field    = PathDistanceField::from_path(vec![circle_path], ContourSize(65, 65));

        let brush           = ScaledBrush::from_distance_field(&circle_field);

        let brush_curve      = create_brush_stroke(20.0);
        let brush            = &brush;
        let (daubs, _offset) = brush_stroke_daubs_from_curve(&brush, &brush_curve, 0.5, 0.25);

        let daub_distance_field = DaubBrushDistanceField::from_daubs(daubs);

        trace_distance_field(&daub_distance_field)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
