use super::curve::*;
use super::normal::*;
use super::characteristics::*;
use super::super::geo::*;
use super::super::line::*;
use super::super::bezier::{CurveSection};

use smallvec::*;

///
/// Computes a series of curves that approximate an offset curve from the specified origin curve.
/// 
/// Based on the algorithm described in https://pomax.github.io/bezierinfo/#offsetting
///
pub fn offset<Curve: BezierCurveFactory+NormalCurve>(curve: &Curve, initial_offset: f64, final_offset: f64) -> Vec<Curve>
where Curve::Point: Normalize+Coordinate2D {
    // Split at the location of any features the curve might have
    let sections: SmallVec<[_; 4]>  = match features_for_curve(curve, 0.01) {
        CurveFeatures::DoubleInflectionPoint(t1, t2)  => {
            let t1 = if t1 > 0.9999 { 1.0 } else if t1 < 0.0001 { 0.0 } else { t1 };
            let t2 = if t2 > 0.9999 { 1.0 } else if t2 < 0.0001 { 0.0 } else { t2 };

            if t2 > t1 {
                smallvec![(0.0, t1), (t1, t2), (t2, 1.0)]
            } else {
                smallvec![(0.0, t2), (t2, t1), (t1, 1.0)]
            }
        }

        CurveFeatures::Loop(t1, t3) => {
            let t1 = if t1 > 0.9999 { 1.0 } else if t1 < 0.0001 { 0.0 } else { t1 };
            let t3 = if t3 > 0.9999 { 1.0 } else if t3 < 0.0001 { 0.0 } else { t3 };
            let t2 = (t1+t3)/2.0;

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

        _ => { smallvec![(0.0, 1.0)] }
    };
    let sections            = sections.into_iter()
        .filter(|(t1, t2)| t1 != t2)
        .map(|(t1, t2)| curve.section(t1, t2))
        .collect::<SmallVec<[_; 8]>>();

    // Offset the set of curves that we retrieved
    let offset_distance     = final_offset-initial_offset;

    sections.into_iter()
        .flat_map(|section| {
            // Compute the offsets for this section (TODO: use the curve length, not the t values)
            let (t1, t2)            = section.original_curve_t_values();
            let (offset1, offset2)  = (t1*offset_distance+initial_offset, t2*offset_distance+initial_offset);

            subdivide_offset(&section, offset1, offset2)
        })
        .collect()
}

///
/// Attempts a simple offset of a curve, and subdivides it if the midpoint is too far away from the expected distance
///
fn subdivide_offset<'a, P: Coordinate, CurveIn: NormalCurve+BezierCurve<Point=P>, CurveOut: BezierCurveFactory<Point=P>>(curve: &CurveSection<'a, CurveIn>, initial_offset: f64, final_offset: f64) -> SmallVec<[CurveOut; 2]>
where P: Coordinate2D+Normalize {
    // Fetch the original points
    let start           = curve.start_point();
    let end             = curve.end_point();

    // The normals at the start and end of the curve define the direction we should move in
    let normal_start    = curve.normal_at_pos(0.0);
    let normal_end      = curve.normal_at_pos(1.0);
    let normal_start    = normal_start.to_unit_vector();
    let normal_end      = normal_end.to_unit_vector();

    // If we can we want to scale the control points around the intersection of the normals
    let intersect_point = ray_intersects_ray(&(start, start+normal_start), &(end, end+normal_end));

    if let Some(intersect_point) = intersect_point {
        // Subdivide again if the intersection point is too close to one or other of the mpr,a;s
        let start_distance  = intersect_point.distance_to(&start);
        let end_distance    = intersect_point.distance_to(&end);
        let distance_ratio = start_distance.min(end_distance) / start_distance.max(end_distance);

        // TODO: the closer to 1 this value is, the better the quality of the offset (0.99 produces good results)
        // but the number of subdivisions tends to be too high: we need to find either a way to generate a better offset
        // curve for an arch with a non-centered intersection point, or a better way to pick the subdivision point
        if distance_ratio < 0.99 {
            let divide_point    = 0.5;

            let mid_offset      = initial_offset + (final_offset - initial_offset) * divide_point;
            let left_curve      = curve.subsection(0.0, divide_point);
            let right_curve     = curve.subsection(divide_point, 1.0);

            let left_offset     = subdivide_offset(&left_curve, initial_offset, mid_offset);
            let right_offset    = subdivide_offset(&right_curve, mid_offset, final_offset);

            left_offset.into_iter()
                .chain(right_offset)
                .collect()
        } else {
            // Event intersection point
            smallvec![offset_by_scaling(curve, initial_offset, final_offset, intersect_point, normal_start, normal_end)]
        }

    } else {
        // No intersection point
        smallvec![offset_by_moving(curve, initial_offset, final_offset, normal_start, normal_end)]
    }
}

///
/// Offsets a curve by scaling around a central point
///
#[inline]
fn offset_by_scaling<CurveIn, CurveOut>(curve: &CurveIn, initial_offset: f64, final_offset: f64, intersect_point: CurveIn::Point, unit_normal_start: CurveIn::Point, unit_normal_end: CurveIn::Point) -> CurveOut
where   CurveIn:        NormalCurve+BezierCurve,
        CurveOut:       BezierCurveFactory<Point=CurveIn::Point>,
        CurveIn::Point: Coordinate2D+Normalize {
    let start           = curve.start_point();
    let end             = curve.end_point();
    let (cp1, cp2)      = curve.control_points();

    // The control points point at an intersection point. We want to scale around this point so that start and end wind up at the appropriate offsets
    let new_start   = start + (unit_normal_start * initial_offset);
    let new_end     = end + (unit_normal_end * final_offset);

    let start_scale = (intersect_point.distance_to(&new_start))/(intersect_point.distance_to(&start));
    let end_scale   = (intersect_point.distance_to(&new_end))/(intersect_point.distance_to(&end));

    let new_cp1     = ((cp1-intersect_point) * start_scale) + intersect_point;
    let new_cp2     = ((cp2-intersect_point) * end_scale) + intersect_point;

    CurveOut::from_points(new_start, (new_cp1, new_cp2), new_end)
}

///
/// Given a curve where the start and end normals do not intersect at a point, calculates the offset (by moving the start and end points along the normal)
///
#[inline]
fn offset_by_moving<CurveIn, CurveOut>(curve: &CurveIn, initial_offset: f64, final_offset: f64, unit_normal_start: CurveIn::Point, unit_normal_end: CurveIn::Point) -> CurveOut
where   CurveIn:        NormalCurve+BezierCurve,
        CurveOut:       BezierCurveFactory<Point=CurveIn::Point>,
        CurveIn::Point: Coordinate2D+Normalize {
    let start           = curve.start_point();
    let end             = curve.end_point();
    let (cp1, cp2)      = curve.control_points();

    // Offset start & end by the specified amounts to create the first approximation of a curve
    let new_start   = start + (unit_normal_start * initial_offset);
    let new_cp1     = cp1 + (unit_normal_start * initial_offset);
    let new_cp2     = cp2 + (unit_normal_end * final_offset);
    let new_end     = end + (unit_normal_end * final_offset);

    CurveOut::from_points(new_start, (new_cp1, new_cp2), new_end)
}
