use criterion::{criterion_group, criterion_main, Criterion};

use flo_curves::arc::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::geo::*;

fn scan_convert_circle(path: &Vec<SimpleBezierPath>) -> Vec<ScanEdgeFragment> {
    let scan_converter = BezierPathScanConverter::new(0..1000);
    scan_converter.scan_convert(&path).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let radius      = 300.0;
    let center      = Coord2(500.0, 500.0);
    let circle_path = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    c.bench_function("scan_convert_circle", |b| b.iter(|| scan_convert_circle(&vec![circle_path.clone()])));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
