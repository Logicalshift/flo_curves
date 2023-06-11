use criterion::{criterion_group, criterion_main, Criterion};

use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;

use std::f64;

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

fn find_edges<TContour: SampledContour>(contour: TContour) -> Vec<(ContourPosition, ContourCell)> {
    contour.edge_cell_iterator().collect()
}

fn circle_from_contours<TContour: SampledContour>(contour: TContour) {
    // Trace the samples to generate a vector
    trace_paths_from_samples::<SimpleBezierPath>(contour, 0.1);
}

fn create_brush_stroke_daubs() -> Vec<(CircularDistanceField, ContourPosition)> {
    let pos  = 0.3 * 2.0*f64::consts::PI;
    let pos  = (pos.sin() + 1.0) * 200.0;
    let off1 = 200.0 - pos/2.0;
    let off2 = pos/2.0;

    let t  = 0.4f64;
    let p0 = Coord2(-(t*1.0/2.0).cos() * 400.0, (t*1.0/3.0).sin() * 500.0) + Coord2(500.0, 500.0);
    let p1 = Coord2(-(t*2.0/3.0).cos() * 400.0, (t*1.0/4.0).sin() * 200.0) + Coord2(500.0, 500.0);
    let p2 = Coord2((t*1.0/4.0).cos() * 200.0, -(t*2.0/3.0).sin() * 400.0) + Coord2(500.0, 500.0);
    let p3 = Coord2((t*1.0/3.0).cos() * 500.0, -(t*1.0/2.0).sin() * 200.0) + Coord2(500.0, 500.0);

    let p0_3 = Coord3::from((p0, off1));
    let p1_3 = Coord3::from((p1, (off2-off1)*(1.0/3.0) + off1));
    let p2_3 = Coord3::from((p2, (off2-off1)*(2.0/3.0) + off1));
    let p3_3 = Coord3::from((p3, off2));

    let brush_curve      = Curve::from_points(p0_3, (p1_3, p2_3), p3_3);
    let (daubs, _offset) = brush_stroke_daubs::<CircularDistanceField, _>(&brush_curve, 0.5, 0.25);

    daubs.collect::<Vec<_>>()
}

fn read_edge_distances(daubs: &DaubBrushDistanceField<CircularDistanceField>, edges: &Vec<(ContourPosition, ContourCell)>) {
    let mut distances = vec![];
    for (pos, _) in edges.iter() {
        distances.push(daubs.distance_at_point(*pos));
    }
}

fn trace_distance_field(daubs: &DaubBrushDistanceField<CircularDistanceField>) {
    trace_paths_from_distance_field::<SimpleBezierPath>(daubs, 1.0);
}

fn criterion_benchmark(c: &mut Criterion) {
    let circle_100              = sampled_circle(100, 30);
    let circle_1000             = sampled_circle(1000, 300);
    let circle_100_generated    = CircularDistanceField::with_radius(30.0);
    let circle_1000_generated   = CircularDistanceField::with_radius(300.0);
    let daub_distance_field     = DaubBrushDistanceField::from_daubs(create_brush_stroke_daubs());
    let distance_field_edges    = find_edges(&daub_distance_field);

    c.bench_function("find_edges 100", |b| b.iter(|| find_edges(&circle_100)));
    c.bench_function("find_edges 1000", |b| b.iter(|| find_edges(&circle_1000)));
    c.bench_function("find_edges_not_sampled 100", |b| b.iter(|| find_edges(&circle_100_generated)));
    c.bench_function("find_edges_not_sampled 1000", |b| b.iter(|| find_edges(&circle_1000_generated)));

    c.bench_function("circle_from_contours 100", |b| b.iter(|| circle_from_contours(&circle_100)));
    c.bench_function("circle_from_contours 1000", |b| b.iter(|| circle_from_contours(&circle_1000)));
    c.bench_function("circle_from_contours_not_sampled 100", |b| b.iter(|| circle_from_contours(&circle_100_generated)));
    c.bench_function("circle_from_contours_not_sampled 1000", |b| b.iter(|| circle_from_contours(&circle_1000_generated)));

    c.bench_function("create_brush_stroke_daubs", |b| b.iter(|| create_brush_stroke_daubs()));
    c.bench_function("read_brush_stroke_edges", |b| b.iter(|| find_edges(&daub_distance_field)));
    c.bench_function("read_edge_distances", |b| b.iter(|| read_edge_distances(&daub_distance_field, &distance_field_edges)));
    c.bench_function("trace_distance_field", |b| b.iter(|| trace_distance_field(&daub_distance_field)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
