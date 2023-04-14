use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;

#[test]
fn basic_circle() {
    let radius              = 300.0;
    let center              = Coord2(500.0, 500.0);
    let circle_path         = Circle::new(center, radius).to_path::<SimpleBezierPath>();
    let scan_converter      = BezierPathScanConverter::new(0..1000);

    let circle_points       = scan_converter.scan_convert(&vec![circle_path]).collect::<Vec<_>>();

    let mut last_scanline_point_count   = 2;
    let mut current_scanline            = -1;
    for point in circle_points {
        match point {
            ScanEdgeFragment::StartScanline(new_scanline) => { 
                assert!(last_scanline_point_count == 2, "Last scanline point count count was {:?}", last_scanline_point_count);
                assert!(new_scanline > current_scanline, "Scanlines went backwards: {:?} -> {:?}", current_scanline, new_scanline);
                assert!(current_scanline == -1 || new_scanline == current_scanline + 1, "Missed scanline: {:?} -> {:?}", current_scanline, new_scanline);

                current_scanline            = new_scanline; 
                last_scanline_point_count   = 0;
            }

            ScanEdgeFragment::Edge(ScanX(x_pos), _fragment) => {
                last_scanline_point_count += 1;

                let y_pos       = current_scanline as f64;
                let pos         = Coord2(x_pos, y_pos);
                let distance    = pos.distance_to(&center);

                assert!((distance - radius).abs() < 0.1, "Point was {:?} units from the center of the circle", distance);
            }
        }
    }

    assert!(last_scanline_point_count == 2, "Last scanline point count count was {:?}", last_scanline_point_count);
}
