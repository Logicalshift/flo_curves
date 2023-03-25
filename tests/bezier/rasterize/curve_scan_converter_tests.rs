use flo_curves::bezier::*;
use flo_curves::bezier::rasterize::*;

#[test]
fn basic_line() {
    let line            = Curve::from_points(Coord2(0.0, 0.0), (Coord2(10.0, 10.0), Coord2(20.0, 20.0)), Coord2(30.0, 30.0));
    let scan_converter  = RootSolvingScanConverter::new(0..1000);

    let mut line_points = scan_converter.scan_convert(&line);

    for scanline in 0..=30 {
        let start_scanline = line_points.next();
        assert!(start_scanline == Some(ScanEdgeFragment::StartScanline(scanline)), "Expected StartScanline({}), got {:?}", scanline, start_scanline);

        let edge_fragment = line_points.next();
        if let Some(ScanEdgeFragment::Edge(ScanX(x), _)) = edge_fragment {
            assert!((x-(scanline as f64)).abs() < 0.1, "Xpos should be {} but was {:?}", scanline, x);
        } else {
            assert!(false, "Was expecting an edge fragment, got {:?}", edge_fragment);
        }
    }

    assert!(line_points.next() == None);
}