use super::curve::*;
use super::normal::*;
use super::deform::*;
use super::characteristics::*;
use super::super::geo::*;
use super::super::line::*;

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

            if t2 > t1 {
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
        .map(|section| {
            // Compute the offsets for this section (TODO: use the curve length, not the t values)
            let (t1, t2)            = section.original_curve_t_values();
            let (offset1, offset2)  = (t1*offset_distance+initial_offset, t2*offset_distance+initial_offset);

            simple_offset(&section, offset1, offset2)
        })
        .collect()
}

///
/// Offsets the endpoints and mid-point of a curve by the specified amounts without subdividing
/// 
/// This won't produce an accurate offset if the curve doubles back on itself. The return value is the curve and the error
/// 
fn simple_offset<P: Coordinate, CurveIn: NormalCurve+BezierCurve<Point=P>, CurveOut: BezierCurveFactory<Point=P>>(curve: &CurveIn, initial_offset: f64, final_offset: f64) -> CurveOut
where P: Coordinate2D+Normalize {
    // Fetch the original points
    let start           = curve.start_point();
    let end             = curve.end_point();
    let (cp1, cp2)      = curve.control_points();

    // The normals at the start and end of the curve define the direction we should move in
    let normal_start    = curve.normal_at_pos(0.0);
    let normal_end      = curve.normal_at_pos(1.0);
    let normal_start    = normal_start.to_unit_vector();
    let normal_end      = normal_end.to_unit_vector();

    // If we can we want to scale the control points around the intersection of the normals
    let intersect_point = ray_intersects_ray(&(start, start+normal_start), &(end, end+normal_end));

    let offset_curve = if let Some(intersect_point) = intersect_point {
        // The control points point at an intersection point. We want to scale around this point so that start and end wind up at the appropriate offsets
        let new_start   = start + (normal_start * initial_offset);
        let new_end     = end + (normal_end * final_offset);

        let start_scale = (intersect_point.distance_to(&new_start))/(intersect_point.distance_to(&start));
        let end_scale   = (intersect_point.distance_to(&new_end))/(intersect_point.distance_to(&end));

        let new_cp1     = ((cp1-intersect_point) * start_scale) + intersect_point;
        let new_cp2     = ((cp2-intersect_point) * end_scale) + intersect_point;

        return CurveOut::from_points(new_start, (new_cp1, new_cp2), new_end);
    } else {
        // No intersection point: just move everything along the normal

        // Offset start & end by the specified amounts to create the first approximation of a curve
        let new_start   = start + (normal_start * initial_offset);
        let new_cp1     = cp1 + (normal_start * initial_offset);
        let new_cp2     = cp2 + (normal_end * final_offset);
        let new_end     = end + (normal_end * final_offset);

        CurveOut::from_points(new_start, (new_cp1, new_cp2), new_end)
    };

    // Adjust the center point of the curve
    let mid_offset  = (initial_offset + final_offset) * 0.5;
    let mid_normal  = curve.normal_at_pos(0.5).to_unit_vector();
    let cur_pos     = offset_curve.point_at_pos(0.5);
    let target_pos  = curve.point_at_pos(0.5) + (mid_normal * mid_offset);

    move_point(&offset_curve, 0.5, &(target_pos-cur_pos))
}
