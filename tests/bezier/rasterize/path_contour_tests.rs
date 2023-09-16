use flo_curves::arc::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;
use flo_curves::bezier::rasterize::*;
use flo_curves::bezier::vectorize::*;

use itertools::*;

fn draw(contour: &Vec<Vec<bool>>, size: ContourSize) {
    let bitmap = (0..(size.0 * size.1)).into_iter()
        .map(|pos| (pos % size.1, pos / size.1))
        .map(|(x, y)| contour[y][x])
        .collect::<Vec<_>>();

    for p in 0..bitmap.len() {
        print!("{}", if bitmap[p] { '#' } else { '.' });
        if ((p+1)%size.1) == 0 { println!() };
    }
    println!();
}

fn check_columns_vs_rows(contour: &impl ColumnSampledContour) {
    // Create a vec of the rows in the contour
    let pixels_from_rows = (0..contour.contour_size().height())
        .map(|y| contour.rounded_intercepts_on_line(y as _))
        .map(|intercepts| {
            let mut line = vec![false; contour.contour_size().width()];

            for range in intercepts {
                for pixel in range {
                    line[pixel] = true;
                }
            }

            line
        })
        .collect::<Vec<_>>();

    // ... and also the columns
    let pixels_from_columns = (0..contour.contour_size().width())
        .map(|x| contour.rounded_intercepts_on_column(x as _))
        .map(|intercepts| {
            let mut column = vec![false; contour.contour_size().width()];

            for range in intercepts {
                for pixel in range {
                    column[pixel] = true;
                }
            }

            column
        })
        .collect::<Vec<_>>();

    draw(&pixels_from_rows, contour.contour_size());
    draw(&pixels_from_columns, contour.contour_size());

    // Test all the pixels
    for y in 0..(contour.contour_size().height()) {
        for x in 0..(contour.contour_size().width()) {
            assert!(pixels_from_rows[y][x] == pixels_from_columns[x][y], "Row/column mismatch at {}, {}", x, y);
        }
    }
}

#[test]
fn basic_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_contour  = PathContour::from_path(vec![circle_path], ContourSize(1000, 1000));

    let mut num_intercepts = 0;
    for y in 0..1000 {
        let intercepts = circle_contour.intercepts_on_line(y as _);

        num_intercepts += intercepts.len();

        for range in intercepts {
            let p1 = Coord2(range.start as _, y as _);
            let p2 = Coord2(range.end as _, y as _);

            let d1 = p1.distance_to(&center);
            let d2 = p2.distance_to(&center);

            assert!((d1-radius).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
            assert!((d2-radius).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
        }
    }

    assert!(num_intercepts >= 600 && num_intercepts <= 602, "num_intercepts = {:?}", num_intercepts);
}

#[test]
fn basic_circle_columns() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_contour  = PathContour::from_path(vec![circle_path], ContourSize(1000, 1000));

    check_columns_vs_rows(&circle_contour);
}

#[test]
fn trace_circle() {
    let radius          = 300.0;
    let center          = Coord2(500.0, 500.0);
    let circle_path     = Circle::new(center, radius).to_path::<SimpleBezierPath>();

    let circle_contour  = PathContour::from_path(vec![circle_path], ContourSize(1000, 1000));
    let traced_circle   = trace_paths_from_samples::<SimpleBezierPath>(&circle_contour, 2.0);

    debug_assert!(traced_circle.len() == 1);
    debug_assert!(traced_circle[0].to_curves::<Curve<_>>().len() < 40, "Result has {} curves", traced_circle[0].to_curves::<Curve<_>>().len());

    for curve in traced_circle[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);

            let distance    = point.distance_to(&Coord2(501.0, 501.0));

            debug_assert!((distance - radius) < 2.0, "Point at distance {:?}", distance);
        }
    }
}

