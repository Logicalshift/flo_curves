use flo_curves::*;
use flo_curves::line;
use flo_curves::bezier::*;
use flo_curves::bezier::NormalCurve;

use std::f64;

///
/// Computes the distance from an offset curve to a source curve and compares it to the expected distance, returning
/// the highest error
///
/// (When the offsets are different, there are a few choices for distance: we use the 't' value but it would be more
/// correct to use curve length)
///
fn max_error<Curve: BezierCurve>(src_curve: &Curve, offset_curve: &Vec<Curve>, initial_offset: f64, final_offset: f64) -> f64
where Curve::Point: Coordinate2D+Normalize,
Curve: BezierCurve+NormalCurve {
    let mut error                                       = 0.0f64;
    let mut last_closest: Option<(f64, Curve::Point)>   = None;

    for offset in offset_curve.iter() {
        for t in 0..=100 {
            let t = (t as f64)/100.0;

            let pos                 = offset.point_at_pos(t);
            let normal              = offset.normal_at_pos(t);
            let intersect           = curve_intersects_ray(src_curve, &(pos, pos+normal));

            let mut min_error    = f64::MAX;

            if let Some((last_expected_offset, last_point)) = last_closest {
                let distance    = last_point.distance_to(&pos);
                min_error       = min_error.min((distance-last_expected_offset).abs());
            }

            for (curve_t, _, intersect_point) in intersect {
                let expected_offset     = (final_offset-initial_offset) * curve_t + initial_offset;

                let distance            = intersect_point.distance_to(&pos);
                let error               = (distance-expected_offset).abs();
                if error < min_error {
                    min_error           = error;
                    last_closest        = Some((expected_offset, intersect_point));
                }
            }

            if min_error < f64::MAX {
                if min_error > error {
                    println!("{} {}", t, min_error);
                }
                error = error.max(min_error);
            }
        }
    }

    println!("Max error: {}", error);

    error
}

#[test]
fn offset_overlap_start_point() {
    let c           = Curve::from_points(Coord2(412.0, 500.0), (Coord2(412.0, 500.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let offset      = offset(&c, 10.0, 10.0);
    let error       = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 3.5);
}

#[test]
fn offset_overlap_end_point() {
    let c           = Curve::from_points(Coord2(412.0, 500.0), (Coord2(163.0, 589.0), Coord2(308.0, 665.0)), Coord2(308.0, 665.0));
    let offset      = offset(&c, 10.0, 10.0);
    let error       = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 10.0);
}

#[test]
fn simple_offset_1() {
    let c           = Curve::from_points(Coord2(412.0, 500.0), (Coord2(163.0, 589.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let offset      = offset(&c, 10.0, 10.0);
    let error       = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 2.0);
}

#[test]
fn simple_offset_2() {
    let c           = Curve::from_points(Coord2(110.0, 110.0), (Coord2(110.0,300.0), Coord2(500.0,300.0)),  Coord2(500.0,110.0));
    let offset      = offset(&c, 10.0, 10.0);
    let error       = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 2.0);
}

#[test]
fn simple_offset_3() {
    // This curve doesn't produce a very satisfying result, so it's interesting it has a low error value
    let c           = Curve::from_points(Coord2(516.170654296875, 893.27001953125), (Coord2(445.1522921545783, 856.2028149461783), Coord2(447.7831664737134, 878.3276285260063)), Coord2(450.51018453430754, 901.260980294519));
    let offset      = offset(&c, 10.0, 10.0);
    let error       = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 2.0);
}

#[test]
fn simple_offset_4() {
    // This curve seems to produce a huge spike
    let c           = Curve::from_points(Coord2(987.7637, 993.9645), (Coord2(991.1699, 994.0231), Coord2(1043.5605, 853.44885)), Coord2(1064.9473, 994.277));
    let offset      = offset(&c, 10.0, 10.0);
    let error       = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 10.0);
}

#[test]
fn simple_offset_5() {
    let c           = Curve::from_points(Coord2(170.83203, 534.28906), (Coord2(140.99219, 492.1289), Coord2(0.52734375, 478.67188)), Coord2(262.95313, 533.2656));
    let offset_1    = offset(&c, 10.0, 10.0);
    let offset_2    = offset(&c, -10.0, -10.0);
    let error_1     = max_error(&c, &offset_1, 10.0, 10.0);
    let error_2     = max_error(&c, &offset_2, 10.0, 10.0);

    assert!(error_1 <= 10.0);
    assert!(error_2 <= 10.0);
}

#[test]
fn simple_offset_6() {
    let c           = Curve::from_points(Coord2(170.83203, 534.28906), (Coord2(35.15625, 502.65625), Coord2(0.52734375, 478.67188)), Coord2(262.95313, 533.2656));
    let offset_1    = offset(&c, 10.0, 10.0);
    let offset_2    = offset(&c, -10.0, -10.0);
    let error_1     = max_error(&c, &offset_1, 10.0, 10.0);
    let error_2     = max_error(&c, &offset_2, 10.0, 10.0);

    assert!(error_1 <= 10.0);
    assert!(error_2 <= 10.0);
}

#[test]
fn resizing_offset_1() {
    let c           = Curve::from_points(Coord2(412.0, 500.0), (Coord2(163.0, 589.0), Coord2(163.0, 504.0)), Coord2(308.0, 665.0));
    let offset      = offset(&c, 10.0, 40.0);
    let error       = max_error(&c, &offset, 10.0, 40.0);

    assert!(error <= 2.0);
}

#[test]
fn resizing_offset_2() {
    let c           = Curve::from_points(Coord2(110.0, 110.0), (Coord2(110.0,300.0), Coord2(500.0,300.0)),  Coord2(500.0,110.0));
    let offset      = offset(&c, 10.0, 40.0);
    let error       = max_error(&c, &offset, 10.0, 40.0);

    assert!(error <= 6.0);
}

#[test]
fn resize_offset_3() {
    let c           = Curve::from_points(Coord2(516.170654296875, 893.27001953125), (Coord2(445.1522921545783, 856.2028149461783), Coord2(447.7831664737134, 878.3276285260063)), Coord2(450.51018453430754, 901.260980294519));
    let offset      = offset(&c, 10.0, 40.0);
    let error       = max_error(&c, &offset, 10.0, 40.0);

    // The error seems to get so high because we're using the 't' value as a ratio for determining width rather than curve length
    // This also results in this offset curve not being particularly smooth
    assert!(error <= 15.0);
}

#[test]
fn normals_for_line_do_not_meet_at_intersection() {
    // Overlapping control points mean that this curve defines a line
    let c               = Curve::from_points(Coord2(163.0, 579.0), (Coord2(163.0, 579.0), Coord2(405.0, 684.0)), Coord2(405.0, 684.0));

    // Compute the normal at the start and the end of the line
    let start           = c.start_point();
    let end             = c.end_point();
    let start_normal    = c.normal_at_pos(0.0).to_unit_vector();
    let end_normal      = c.normal_at_pos(1.0).to_unit_vector();

    // The rays starting from the start and end of this line should not intersect
    let intersection    = line::ray_intersects_ray(&(start, start+start_normal), &(end, end+end_normal));
    assert!(intersection.is_none());
}
