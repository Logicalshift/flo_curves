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

#[test]
fn wave_horizontal() {
    let wave            = Curve::from_points(Coord2(10.0, 10.0), (Coord2(30.0, 20.0), Coord2(50.0, 0.0)), Coord2(70.0, 10.0));
    let scan_converter  = RootSolvingScanConverter::new(0..1000);

    let wave_points     = scan_converter.scan_convert(&wave);

    let mut square      = vec![' '; 80*80];
    let mut y           = 0;
    for p in wave_points {
        match p {
            ScanEdgeFragment::StartScanline(new_y)  => { y = new_y; },
            ScanEdgeFragment::Edge(ScanX(x), ScanFragment { t, .. })   => {
                let t_point         = wave.point_at_pos(t);
                let distance        = t_point.distance_to(&Coord2(x, y as f64));

                println!("t = {}, p = {:?}", t, t_point);

                assert!(distance < 0.01, "{:?} is {} units away from where it should be (actual point is {:?})", Coord2(x, y as f64), distance, t_point);
                assert!(t >= 0.0, "t={} (negative numbers should not be in the results", t);
                assert!(t <= 1.0, "t={} (>1 should not be in the results", t);

                let closest_point   = wave.nearest_point(&Coord2(x, y as f64));
                let distance        = closest_point.distance_to(&Coord2(x, y as f64));
                assert!(distance < 0.01, "{:?} is {} units away from where it should be", Coord2(x, y as f64), distance);

                let x = x as i64; 
                square[(x + y*80) as usize] = 'o'; 
            }
        }
    }

    for y in 0..80 {
        for x in 0..80 {
            print!("{}", square[(x + y*80) as usize]);
        }
        println!();
    }
}

#[test]
fn wave_vertical() {
    let wave            = Curve::from_points(Coord2(10.0, 10.0), (Coord2(20.0, 30.0), Coord2(0.0, 50.0)), Coord2(10.0, 70.0));
    let scan_converter  = RootSolvingScanConverter::new(0..1000);

    let wave_points     = scan_converter.scan_convert(&wave);

    let mut square      = vec![' '; 80*80];
    let mut y           = 0;
    for p in wave_points {
        match p {
            ScanEdgeFragment::StartScanline(new_y)                      => { y = new_y; },
            ScanEdgeFragment::Edge(ScanX(x), ScanFragment { t, .. })   => {
                let t_point         = wave.point_at_pos(t);
                let distance        = t_point.distance_to(&Coord2(x, y as f64));
                assert!(distance < 0.01, "{:?} is {} units away from where it should be (actual point is {:?})", Coord2(x, y as f64), distance, t_point);
                assert!(t >= 0.0, "t={} (negative numbers should not be in the results", t);
                assert!(t <= 1.0, "t={} (>1 should not be in the results", t);

                let closest_point   = wave.nearest_point(&Coord2(x, y as f64));
                let distance        = closest_point.distance_to(&Coord2(x, y as f64));
                assert!(distance < 0.01, "{:?} is {} units away from where it should be", Coord2(x, y as f64), distance);

                let x = x as i64; 
                square[(x + y*80) as usize] = 'o'; }
        }
    }

    for y in 0..80 {
        for x in 0..80 {
            print!("{}", square[(x + y*80) as usize]);
        }
        println!();
    }
}
