use crate::geo::*;
use crate::bezier::*;
use super::find_roots::*;

use itertools::*;
use smallvec::*;

use std::iter;

///
/// Creates a 5th degree bezier curve that describes the dot product of the curve's tangent and the line connecting to 
/// the point at every point on the curve. This is 0 when the point is perpendicular to the curve (ie, where the curve
/// is neither moving away from or towards the point)
///
/// The closest points must be either one that is perpendicular or the start or end point of the curve.
///
fn distance_in_bezier_form<C>(curve: &C, point: &C::Point) -> [Coord2; 6]
where
    C:          BezierCurve + BezierCurve2D,
    C::Point:   Coordinate2D,
{
    // Precomputed 'z' factor for cubic curves
    const Z: [[f64; 4]; 3] = [
        [1.0, 0.6, 0.3, 0.1],
        [0.4, 0.6, 0.6, 0.4],
        [0.1, 0.3, 0.6, 1.0],
    ];

    // Fetch the control points of the curve
    let start_point     = curve.start_point();
    let end_point       = curve.end_point();
    let (cp1, cp2)      = curve.control_points();
    let curve_points    = [start_point, cp1, cp2, end_point];

    // Convert to Coord2s
    let point           = Coord2(point.x(), point.y());
    let curve_points    = curve_points.iter().map(|p| Coord2(p.x(), p.y())).collect::<SmallVec<[_; 4]>>();

    // Get the vectors from each control point to the control points, and from each control point to the next
    let control_point_to_point  = curve_points.iter()
        .map(|control_point| *control_point - point)
        .collect::<SmallVec<[_; 4]>>();
    let control_point_to_next   = curve_points.iter().tuple_windows()
        .map(|(cp1, cp2)| (*cp2-*cp1) * 3.0)
        .collect::<SmallVec<[_; 3]>>();

    // Create a table of dot products of the points in each of the two lists we just made
    let cp_dot_products = control_point_to_next.into_iter()
        .map(|to_next_cp| {
            control_point_to_point.iter()
                .map(|to_point| to_next_cp.dot(to_point))
                .collect::<SmallVec<[_; 4]>>()
        }).collect::<SmallVec<[_; 3]>>();

    // Apply the 'z' factors to create the final curve
    let mut curve = [Coord2(0.0/5.0, 0.0), Coord2(1.0/5.0, 0.0), Coord2(2.0/5.0, 0.0), Coord2(3.0/5.0, 0.0), Coord2(4.0/5.0, 0.0), Coord2(5.0/5.0, 0.0)];

    for k in 0..=5i32 {
        let lower = 0.max(k-2);
        let upper = k.min(3);

        for i in lower..=upper {
            let j = k - i;

            curve[(i+j) as usize].1 += cp_dot_products[j as usize][i as usize] * Z[j as usize][i as usize];
        }
    }

    curve
}

///
/// Uses the root-finding algorithm described in Graphics Gems to find the nearest points on the
/// bezier curve.
///
pub fn nearest_point_on_curve_bezier_root_finder<C>(curve: &C, point: &C::Point) -> f64
where
    C:          BezierCurve + BezierCurve2D,
    C::Point:   Coordinate + Coordinate2D,
{
    // See "Solving the Nearest-Point-On-Curve Problem", Philip J Schneider, Graphics Gems

    // Create a curve of order 5 to find the points that are perpendicular to the curve
    let tangent_curve = distance_in_bezier_form(curve, point);

    // Solve to find the roots of this curve
    let perpendicular_t_values = find_bezier_roots(tangent_curve);

    // Need to find the closest roots, or the start or end points can be closer
    let mut min_t_value     = 0.0;
    let offset              = curve.point_at_pos(0.0) - *point;
    let mut min_distance_sq = offset.dot(&offset);

    for t in perpendicular_t_values.into_iter().chain(iter::once(1.0)) {
        let offset      = curve.point_at_pos(t) - *point;
        let distance_sq = offset.dot(&offset);

        if distance_sq <= min_distance_sq {
            min_t_value     = t;
            min_distance_sq = distance_sq;
        }
    }

    // Closest point on the curve should be min_t_value
    min_t_value
}
