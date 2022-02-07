use super::super::super::geo::*;
use super::super::characteristics::*;
use super::super::curve::*;
use super::super::section::*;
use super::curve_curve_clip::*;

///
/// If a cubic curve contains a loop, finds the t values where the curve self-intersects
///
pub fn find_self_intersection_point<C: BezierCurve>(curve: &C, accuracy: f64) -> Option<(f64, f64)>
where
    C::Point: Coordinate + Coordinate2D,
{
    let curve_type = curve.characteristics();

    if curve_type == CurveCategory::Loop {
        let full_curve = CurveSection::new(curve, 0.0, 1.0);
        find_intersection_point_in_loop(full_curve, accuracy)
    } else {
        None
    }
}

///
/// Given a curve known to have a loop in it, subdivides it in order to determine where the intersection lies
///
fn find_intersection_point_in_loop<C: BezierCurve>(
    curve: CurveSection<C>,
    accuracy: f64,
) -> Option<(f64, f64)>
where
    C::Point: Coordinate + Coordinate2D,
{
    use self::CurveCategory::*;

    // The algorithm here is to divide the curve into two. We'll either find a smaller curve with a loop or split the curve in the middle of the loop
    // If we split in the middle of the loop, we use the bezier clipping algorithm to find where the two sides intersect
    let (left, right) = (curve.subsection(0.0, 0.5), curve.subsection(0.5, 1.0));
    let (left_type, right_type) = (left.characteristics(), right.characteristics());

    match (left_type, right_type) {
        (Loop, Loop) => {
            // If both sides are loops then we've split the original curve at the intersection point
            unimplemented!("Need support for a loop where we hit the intersection point")
        }

        (Loop, _) => {
            // Loop is in the left side
            find_intersection_point_in_loop(left, accuracy)
        }

        (_, Loop) => {
            // Loop is in the right side
            find_intersection_point_in_loop(right, accuracy)
        }

        (_, _) => {
            // Can find the intersection by using the clipping algorithm
            let intersections = curve_intersects_curve_clip(&left, &right, accuracy);

            if intersections.is_empty()
                && left.start_point().is_near_to(&right.end_point(), accuracy)
            {
                // Didn't find an intersection but the left and right curves start and end at the same position
                return Some((left.t_for_t(0.0), right.t_for_t(1.0)));
            }

            test_assert!(!intersections.is_empty());

            if intersections.len() == 1 {
                // Only found a single intersection
                intersections
                    .into_iter()
                    .next()
                    .map(|(t1, t2)| (left.t_for_t(t1), right.t_for_t(t2)))
            } else {
                // Intersection may include the point between the left and right curves (ignore any point that's at t=1 on the left or t=0 on the right)
                intersections
                    .into_iter()
                    .find(|(t1, t2)| *t1 < 1.0 && *t2 > 0.0)
                    .map(|(t1, t2)| (left.t_for_t(t1), right.t_for_t(t2)))
            }
        }
    }
}
