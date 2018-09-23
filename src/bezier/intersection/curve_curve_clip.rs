use super::fat_line::*;
use super::super::super::bezier::*;

///
/// Determines the length of a curve's hull as a sum of squares
/// 
fn curve_hull_length_sq<C: BezierCurve>(curve: &C) -> f64 {
    let start       = curve.start_point();
    let end         = curve.end_point();
    let (cp1, cp2)  = curve.control_points();

    let offset1 = cp1-start;
    let offset2 = cp2-cp1;
    let offset3 = cp2-end;

    offset1.dot(&offset1) + offset2.dot(&offset2) + offset3.dot(&offset3)
}

///
/// Performs the fat-line clipping algorithm on two curves, returning the t values if they overlap
/// 
#[inline]
fn clip<'a, C: BezierCurve>(curve_to_clip: &CurveSection<'a, C>, curve_to_clip_against: &CurveSection<'a, C>) -> Option<(f64, f64)>
where C::Point: 'a+Coordinate2D {
    // Clip against the fat line
    let fat_line    = FatLine::from_curve(curve_to_clip_against);
    let clip_t      = fat_line.clip_t(curve_to_clip);

    // t1 and t2 must not match (exact matches produce an invalid curve)
    clip_t.map(|(t1, t2)| if t1 == t2 { (t1-0.01, t2) } else { (t1, t2) })
}

///
/// Given a set of intersections found on a left and right curve, joins them in a way that eliminates duplicates
/// 
fn join_subsections<'a, C: BezierCurve>(curve1: &CurveSection<'a, C>, left: Vec<(f64, f64)>, right: Vec<(f64, f64)>, accuracy_squared: f64) -> Vec<(f64, f64)> 
where C::Point: Coordinate2D {
    if left.len() == 0 {
        // No further work to do
        right
    } else if right.len() == 0 {
        // No further work to do
        left
    } else {
        // The last intersection in left might be the same as the first in right
        let (left_t1, _left_t2)     = left[left.len()-1];
        let (right_t1, _right_t2)   = right[0];

        // We use t1 and curve1 to determine this
        let left_t1                 = curve1.section_t_for_original_t(left_t1);
        let right_t1                = curve1.section_t_for_original_t(right_t1);

        if (right_t1-left_t1).abs() < 0.1 {
            // Could be the same point
            let p1 = curve1.point_at_pos(left_t1);
            let p2 = curve1.point_at_pos(right_t1);

            let offset              = p2-p1;
            let distance_squared    = offset.dot(&offset);

            if distance_squared <= (accuracy_squared*2.0) {
                // First and last points are the same: only use the version of the LHS
                let mut combined = left;
                combined.extend(right.into_iter().skip(1));
                combined
            } else {
                // Not the same points: just combine the two curves
                let mut combined = left;
                combined.extend(right);
                combined
            }
        } else {
            // Not the same points: just combine the two curves
            let mut combined = left;
            combined.extend(right);
            combined
        }
    }
}

fn format_curve<C: BezierCurve>(curve: &C) -> String
where C::Point: Coordinate2D {
    let start_point = curve.start_point();
    let end_point = curve.end_point();
    let (cp1, cp2) = curve.control_points();

    format!("[B({:?}, {:?}, {:?}, {:?}, u), B({:?}, {:?}, {:?}, {:?}, u)]", 
        start_point.x(), cp1.x(), cp2.x(), end_point.x(),
        start_point.y(), cp1.y(), cp2.y(), end_point.y())
}

