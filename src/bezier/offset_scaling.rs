use super::characteristics::*;
use super::curve::*;
use super::normal::*;
use crate::bezier::CurveSection;
use crate::geo::*;
use crate::line::*;

use itertools::*;
use smallvec::*;

// This is loosely based on the algorithm described at: https://pomax.github.io/bezierinfo/#offsetting,
// with numerous changes to allow for variable-width offsets and consistent behaviour (in particular,
// a much more reliable method of subdividing the curve)
//
// This algorithm works by subdividing the original curve into arches. We use the characteristics of the
// curve to do this: by subdividing a curve at its inflection point, we turn it into a series of arches.
// Arches have a focal point that the normal vectors along the curve roughly converge to, so we can
// scale around this point to generate an approximate offset curve (every point of the curve will move
// away from the focal point along its normal axis).
//
// As the focal point is approximate, using the start and end points to compute its location ensures that
// the offset is exact at the start and end of the curve.
//
// Edge cases: curves with inflection points at the start or end, arches where the normal vectors at the
// start and end are in parallel.
//
// Not all arches have normal vectors that converge (close to) a focal point. We can spot these quickly
// because the focal point of any two points is in general not equidistant from those two points: this
// also results in uneven scaling of the start and end points.
//
// TODO: we currently assume that 't' maps to 'length' which is untrue, so this can produce 'lumpy' curves
// when varying the width.
//
// It might be possible to use the canonical curve to better identify how to subdivide curves for the
// best results.

///
/// Computes a series of curves that approximate an offset curve from the specified origin curve.
///
/// This uses a scaling algorithm to compute the offset curve, which is fast but which can produce
/// errors, especially if the initial and final offsets are very different from one another.
///
pub fn offset_scaling<Curve>(curve: &Curve, initial_offset: f64, final_offset: f64) -> Vec<Curve>
where
    Curve: BezierCurveFactory + NormalCurve,
    Curve::Point: Normalize + Coordinate2D,
{
    // Split at the location of any features the curve might have
    let sections: SmallVec<[_; 4]> = match features_for_curve(curve, 0.01) {
        CurveFeatures::DoubleInflectionPoint(t1, t2) => {
            let t1 = if t1 > 0.9999 {
                1.0
            } else if t1 < 0.0001 {
                0.0
            } else {
                t1
            };
            let t2 = if t2 > 0.9999 {
                1.0
            } else if t2 < 0.0001 {
                0.0
            } else {
                t2
            };

            if t2 > t1 {
                smallvec![(0.0, t1), (t1, t2), (t2, 1.0)]
            } else {
                smallvec![(0.0, t2), (t2, t1), (t1, 1.0)]
            }
        }

        CurveFeatures::Loop(t1, t3) => {
            let t1 = if t1 > 0.9999 {
                1.0
            } else if t1 < 0.0001 {
                0.0
            } else {
                t1
            };
            let t3 = if t3 > 0.9999 {
                1.0
            } else if t3 < 0.0001 {
                0.0
            } else {
                t3
            };
            let t2 = (t1 + t3) / 2.0;

            if t3 > t1 {
                smallvec![(0.0, t1), (t1, t2), (t2, t3), (t3, 1.0)]
            } else {
                smallvec![(0.0, t3), (t3, t2), (t2, t1), (t1, 1.0)]
            }
        }

        CurveFeatures::SingleInflectionPoint(t) => {
            if t > 0.0001 && t < 0.9999 {
                smallvec![(0.0, t), (t, 1.0)]
            } else {
                smallvec![(0.0, 1.0)]
            }
        }

        _ => {
            smallvec![(0.0, 1.0)]
        }
    };
    let sections = sections
        .into_iter()
        .filter(|(t1, t2)| t1 != t2)
        .map(|(t1, t2)| curve.section(t1, t2))
        .collect::<SmallVec<[_; 8]>>();

    // Offset the set of curves that we retrieved
    let offset_distance = final_offset - initial_offset;

    sections
        .into_iter()
        .flat_map(|section| {
            // Compute the offsets for this section (TODO: use the curve length, not the t values)
            let (t1, t2) = section.original_curve_t_values();
            let (offset1, offset2) = (
                t1 * offset_distance + initial_offset,
                t2 * offset_distance + initial_offset,
            );

            subdivide_offset(&section, offset1, offset2, 0)
        })
        .collect()
}

