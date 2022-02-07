use flo_curves::bezier::NormalCurve;
use flo_curves::bezier::{
    curve_intersects_ray, offset, offset_lms_sampling, BezierCurve, BezierCurveFactory,
    BoundingBox, Coord2, Coordinate, Coordinate2D, Coordinate3D, Curve, Normalize,
};
use flo_curves::line;
use flo_curves::line::Line2D;

use std::f64;

///
/// Computes the distance from an offset curve to a source curve and compares it to the expected distance, returning
/// the highest error
///
/// (When the offsets are different, there are a few choices for distance: we use the 't' value but it would be more
/// correct to use curve length)
///
fn max_error<Curve: BezierCurve>(
    src_curve: &Curve,
    offset_curve: &Vec<Curve>,
    initial_offset: f64,
    final_offset: f64,
) -> f64
where
    Curve::Point: Coordinate2D + Normalize,
    Curve: BezierCurve + NormalCurve,
{
    let mut error = 0.0f64;
    let mut last_closest: Option<(f64, Curve::Point)> = None;

    for offset in offset_curve.iter() {
        for t in 1..=99 {
            let t = (t as f64) / 100.0;

            let pos = offset.point_at_pos(t);
            let normal = offset.normal_at_pos(t);
            let intersect = curve_intersects_ray(src_curve, &(pos, pos + normal));

            let mut min_error = f64::MAX;

            if let Some((last_expected_offset, last_point)) = last_closest {
                let distance = last_point.distance_to(&pos);
                min_error = min_error.min((distance - last_expected_offset).abs());
            }

            for (curve_t, _, intersect_point) in intersect {
                let expected_offset = (final_offset - initial_offset) * curve_t + initial_offset;

                let distance = intersect_point.distance_to(&pos);
                let error = (distance - expected_offset).abs();
                if error < min_error {
                    min_error = error;
                    last_closest = Some((expected_offset, intersect_point));
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
    let c = Curve::from_points(
        Coord2(412.0, 500.0),
        (Coord2(412.0, 500.0), Coord2(163.0, 504.0)),
        Coord2(308.0, 665.0),
    );
    let offset = offset(&c, 10.0, 10.0);
    let error = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 3.5);
}

#[test]
fn offset_overlap_end_point() {
    let c = Curve::from_points(
        Coord2(412.0, 500.0),
        (Coord2(163.0, 589.0), Coord2(308.0, 665.0)),
        Coord2(308.0, 665.0),
    );
    let offset = offset(&c, 10.0, 10.0);
    let error = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 10.0);
}

#[test]
fn simple_offset_1() {
    let c = Curve::from_points(
        Coord2(412.0, 500.0),
        (Coord2(163.0, 589.0), Coord2(163.0, 504.0)),
        Coord2(308.0, 665.0),
    );
    let offset = offset(&c, 10.0, 10.0);
    let error = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 2.0);
}

#[test]
fn simple_offset_2() {
    let c = Curve::from_points(
        Coord2(110.0, 110.0),
        (Coord2(110.0, 300.0), Coord2(500.0, 300.0)),
        Coord2(500.0, 110.0),
    );
    let offset = offset(&c, 10.0, 10.0);
    let error = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 2.0);
}

#[test]
fn simple_offset_3() {
    // This curve doesn't produce a very satisfying result, so it's interesting it has a low error value
    let c = Curve::from_points(
        Coord2(516.170654296875, 893.27001953125),
        (
            Coord2(445.1522921545783, 856.2028149461783),
            Coord2(447.7831664737134, 878.3276285260063),
        ),
        Coord2(450.51018453430754, 901.260980294519),
    );
    let offset = offset(&c, 10.0, 10.0);
    let error = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 2.0);
}

#[test]
fn simple_offset_4() {
    // This curve seems to produce a huge spike
    let c = Curve::from_points(
        Coord2(987.7637, 993.9645),
        (Coord2(991.1699, 994.0231), Coord2(1043.5605, 853.44885)),
        Coord2(1064.9473, 994.277),
    );
    let offset = offset(&c, 10.0, 10.0);
    let error = max_error(&c, &offset, 10.0, 10.0);

    assert!(error <= 10.0);
}

