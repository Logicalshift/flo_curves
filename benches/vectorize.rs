use criterion::{criterion_group, criterion_main, Criterion};

use flo_curves::bezier::path::*;
use flo_curves::bezier::vectorize::*;

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

fn criterion_benchmark(c: &mut Criterion) {
    let circle_100              = sampled_circle(100, 30);
    let circle_1000             = sampled_circle(1000, 300);
    let circle_100_generated    = CircularDistanceField::with_radius(30.0);
    let circle_1000_generated   = CircularDistanceField::with_radius(300.0);

    c.bench_function("find_edges 100", |b| b.iter(|| find_edges(&circle_100)));
    c.bench_function("find_edges 1000", |b| b.iter(|| find_edges(&circle_1000)));
    c.bench_function("find_edges_not_sampled 100", |b| b.iter(|| find_edges(&circle_100_generated)));
    c.bench_function("find_edges_not_sampled 1000", |b| b.iter(|| find_edges(&circle_1000_generated)));

    c.bench_function("circle_from_contours 100", |b| b.iter(|| circle_from_contours(&circle_100)));
    c.bench_function("circle_from_contours 1000", |b| b.iter(|| circle_from_contours(&circle_1000)));
    c.bench_function("circle_from_contours_not_sampled 100", |b| b.iter(|| circle_from_contours(&circle_100_generated)));
    c.bench_function("circle_from_contours_not_sampled 1000", |b| b.iter(|| circle_from_contours(&circle_1000_generated)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