#[test]
fn doughnut() {
    let radius_outer    = 300.0;
    let radius_inner    = 200.0;
    let center          = Coord2(500.0, 500.0);
    let outer_circle    = Circle::new(center, radius_outer).to_path::<SimpleBezierPath>();
    let inner_circle    = Circle::new(center, radius_inner).to_path::<SimpleBezierPath>();

    let circle_contour  = PathContour::from_path(vec![outer_circle, inner_circle], ContourSize(1000, 1000));

    let mut num_intercepts = 0;
    for y in 0..1000 {
        let intercepts = circle_contour.intercepts_on_line(y as _);

        num_intercepts += intercepts.len();

        for (idx, range) in intercepts.iter().enumerate() {
            let p1 = Coord2(range.start as _, y as _);
            let p2 = Coord2(range.end as _, y as _);

            let d1 = p1.distance_to(&center);
            let d2 = p2.distance_to(&center);

            if intercepts.len() == 1 {
                assert!((d1-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                assert!((d2-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
            } else {
                assert!(idx < 2);

                if idx == 0 {
                    assert!((d1-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                    assert!((d2-radius_inner).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                } else {
                    assert!((d1-radius_inner).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                    assert!((d2-radius_outer).abs() < 2.0, "y={} d1={} d2={} p1={:?} p2={:?}", y, d1, d2, p1, p2);
                }
            }
        }
    }

    // 600 intercepts on the outer circle, 400 on in the inner
    assert!(num_intercepts >= 997 && num_intercepts <= 1003, "num_intercepts = {:?}", num_intercepts);
}

#[test]
fn trace_chisel_contours_using_intercepts() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();

    let (chisel_field, offset)  = PathContour::center_path(vec![chisel.clone()], 4);
    let traced_chisel           = trace_contours_from_intercepts::<Coord2>(&chisel_field);

    debug_assert!(traced_chisel.len() == 1);

    let mut num_points  = 0;
    let mut max_error   = 0.0f64;
    let mut total_error = 0.0f64;
    let mut error_count = 0;
    for point in traced_chisel[0].iter().copied() {
        num_points += 1;

        let point = point + offset - Coord2(1.0, 1.0);

        let nearest_distance = chisel.to_curves::<Curve<_>>().into_iter()
            .map(|curve| curve.distance_to(&point))
            .reduce(|d1, d2| d1.min(d2))
            .unwrap()
            .abs();
        max_error   = max_error.max(nearest_distance);
        total_error += nearest_distance;

        if nearest_distance > 0.1 {
            error_count += 1;
        }
    }

    let avg_error = total_error / (num_points as f64);

    debug_assert!(max_error < 0.01, "Max error was {} (average {}, num >0.1 {}/{})", max_error, avg_error, error_count, num_points);
}

#[test]
fn chisel_no_very_close_points_samples() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();
    let (chisel_field, _) = PathContour::center_path(vec![chisel.clone()], 4);

    let chisel_edges  = trace_contours_from_samples(&chisel_field);
    let contour_size  = chisel_field.contour_size();
    let chisel_points = chisel_edges.into_iter()
        .map(|edges| edges.into_iter().map(|edge| edge.to_coords::<Coord2>(contour_size)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert!(chisel_points.len() > 0);

    for subpath in chisel_points {
        for (p1, p2) in subpath.iter().tuple_windows() {
            let distance = p1.distance_to(p2);

            assert!(distance > 0.1, "{:?} {:?} are very close", p1, p2);
            assert!(distance < 2.0, "{:?} {:?} are very far apart", p1, p2);
        }
    }
}

#[test]
fn chisel_no_very_close_points_intercepts() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();
    let (chisel_field, _) = PathContour::center_path(vec![chisel.clone()], 4);

    let chisel_points = trace_contours_from_intercepts::<Coord2>(&chisel_field);
    assert!(chisel_points.len() > 0);

    for subpath in chisel_points {
        for (p1, p2) in subpath.iter().tuple_windows() {
            let distance = p1.distance_to(p2);

            assert!(distance > 0.1, "{:?} {:?} are very close", p1, p2);
            assert!(distance < 2.0, "{:?} {:?} are very far apart", p1, p2);
        }
    }
}

#[test]
fn trace_chisel_paths_using_intercepts() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();

    let (chisel_field, offset)  = PathContour::center_path(vec![chisel.clone()], 4);
    let traced_chisel           = trace_paths_from_intercepts::<SimpleBezierPath>(&chisel_field, 0.05);

    debug_assert!(traced_chisel.len() == 1);

    let mut num_points  = 0;
    let mut max_error   = 0.0f64;
    let mut total_error = 0.0f64;
    let mut error_count = 0;
    for curve in traced_chisel[0].to_curves::<Curve<_>>() {
        for t in 0..100 {
            num_points += 1;

            let t           = (t as f64) / 100.0;
            let point       = curve.point_at_pos(t);
            let point       = point + offset - Coord2(1.0, 1.0);

            // Ignore points that are close to the corners (these will be inherently out of range as we'll see the corner as curved, in particular because the angles between edges is very high)
            let in_corner = 
                point.distance_to(&Coord2(0.0, 0.0)) < 1.5
                || point.distance_to(&Coord2(12.0, 36.0)) < 1.5
                || point.distance_to(&Coord2(36.0, 48.0)) < 1.5
                || point.distance_to(&Coord2(24.0, 12.0)) < 1.5;

            let nearest_distance = chisel.to_curves::<Curve<_>>().into_iter()
                .map(|curve| curve.distance_to(&point))
                .reduce(|d1, d2| d1.min(d2))
                .unwrap()
                .abs();

            if !in_corner {
                max_error   = max_error.max(nearest_distance);
                total_error += nearest_distance;

                if nearest_distance > 0.1 {
                    error_count += 1;
                }
            } else {
                debug_assert!(nearest_distance < 1.0, "Corner point further than 1px out of range");
            }
        }
    }

    let avg_error = total_error / (num_points as f64);

    // TODO: this subdivides a lot, even though the straight edges should provide good fits, so there's probably some improvements that can be made to fit_curves 
    //  - maybe not actually picking the correct point when we subdivide (supposed to be the point of biggest error, which intuitively seems if should be the corner)
    //  - maybe curves can be joined after fitting
    debug_assert!(max_error < 0.4, "Max error was {} in {} curves (average {}, num >0.1 {}/{})", max_error, traced_chisel[0].to_curves::<Curve<_>>().len(), avg_error, error_count, num_points);
    debug_assert!(traced_chisel[0].to_curves::<Curve<_>>().len() < 20, "Result has {} curves", traced_chisel[0].to_curves::<Curve<_>>().len());
}

#[test]
pub fn intercepts_1() {
    // Create a path to try intercepts against (these coordinates are from -1 to 1)
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(-0.2037662266584257, -0.34995458248326766))
        .curve_to((Coord2(-0.20006384207737574, -0.35319049320697476), Coord2(-0.19778599976302993, -0.3576703897326853)), Coord2(-0.19431519023924634, -0.36111487969372347))
        .curve_to((Coord2(-0.19431503462317426, -0.36111503420780017), Coord2(-0.1943148790071022, -0.3611151887218768)), Coord2(-0.19431472334434063, -0.3611153432823124))
        .curve_to((Coord2(-0.18294289255315344, -0.3724123516207334), Coord2(-0.16958474910311086, -0.3835618084981183)), Coord2(-0.15358122129524585, -0.38745096337884))
        .curve_to((Coord2(-0.15358119165293727, -0.3874509705835756), Coord2(-0.15358116201062869, -0.38745097778831117)), Coord2(-0.15358113235942653, -0.3874509849952084))
        .curve_to((Coord2(-0.14395285570388705, -0.3897915520488139), Coord2(-0.13347754501911002, -0.38814846670720726)), Coord2(-0.12368297576904297, -0.38814846670720726))
        .curve_to((Coord2(-0.12368297576904297, -0.38814846670720726), Coord2(-0.12368297576904297, -0.38814846670720726)), Coord2(-0.12368297576904297, -0.38814846670720726))
        .curve_to((Coord2(-0.11573407727766122, -0.38814846670720726), Coord2(-0.10778901214865767, -0.387407625817904)),  Coord2(-0.10026765180850696, -0.3845881040578401))
        .curve_to((Coord2(-0.10026771361469485, -0.3845881272323997), Coord2(-0.10026777542088273, -0.38458815040695926)), Coord2(-0.10026783724561433, -0.3845881735884719))
        .curve_to((Coord2(-0.09822388934861571, -0.3838216071313091), Coord2(-0.09396942460443825, -0.3849772284862197)), Coord2(-0.09396942460443825, -0.38279426097869873))
        .curve_to((Coord2(-0.09310145582857077, -0.38279426097869873), Coord2(-0.09223348705270328, -0.38279426097869873)), Coord2(-0.09136525786016136, -0.38279426097869873))
        .curve_to((Coord2(-0.09136525786016136, -0.3851364602855769), Coord2(-0.09784821500867084, -0.38646200322312213)), Coord2(-0.0993533633159091, -0.3870264971650071))
        .curve_to((Coord2(-0.099353425122097, -0.3870265203395667), Coord2(-0.09935348692828488, -0.38702654351412624)), Coord2(-0.09935354875301648, -0.3870265666956389))
        .curve_to((Coord2(-0.10712695885690601, -0.3899405739957303), Coord2(-0.1154278620549664, -0.39075263345148414)), Coord2(-0.12368297576904297, -0.39075263345148414))
        .curve_to((Coord2(-0.12368297576904297, -0.39075263345148414), Coord2(-0.12368297576904297, -0.39075263345148414)), Coord2(-0.12368297576904297, -0.39075263345148414))
        .curve_to((Coord2(-0.13373978674827755, -0.39075263345148414), Coord2(-0.14429743492929809, -0.39238779522746203)), Coord2(-0.15419627242572972, -0.38998145671621737))
        .curve_to((Coord2(-0.15419624278342114, -0.38998146392095295), Coord2(-0.15419621314111256, -0.38998147112568854)), Coord2(-0.1541961834899104, -0.3899814783325858))
        .curve_to((Coord2(-0.17063635371210378, -0.38598621122478494), Coord2(-0.1844484936444886, -0.37458740886243014)), Coord2(-0.19615005937050312, -0.36296283512344935))
        .curve_to((Coord2(-0.19614990375443103, -0.36296298963752605), Coord2(-0.19614974813835898, -0.3629631441516027)), Coord2(-0.1961495924755974, -0.36296329871203825))
        .curve_to((Coord2(-0.19957834020225215, -0.3595605515885026), Coord2(-0.20183200830256534, -0.35510373526121125)), Coord2(-0.20547997962514364, -0.3519153821163417))
        .curve_to((Coord2(-0.20490878576133656, -0.35126184759863815), Coord2(-0.20433759189752945, -0.35060831308093454)), Coord2(-0.2037662266584257, -0.34995458248326766))
        .build();

    // Change to a path with points from 0-200
    let path = path.map_points::<SimpleBezierPath>(|p| {
        (p + Coord2(1.0, 1.0)) * 100.0
    });

    // Center the contour
    let (contour, offset) = PathContour::center_path(vec![path.clone()], 2);

    // Read intercepts exactly where each point is (which is most likely to generate errors)
    for y_pos in path.to_curves::<Curve<_>>().into_iter().map(|curve| curve.start_point().y()) {
        // Forward
        println!("y={} ({})", y_pos, (y_pos/100.0)-1.0);
        let intercepts  = contour.intercepts_on_line(y_pos - offset.y());

        //assert!(intercepts.len()%2 == 0, "Uneven number of intercepts at ypos {:?} {:?} ({:?})", y_pos, (y_pos/100.0)-1.0, intercepts);
    }
}

/*
#[test]
fn chisel_columns() {
    let chisel = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(0.0, 0.0))
        .line_to(Coord2(12.0, 36.0))
        .line_to(Coord2(36.0, 48.0))
        .line_to(Coord2(24.0, 12.0))
        .line_to(Coord2(0.0, 0.0))
        .build();

    let chisel_contour = PathContour::from_path(vec![chisel], ContourSize(50, 50));

    check_columns_vs_rows(&chisel_contour);
}
*/
