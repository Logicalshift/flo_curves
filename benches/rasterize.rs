use criterion::{criterion_group, criterion_main, Criterion};

use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

use smallvec::*;

use std::ops::{Range};

fn scan_convert_path(path: &Vec<SimpleBezierPath>) -> Vec<SmallVec<[Range<usize>; 4]>> {
    let scan_converter = PathContour::from_path(path.clone(), ContourSize(1000, 1000));
    (0..1000).map(|y| scan_converter.rounded_intercepts_on_line(y as f64)).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    c.bench_function("scan_convert_circle", |b| b.iter(|| scan_convert_path(&vec![circle_path.clone()])));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
