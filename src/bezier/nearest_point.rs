use super::curve::*;
use super::section::*;
use crate::geo::*;
use crate::line::*;

///
/// Computes the t position of the nearest point on a curve using a subdivision algorithm
///
pub fn nearest_point_on_curve_subdivision<C, P>(curve: &C, point: &P, precision: f64) -> f64
where
    C:          BezierCurve<Point=P>,
    C::Point:   Coordinate2D,
    P:          Coordinate + Coordinate2D,
{
    nearest_point_on_curve_subsection(curve.section(0.0, 1.0), point, precision)
}

///
/// Returns the minimum and maximum distances between a point and a curve section
///
fn distances_for_section<C, P>(section: CurveSection<'_, C>, point: &P) -> (f64, f64)
where
    C:          BezierCurve<Point=P>,
    C::Point:   Coordinate2D,
    P:          Coordinate + Coordinate2D,
{
    let start_point     = section.start_point();
    let end_point       = section.end_point();
    let (cp1, cp2)      = section.control_points();

    // Decide on the top and bottom lines of the section
    let baseline        = (start_point, end_point);
    let (top, bottom)   = if baseline.which_side(&cp1) == baseline.which_side(&cp2) {
        // Both control points on the same side as the baseline
        (baseline, (cp1, cp2))
    } else {
        // Control points on opposite sides
        ((baseline.0, cp2), (cp1, baseline.1))
    };

    // Distance to the two parts of the section
    let distance_top    = top.point_at_pos(0.5).distance_to(point);
    let distance_bottom = bottom.point_at_pos(0.5).distance_to(point);

    (f64::min(distance_top, distance_bottom), f64::max(distance_top, distance_bottom))
}

///
/// `nearest_point_on_curve_subdivision` but operating on curve sections 
///
fn nearest_point_on_curve_subsection<C, P>(curve: CurveSection<'_, C>, point: &P, precision: f64) -> f64
where
    C:          BezierCurve<Point=P>,
    C::Point:   Coordinate2D,
    P:          Coordinate + Coordinate2D,
{
    // This is a fairly straightforward algorithm: we know the curve must lie between the lines formed by the start and end points and the control points,
    // so we search for the closest point by taking the distance to those two lines as a minimum and maximum value for the distance of the point and discard
    // sections when the two values are outside of that range (or assume we've found a point when the range is within the precision value)
    //
    // This algorithm is not especially fast, but it should be reasonably reliable.
    let (min, max)          = distances_for_section(curve.clone(), point);
    let mut section_stack   = vec![(curve, min, max)];

    let mut candidates      = vec![];

    while let Some((section, _min, max)) = section_stack.pop() {
        // Remove any section from the stack that is definitely further away than this section
        section_stack.retain(|(_section, section_min, _section_max)| *section_min < max);

        // Divide the section into 2
        let section_1       = section.subsection(0.0, 0.5);
        let section_2       = section.subsection(0.5, 1.0);

        // Calculate the distances to the hull of the two sections
        let (min_1, max_1)  = distances_for_section(section_1.clone(), point);
        let (min_2, max_2)  = distances_for_section(section_2.clone(), point);

        // If the min/max values are within precision of each other, then we may have found a close point
        if section.start_point().distance_to(&section.end_point()) <= precision {
            if (max_1 - min_1).abs() <= precision {
                candidates.push(section_1.t_for_t(0.5));
                continue;
            }

            if (max_2 - min_2).abs() <= precision {
                candidates.push(section_2.t_for_t(0.5));
                continue;
            }
        }

        // Add the two sections to consideration (or just the closest section if they don't overlap)
        if min_1 < max_2 {
            section_stack.push((section_1, min_1, max_1));
        }

        if min_2 < max_1 {
            section_stack.push((section_2, min_2, max_2));
        }
    }

    candidates.pop().unwrap_or(0.0)
}