#[test]
fn simple_offset_5() {
    // This curve has a point approaching a cusp, so it produces 'strange' values

    // We bulge out slightly around the cusp so there's a large error
    let c = Curve::from_points(
        Coord2(170.83203, 534.28906),
        (Coord2(140.99219, 492.1289), Coord2(0.52734375, 478.67188)),
        Coord2(262.95313, 533.2656),
    );
    let offset_1 = offset(&c, 10.0, 10.0);
    let error_1 = max_error(&c, &offset_1, 10.0, 10.0);
    assert!(error_1 <= 12.0);

    // Offsetting too much 'inside' the curve starts to produce chaotic behaviour around the cusp with this algorithm
    let offset_2 = offset(&c, -2.0, -2.0);
    let error_2 = max_error(&c, &offset_2, 2.0, 2.0);
    assert!(error_2 <= 4.0);
}

#[test]
fn simple_offset_6() {
    let c = Curve::from_points(
        Coord2(170.83203, 534.28906),
        (Coord2(35.15625, 502.65625), Coord2(0.52734375, 478.67188)),
        Coord2(262.95313, 533.2656),
    );

    // This is a very tight curve, so there's no good solution in this direction for large offsets (the scaling algorithm produces a very chaotic curve)
    let offset_1 = offset(&c, 2.0, 2.0);
    let error_1 = max_error(&c, &offset_1, 2.0, 2.0);
    assert!(error_1 <= 2.0);

    let offset_2 = offset(&c, -10.0, -10.0);
    let error_2 = max_error(&c, &offset_2, 10.0, 10.0);

    assert!(error_2 <= 1.0);
}

#[test]
fn resizing_offset_1() {
    let c = Curve::from_points(
        Coord2(412.0, 500.0),
        (Coord2(163.0, 589.0), Coord2(163.0, 504.0)),
        Coord2(308.0, 665.0),
    );
    let offset = offset(&c, 10.0, 40.0);
    let error = max_error(&c, &offset, 10.0, 40.0);

    assert!(error <= 2.0);
}

#[test]
fn resizing_offset_2() {
    let c = Curve::from_points(
        Coord2(110.0, 110.0),
        (Coord2(110.0, 300.0), Coord2(500.0, 300.0)),
        Coord2(500.0, 110.0),
    );
    let offset = offset(&c, 10.0, 40.0);
    let error = max_error(&c, &offset, 10.0, 40.0);

    assert!(error <= 6.0);
}

#[test]
fn resize_offset_3() {
    let c = Curve::from_points(
        Coord2(516.170654296875, 893.27001953125),
        (
            Coord2(445.1522921545783, 856.2028149461783),
            Coord2(447.7831664737134, 878.3276285260063),
        ),
        Coord2(450.51018453430754, 901.260980294519),
    );
    let offset = offset(&c, 10.0, 40.0);
    let error = max_error(&c, &offset, 10.0, 40.0);

    // The error seems to get so high because we're using the 't' value as a ratio for determining width rather than curve length
    // This also results in this offset curve not being particularly smooth
    assert!(error <= 15.0);
}

#[test]
fn move_offset_1() {
    let c = Curve::from_points(
        Coord2(163.0, 579.0),
        (Coord2(163.0, 579.0), Coord2(405.0, 684.0)),
        Coord2(405.0, 684.0),
    );
    let offset = offset(&c, 10.0, 10.0);
    let error = max_error(&c, &offset, 10.0, 10.0);

    assert!(offset.len() == 1);

    let w1 = offset[0].start_point();
    let (w2, w3) = offset[0].control_points();
    let w4 = offset[0].end_point();

    assert!((w2, w3).distance_to(&w1) < 0.01);
    assert!((w2, w3).distance_to(&w4) < 0.01);
    assert!(error <= 1.0);
}