///
/// Attempts a simple offset of a curve, and subdivides it if the midpoint is too far away from the expected distance
///
fn subdivide_offset<'a, CurveIn, CurveOut>(
    curve: &CurveSection<'a, CurveIn>,
    initial_offset: f64,
    final_offset: f64,
    depth: usize,
) -> SmallVec<[CurveOut; 2]>
where
    CurveIn: NormalCurve + BezierCurve,
    CurveOut: BezierCurveFactory<Point = CurveIn::Point>,
    CurveIn::Point: Coordinate2D + Normalize,
{
    const MAX_DEPTH: usize = 5;

    // Fetch the original points
    let start = curve.start_point();
    let end = curve.end_point();

    // The normals at the start and end of the curve define the direction we should move in
    let normal_start = curve.normal_at_pos(0.0);
    let normal_end = curve.normal_at_pos(1.0);
    let normal_start = normal_start.to_unit_vector();
    let normal_end = normal_end.to_unit_vector();

    // If we can we want to scale the control points around the intersection of the normals
    let intersect_point =
        ray_intersects_ray(&(start, start + normal_start), &(end, end + normal_end));

    if intersect_point.is_none()
        && characterize_curve(curve) != CurveCategory::Linear
        && depth < MAX_DEPTH
    {
        // Collinear normals
        let divide_point = 0.5;

        let mid_offset = initial_offset + (final_offset - initial_offset) * divide_point;
        let left_curve = curve.subsection(0.0, divide_point);
        let right_curve = curve.subsection(divide_point, 1.0);

        let left_offset = subdivide_offset(&left_curve, initial_offset, mid_offset, depth + 1);
        let right_offset = subdivide_offset(&right_curve, mid_offset, final_offset, depth + 1);

        return left_offset.into_iter().chain(right_offset).collect();
    }

    if let Some(intersect_point) = intersect_point {
        // Subdivide again if the intersection point is too close to one or other of the normals
        let start_distance = intersect_point.distance_to(&start);
        let end_distance = intersect_point.distance_to(&end);
        let distance_ratio = start_distance.min(end_distance) / start_distance.max(end_distance);

        // TODO: the closer to 1 this value is, the better the quality of the offset (0.99 produces good results)
        // but the number of subdivisions tends to be too high: we need to find either a way to generate a better offset
        // curve for an arch with a non-centered intersection point, or a better way to pick the subdivision point
        if distance_ratio < 0.995 && depth < MAX_DEPTH {
            // Try to subdivide at the curve's extremeties
            let mut extremeties = curve.find_extremities();
            extremeties.retain(|item| item > &0.01 && item < &0.99);

            if extremeties.len() == 0 || true {
                // No extremeties (or they're all too close to the edges)
                let divide_point = 0.5;

                let mid_offset = initial_offset + (final_offset - initial_offset) * divide_point;
                let left_curve = curve.subsection(0.0, divide_point);
                let right_curve = curve.subsection(divide_point, 1.0);

                let left_offset =
                    subdivide_offset(&left_curve, initial_offset, mid_offset, depth + 1);
                let right_offset =
                    subdivide_offset(&right_curve, mid_offset, final_offset, depth + 1);

                left_offset.into_iter().chain(right_offset).collect()
            } else {
                let mut extremeties = extremeties;
                extremeties.insert(0, 0.0);
                extremeties.push(1.0);

                extremeties
                    .into_iter()
                    .tuple_windows()
                    .flat_map(|(t1, t2)| {
                        let subsection = curve.subsection(t1, t2);
                        let offset1 = initial_offset + (final_offset - initial_offset) * t1;
                        let offset2 = initial_offset + (final_offset - initial_offset) * t2;
                        let res = subdivide_offset(&subsection, offset1, offset2, depth + 1);
                        res
                    })
                    .collect()
            }
        } else {
            // Event intersection point
            smallvec![offset_by_scaling(
                curve,
                initial_offset,
                final_offset,
                intersect_point,
                normal_start,
                normal_end
            )]
        }
    } else {
        // No intersection point
        smallvec![offset_by_moving(
            curve,
            initial_offset,
            final_offset,
            normal_start,
            normal_end
        )]
    }
}

///
/// Offsets a curve by scaling around a central point
///
#[inline]
fn offset_by_scaling<CurveIn, CurveOut>(
    curve: &CurveIn,
    initial_offset: f64,
    final_offset: f64,
    intersect_point: CurveIn::Point,
    unit_normal_start: CurveIn::Point,
    unit_normal_end: CurveIn::Point,
) -> CurveOut
where
    CurveIn: NormalCurve + BezierCurve,
    CurveOut: BezierCurveFactory<Point = CurveIn::Point>,
    CurveIn::Point: Coordinate2D + Normalize,
{
    let start = curve.start_point();
    let end = curve.end_point();
    let (cp1, cp2) = curve.control_points();

    // The control points point at an intersection point. We want to scale around this point so that start and end wind up at the appropriate offsets
    let new_start = start + (unit_normal_start * initial_offset);
    let new_end = end + (unit_normal_end * final_offset);

    let start_scale =
        (intersect_point.distance_to(&new_start)) / (intersect_point.distance_to(&start));
    let end_scale = (intersect_point.distance_to(&new_end)) / (intersect_point.distance_to(&end));

    // When the scale is changing, the control points are effectively 1/3rd and 2/3rds of the way along the curve
    let cp1_scale = (end_scale - start_scale) * (1.0 / 3.0) + start_scale;
    let cp2_scale = (end_scale - start_scale) * (2.0 / 3.0) + start_scale;

    let new_cp1 = ((cp1 - intersect_point) * cp1_scale) + intersect_point;
    let new_cp2 = ((cp2 - intersect_point) * cp2_scale) + intersect_point;

    CurveOut::from_points(new_start, (new_cp1, new_cp2), new_end)
}

///
/// Given a curve where the start and end normals do not intersect at a point, calculates the offset (by moving the start and end points along the normal)
///
#[inline]
fn offset_by_moving<CurveIn, CurveOut>(
    curve: &CurveIn,
    initial_offset: f64,
    final_offset: f64,
    unit_normal_start: CurveIn::Point,
    unit_normal_end: CurveIn::Point,
) -> CurveOut
where
    CurveIn: NormalCurve + BezierCurve,
    CurveOut: BezierCurveFactory<Point = CurveIn::Point>,
    CurveIn::Point: Coordinate2D + Normalize,
{
    let start = curve.start_point();
    let end = curve.end_point();
    let (cp1, cp2) = curve.control_points();

    // Offset start & end by the specified amounts to create the first approximation of a curve
    let new_start = start + (unit_normal_start * initial_offset);
    let new_cp1 = cp1 + (unit_normal_start * initial_offset);
    let new_cp2 = cp2 + (unit_normal_end * final_offset);
    let new_end = end + (unit_normal_end * final_offset);

    CurveOut::from_points(new_start, (new_cp1, new_cp2), new_end)
}