///
/// Determines the points at which two curves intersect using the Bezier clipping algorithm
/// 
fn curve_intersects_curve_clip_inner<'a, C: BezierCurve>(curve1: CurveSection<'a, C>, curve2: CurveSection<'a, C>, accuracy_squared: f64) -> Vec<(f64, f64)>
where C::Point: 'a+Coordinate2D {
    // We'll iterate on the two curves
    let mut curve1 = curve1;
    let mut curve2 = curve2;

    // If a curve stops shrinking, we need to subdivide it to continue the match
    let mut curve1_last_len = curve_hull_length_sq(&curve1);
    let mut curve2_last_len = curve_hull_length_sq(&curve2);

    // Edge case: 0-length curves have no match
    if curve1_last_len == 0.0 { return vec![]; }
    if curve2_last_len == 0.0 { return vec![]; }

    println!("/* Search {:?} {:?} */ ParametricPlot([{}, {}], u=[0,1])", curve1.original_curve_t_values(), curve2.original_curve_t_values(), format_curve(&curve1), format_curve(&curve2));

    // Iterate to refine the match
    loop {
        if curve2_last_len > accuracy_squared {
            // Clip curve2 against curve1
            let clip_t  = clip(&curve2, &curve1);
            let clip_t  = match clip_t {
                None            => { return vec![]; }
                Some(clip_t)    => clip_t
            };

            curve2 = curve2.subsection(clip_t.0, clip_t.1);

            println!("ParametricPlot([{}, {}], u=[0,1])", format_curve(&curve1), format_curve(&curve2));

            // Work out the length of the new curve
            let curve2_len = curve_hull_length_sq(&curve2);

            // If the curve doesn't shrink at least 20%, subdivide it
            if curve2_last_len*0.8 < curve2_len {
                let (left, right)   = (curve2.subsection(0.0, 0.5), curve2.subsection(0.5, 1.0));

                println!("/* SubLeft */");
                let left            = curve_intersects_curve_clip_inner(curve1.clone(), left, accuracy_squared);

                println!("/* SubRight */");
                let right           = curve_intersects_curve_clip_inner(curve1.clone(), right, accuracy_squared);

                return join_subsections(&curve1, left, right, accuracy_squared);
            }

            // Update the length of the curve
            curve2_last_len = curve2_len;
        }

        if curve1_last_len > accuracy_squared {
            // Clip curve1 against curve2
            let clip_t  = clip(&curve1, &curve2);
            let clip_t  = match clip_t {
                None            => { return vec![]; }
                Some(clip_t)    => clip_t
            };

            curve1 = curve1.subsection(clip_t.0, clip_t.1);

            // Work out the length of the new curve
            let curve1_len = curve_hull_length_sq(&curve1);

            // If the curve doesn't shrink at least 20%, subdivide it
            if curve1_last_len*0.8 < curve1_len {
                let (left, right)   = (curve1.subsection(0.0, 0.5), curve1.subsection(0.5, 1.0));

                println!("/* SubLeft */");
                let left            = curve_intersects_curve_clip_inner(left, curve2.clone(), accuracy_squared);

                println!("/* SubRight */");
                let right           = curve_intersects_curve_clip_inner(right, curve2, accuracy_squared);

                return join_subsections(&curve1, left, right, accuracy_squared);
            }

            // Update the length of the curve
            curve1_last_len = curve1_len;
        }

        if curve1_last_len <= accuracy_squared && curve2_last_len <= accuracy_squared {
            // Found a point to the required accuracy: return it, in coordinates relative to the original curve
            let (t_min1, t_max1) = curve1.original_curve_t_values();
            let (t_min2, t_max2) = curve2.original_curve_t_values();

            println!("/* Found {:?} */", (t_min1, t_min2));

            return vec![((t_min1+t_max1)*0.5, (t_min2+t_max2)*0.5)];
        }
    }
}

///
/// Determines the points at which two curves intersect using the Bezier clipping
/// algorihtm
/// 
pub fn curve_intersects_curve_clip<'a, C: BezierCurve>(curve1: &'a C, curve2: &'a C, accuracy: f64) -> Vec<(f64, f64)>
where C::Point: 'a+Coordinate2D {
    println!("B(a,b,c,d,t) = a*(1-t)^3 + 3*b*(1-t)^2*t + 3*c*(1-t)*t^2 + d*t^3");

    // Start with the entire span of both curves
    let curve1 = curve1.section(0.0, 1.0);
    let curve2 = curve2.section(0.0, 1.0);

    // Perform the clipping algorithm on these curves
    curve_intersects_curve_clip_inner(curve1, curve2, accuracy*accuracy)
}