#[test]
fn normals_for_line_do_not_meet_at_intersection() {
    // Overlapping control points mean that this curve defines a line
    let c = Curve::from_points(
        Coord2(163.0, 579.0),
        (Coord2(163.0, 579.0), Coord2(405.0, 684.0)),
        Coord2(405.0, 684.0),
    );

    // Compute the normal at the start and the end of the line
    let start = c.start_point();
    let end = c.end_point();
    let start_normal = c.normal_at_pos(0.0).to_unit_vector();
    let end_normal = c.normal_at_pos(1.0).to_unit_vector();

    // The rays starting from the start and end of this line should not intersect
    // (This generates a ray divisor of 0.00000000000002603472992745992, because we lose enough precision that the lines appear to be not quite parallel)
    let intersection =
        line::ray_intersects_ray(&(start, start + start_normal), &(end, end + end_normal));
    assert!(intersection.is_none());
}

#[test]
fn offset_lms_sampling_arc_start_tangent() {
    use flo_curves::arc::Circle;

    // 90 degree circle arc
    let circle = Circle::new(Coord2(0.0, 0.0), 100.0);
    let arc = circle.arc(0.0, f64::consts::PI / 2.0);
    let arc_curve = arc.to_bezier_curve::<Curve<Coord2>>();

    // Offset by 10
    let offset_arc =
        offset_lms_sampling(&arc_curve, |_t| 10.0, |_t| 0.0, 20, 0.01).expect("Offset curve");

    let start_tangent_original = arc_curve.tangent_at_pos(0.0).to_unit_vector();
    let start_tangent_new = offset_arc[0].tangent_at_pos(0.0).to_unit_vector();

    println!("{:?} {:?}", start_tangent_original, start_tangent_new);

    assert!(start_tangent_original.distance_to(&start_tangent_new) < 0.01);
}

#[test]
fn offset_lms_sampling_arc_end_tangent() {
    use flo_curves::arc::Circle;

    // 90 degree circle arc
    let circle = Circle::new(Coord2(0.0, 0.0), 100.0);
    let arc = circle.arc(0.0, f64::consts::PI / 2.0);
    let arc_curve = arc.to_bezier_curve::<Curve<Coord2>>();

    // Offset by 10
    let offset_arc =
        offset_lms_sampling(&arc_curve, |_t| 10.0, |_t| 0.0, 20, 0.01).expect("Offset curve");

    let end_tangent_original = arc_curve.tangent_at_pos(1.0).to_unit_vector();
    let end_tangent_new = offset_arc[offset_arc.len() - 1]
        .tangent_at_pos(1.0)
        .to_unit_vector();

    println!("{:?} {:?}", end_tangent_original, end_tangent_new);

    assert!(end_tangent_original.distance_to(&end_tangent_new) < 0.01);
}

#[test]
fn offset_lms_sampling_arc_end_point() {
    use flo_curves::arc::Circle;

    // 90 degree circle arc
    let circle = Circle::new(Coord2(0.0, 0.0), 100.0);
    let arc = circle.arc(0.0, f64::consts::PI / 2.0);
    let arc_curve = arc.to_bezier_curve::<Curve<Coord2>>();

    // Offset by 10
    let offset_arc =
        offset_lms_sampling(&arc_curve, |_t| 10.0, |_t| 0.0, 20, 0.01).expect("Offset curve");

    let end_point_original = arc_curve.point_at_pos(1.0);
    let end_point_new = offset_arc[offset_arc.len() - 1].point_at_pos(1.0);
    let end_point_expected = Coord2(end_point_original.x() + 10.0, end_point_original.y());

    println!("{:?} {:?}", end_point_original, end_point_new);

    assert!((end_point_original.distance_to(&end_point_new) - 10.0).abs() < 0.01);
    assert!(end_point_expected.distance_to(&end_point_new) < 0.01);
}

#[test]
fn offset_lms_sampling_arc_fit_single_curve() {
    use flo_curves::arc::Circle;

    // 90 degree circle arc
    let circle = Circle::new(Coord2(0.0, 0.0), 100.0);
    let arc = circle.arc(0.0, f64::consts::PI / 2.0);
    let arc_curve = arc.to_bezier_curve::<Curve<Coord2>>();

    // Offset by 10
    let offset_arc =
        offset_lms_sampling(&arc_curve, |_t| 10.0, |_t| 0.0, 20, 1.0).expect("Offset curve");

    // We should be able to find a single bezier curve that fits these points
    assert!(offset_arc.len() == 1);
}
