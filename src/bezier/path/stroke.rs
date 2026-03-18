use super::path::*;
use super::arithmetic::*;

use crate::geo::*;
use crate::bezier::*;
use crate::line::*;
use crate::arc::*;

use std::f64;

///
/// How two segments of a line should be joined together
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

///
/// How the end of a line should be drawn
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square
}

///
/// Settings for a line stroke operation
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct StrokeOptions {
    /// How accurately to match the curves,
    accuracy: f64,

    /// The minimum distance between samples, when the minimum tangent is not reached
    min_sample_distance: f64,

    /// How two lines should be joined together
    join: LineJoin,

    /// How to start the line
    start_cap: LineCap,

    /// How to finish the line
    end_cap: LineCap,

    /// Set to true if the interior points should be removed from the resulting stroke (producing a path that is always non-overlapping)
    remove_interior_points: bool,

    /// True if the path that's generated should be closed
    closed: bool,
}

impl Default for StrokeOptions {
    #[inline]
    fn default() -> Self {
        StrokeOptions {
            accuracy:               0.1,
            min_sample_distance:    0.1,
            join:                   LineJoin::Bevel,
            start_cap:              LineCap::Butt,
            end_cap:                LineCap::Butt,
            remove_interior_points: false,
            closed:                 false,
        }
    }
}

impl StrokeOptions {
    ///
    /// Sets the maximum distance allowed between the result and the ideal curve
    ///
    /// Setting this to lower values can result in more curves the fit the offset curves more precisely
    ///
    #[inline]
    pub fn with_accuracy(mut self, accuracy: f64) -> Self {
        self.accuracy = accuracy;
        self
    }

    ///
    /// Provides a lower limit on the length that a curve will be subdivided to when trying to fit the offset curve
    ///
    /// Lower values produce more accurate curves but can generate large numbers of samples (so takes longer)
    /// This is used as a lower limit when the min tangent is never reached (usually for very high curvature curves)
    ///
    #[inline]
    pub fn with_min_sample_distance(mut self, min_sample_distance: f64) -> Self {
        self.min_sample_distance = min_sample_distance;
        self
    }

    ///
    /// If two sections have a large difference in angles, this specifies how the two sections should be joined 
    ///
    #[inline]
    pub fn with_join(mut self, join: LineJoin) -> Self {
        self.join = join;
        self
    }

    ///
    /// Sets the type of start cap to generate 
    ///
    #[inline]
    pub fn with_start_cap(mut self, start_cap: LineCap) -> Self {
        self.start_cap = start_cap;
        self
    }

    ///
    /// Sets the type of end cap to generate 
    ///
    #[inline]
    pub fn with_end_cap(mut self, end_cap: LineCap) -> Self {
        self.end_cap = end_cap;
        self
    }

    ///
    /// Indicates that the path should be post-processed to remove any interior points
    ///
    /// By default, this option is not set. In this state, the generated path may self-overlap, so will need to be rendered with a non-zero
    /// winding rule. If this is set, the resulting path will be processed to remove any overlapping sections, and should be rendered using
    /// the even-odd winding rule.
    ///
    #[inline]
    pub fn with_remove_interior_points(mut self) -> Self {
        self.remove_interior_points = true;
        self
    }

    ///
    /// Indicates that this path should be generated as a closed path (generated as two paths, one inside the other)
    ///
    #[inline]
    pub fn with_closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }
}

impl LineJoin {
    ///
    /// Returns the function to use for joining line segments together for a particular join style
    ///
    #[inline]
    fn join_function<TCoord>(&self) -> impl Fn(TCoord, (TCoord, TCoord), (TCoord, TCoord), f64) -> Vec<(TCoord, (TCoord, TCoord), TCoord)>
    where
        TCoord: Coordinate + Coordinate2D,
    {
        match self {
            LineJoin::Miter     => miter_join,
            LineJoin::Round     => round_join,
            LineJoin::Bevel     => bevel_join,
        }
    }
}

///
/// The bevel join is the simplest way to join two lines, it will just join the two coordinates together
///
#[inline]
fn bevel_join<TCoord>(_join_point: TCoord, (start_point, _start_tangent): (TCoord, TCoord), (end_point, _end_tangent): (TCoord, TCoord), _limit: f64) -> Vec<(TCoord, (TCoord, TCoord), TCoord)>
where
    TCoord: Coordinate + Coordinate2D,
{
    vec![line_to_bezier::<Curve<_>>(&(start_point, end_point)).all_points()]
}

///
/// The miter join extends the lines from the two edges of the curve until they meet (or up until a particular limit)
///
#[inline]
fn miter_join<TCoord>(join_point: TCoord, start_line: (TCoord, TCoord), end_line: (TCoord, TCoord), limit: f64) -> Vec<(TCoord, (TCoord, TCoord), TCoord)>
where
    TCoord: Coordinate + Coordinate2D,
{
    // Must be the outer part of the corner
    if start_line.angle_to(&end_line) > f64::consts::PI {
        // Find where the curves intersect
        if let Some(final_point) = ray_intersects_ray(&start_line, &end_line) {
            if start_line.0.is_near_to(&final_point, limit) {
                // Draw to the intersection point if the lines are shorter than the limit
                vec![
                    line_to_bezier::<Curve<_>>(&(start_line.0, final_point)).all_points(),
                    line_to_bezier::<Curve<_>>(&(final_point, end_line.0)).all_points(),
                ]
            } else {
                // Draw to the limit if the lines are very long
                let start_vector    = (start_line.0 - start_line.1).to_unit_vector();
                let end_vector      = (end_line.0 - end_line.1).to_unit_vector();
                let start_vector    = start_vector * limit;
                let end_vector      = end_vector * limit;

                vec![
                    line_to_bezier::<Curve<_>>(&(start_line.0, start_line.0 + start_vector)).all_points(),
                    line_to_bezier::<Curve<_>>(&(start_line.0 + start_vector, end_line.0 + end_vector)).all_points(),
                    line_to_bezier::<Curve<_>>(&(end_line.0 + end_vector, end_line.0)).all_points(),
                ]
            }
        } else {
            // If the rays don't intersect, use a bevel join instead
            bevel_join(join_point, start_line, end_line, limit)
        }
    } else {
        // Bevel join on the inside part of the corner
        bevel_join(join_point, start_line, end_line, limit)
    }
}

///
/// The round join joins two edges using an arc
///
#[inline]
fn round_join<TCoord>(join_point: TCoord, start_line: (TCoord, TCoord), end_line: (TCoord, TCoord), limit: f64) -> Vec<(TCoord, (TCoord, TCoord), TCoord)>
where
    TCoord: Coordinate + Coordinate2D,
{
    const VERY_CLOSE: f64 = 1e-5;

    // Must be the outer part of the corner, and not too flat
    if !start_line.0.is_near_to(&end_line.0, VERY_CLOSE) {
        // Curve goes between the start of the two lines (both of which are moving away from the corner)
        let start_point = &start_line.0;
        let end_point   = &end_line.0;

        // Start/end tangents (recall that the lines are both moving away from the corner)
        let start_tangent   = (start_line.1 - start_line.0).to_unit_vector();
        let end_tangent     = (end_line.0 - end_line.1).to_unit_vector();

        // Center point is where the lines along the normal vectors intercept
        let center_point    = join_point;
        let radius          = center_point.distance_to(&start_point);

        // Construct an arc to join the two points
        let theta   = start_tangent.dot(&end_tangent).acos();
        let ratio   = -(4.0/3.0)*((theta/4.0).tan());
        let cp1     = *start_point + start_tangent * radius * ratio;
        let cp2     = *end_point - end_tangent * radius * ratio;

        debug_assert!((center_point.distance_to(&start_point) - center_point.distance_to(&end_point)).abs() < 0.01, "Center point is not centered ({} vs {})", center_point.distance_to(&start_point), center_point.distance_to(&end_point));

        vec![(*start_point, (cp1, cp2), *end_point)]
    } else {
        // Bevel join on the inside part of the corner
        bevel_join(join_point, start_line, end_line, limit)
    }
}

///
/// Generates the edges for a single curve, returning true if any extra points are added to the points list
///
/// The start point is supplied as two coordinates: the initial point of the curve, and the tangent at that point. It's
/// updated by this call if it's not already set (as the points contain only the control points and the )
///
fn stroke_edge<TCoord>(start_point: &mut Option<(TCoord, TCoord)>, points: &mut Vec<(TCoord, TCoord, TCoord)>, curve: &Curve<TCoord>, subdivision_options: &SubdivisionOffsetOptions, width: f64, join: &impl Fn(TCoord, (TCoord, TCoord), (TCoord, TCoord), f64) -> Vec<(TCoord, (TCoord, TCoord), TCoord)>) -> bool
where
    TCoord: Coordinate + Coordinate2D,
{
    let mut added_points = false;

    // Offset this curve using the subdivision algorithm
    if let Some(offset_curve) = offset_lms_subdivisions(curve, |_| width, |_| 0.0, &subdivision_options) {
        // Compute the initial point and its tangent
        let initial_point   = offset_curve[0].start_point();
        let initial_tangent = offset_curve[0].tangent_at_pos(0.0);
        let initial_tangent = initial_point + initial_tangent;

        if let Some((start_point, _)) = start_point {
            // Get the curve that preceeds this line
            let mut points_rev_iter = points.iter().rev();
            let last_curve          = points_rev_iter.next().map(|(cp1, cp2, ep)| {
                let sp = points_rev_iter.next().map(|(_, _, ep)| ep).unwrap_or(start_point);
                Curve::from_points(*sp, (*cp1, *cp2), *ep)
            }).unwrap_or(offset_curve[0]);

            let last_point      = last_curve.end_point();
            let last_tangent    = last_curve.tangent_at_pos(1.0);
            let last_tangent    = last_point - last_tangent;

            // Add a join to the existing curve using the join style
            for (_, (cp1, cp2), ep) in join(curve.start_point(), (last_point, last_tangent), (initial_point, initial_tangent), width * 4.0) {
                points.push((cp1, cp2, ep));
            }

            added_points = true;
        } else {
            // Start a new curve
            *start_point = Some((initial_point, initial_tangent));
        }

        // Add the remaining points
        for new_curve in offset_curve {
            let (_, (cp1, cp2), ep) = new_curve.all_points();
            points.push((cp1, cp2, ep));

            added_points = true;
        }
    }

    added_points
}

///
/// Closes a curve using the join function
///
fn close_stroke<TCoord>(start_point: &Option<(TCoord, TCoord)>, points: &mut Vec<(TCoord, TCoord, TCoord)>, width: f64, join: &impl Fn(TCoord, (TCoord, TCoord), (TCoord, TCoord), f64) -> Vec<(TCoord, (TCoord, TCoord), TCoord)>)
where 
    TCoord: Coordinate + Coordinate2D,
{
    // Close by creating a join to the last point
    let last_point          = points.last();
    let last_start_point    = points.len().checked_sub(2).and_then(|idx| points.get(idx));

    if let (Some((start_point, start_tangent)), Some((_, _, last_start_point)), Some((cp1, cp2, last_point))) = (start_point, last_start_point, last_point) {
        // Create a join to the original start point
        let last_tangent = Curve::from_points(*last_start_point, (*cp1, *cp2), *last_point).tangent_at_pos(1.0);

        for (_, (cp1, cp2), ep) in join(*start_point, (*last_point, last_tangent), (*start_point, *start_tangent), width * 4.0) {
            points.push((cp1, cp2, ep));
        }
    }
}

///
/// Creates a path from a set of points we generated as part of a stroke
///
fn create_path<TPathFactory>(start_point: &Option<(TPathFactory::Point, TPathFactory::Point)>, points: Vec<(TPathFactory::Point, TPathFactory::Point, TPathFactory::Point)>) -> Option<TPathFactory> 
where
    TPathFactory:           BezierPathFactory,
    TPathFactory::Point:    Coordinate + Coordinate2D,
{
    // Result is the path if we generated at least 2 points
    if let Some(start_point) = start_point {
        if points.len() > 0 {
            let path = TPathFactory::from_points(start_point.0, points.into_iter());
            Some(path)
        } else {
            // Only generated one point
            None
        }
    } else {
        // Never generated a curve
        None
    }
}

///
/// Creates an endcap between the 'from' and 'to' points by assuming we've already reached the 'from' point
///
#[inline]
fn end_cap<TCoord>(points: &mut Vec<(TCoord, TCoord, TCoord)>, from_coord: TCoord, to_coord: TCoord, end_cap_type: LineCap) -> bool 
where
    TCoord: Coordinate + Coordinate2D,
{
    match end_cap_type {
        LineCap::Butt   => butt_end_cap(points, from_coord, to_coord),
        LineCap::Round  => round_end_cap(points, from_coord, to_coord),
        LineCap::Square => square_end_cap(points, from_coord, to_coord),
    }
}

///
/// Adds an endcap between the 'from' and 'to' points to the end of the points list (possibly updating the 'start' coordinate)
///
/// We assume that we're already at 'from_coord'
///
fn butt_end_cap<TCoord>(points: &mut Vec<(TCoord, TCoord, TCoord)>, from_coord: TCoord, to_coord: TCoord) -> bool
where 
    TCoord: Coordinate + Coordinate2D,
{
    const VERY_CLOSE: f64 = 1e-5;

    // If the start & end points are very close together, then we don't add any points
    if from_coord.is_near_to(&to_coord, VERY_CLOSE) {
        return false;
    }

    // Draw a line between the start point and the end point
    let cp1 = (to_coord - from_coord) * (1.0/3.0) + from_coord;
    let cp2 = (to_coord - from_coord) * (2.0/3.0) + from_coord;

    points.push((cp1, cp2, to_coord));

    true
}

///
/// Adds a 'rounded' endcap between the 'from' and 'to' points to the end of the points list (possibly updating the 'start' coordinate)
///
/// We assume that we're already at 'from_coord'
///
fn round_end_cap<TCoord>(points: &mut Vec<(TCoord, TCoord, TCoord)>, from_coord: TCoord, to_coord: TCoord) -> bool
where 
    TCoord: Coordinate + Coordinate2D,
{
    const VERY_CLOSE: f64 = 1e-5;

    // If the start & end points are very close together, then we don't add any points
    if from_coord.is_near_to(&to_coord, VERY_CLOSE) {
        return false;
    }

    // The most recent point added to the list defines the tangent (use a butt endcap if the curve has no coordinates)
    let Some((_, cp2, ep)) = points.last() else { return butt_end_cap(points, from_coord, to_coord); };

    let diameter = from_coord.distance_to(&to_coord);
    let radius   = diameter / 2.0;
    let tangent  = *ep - *cp2;

    // Center point is the midpoint between from_coord and to_coord
    let mid_x   = (from_coord.x() + to_coord.x()) / 2.0;
    let mid_y   = (from_coord.y() + to_coord.y()) / 2.0;
    let center  = TCoord::from_components(&[mid_x, mid_y]);

    // Calculate start angle based on the tangent direction
    let tangent_angle   = f64::atan2(-tangent.y(), tangent.x());
    let start_angle     = tangent_angle;
    let end_angle       = start_angle + f64::consts::PI;

    // Create a circle at the center with the calculated radius
    let circle  = Circle::new(center, radius);
    let arc1    = circle.arc(start_angle, start_angle + (f64::consts::PI / 2.0));
    let curve1  = arc1.to_bezier_curve::<Curve<_>>();
    let arc2    = circle.arc(start_angle + (f64::consts::PI / 2.0), end_angle);
    let curve2  = arc2.to_bezier_curve::<Curve<_>>();

    // Add the control points and end point
    let (_, (cp1, cp2), ep) = curve1.all_points();
    points.push((cp1, cp2, ep));

    let (_, (cp1, cp2), ep) = curve2.all_points();
    points.push((cp1, cp2, ep));

    true
}

///
/// Adds an endcap between the 'from' and 'to' points to the end of the points list (possibly updating the 'start' coordinate)
///
/// We assume that we're already at 'from_coord'
///
fn square_end_cap<TCoord>(points: &mut Vec<(TCoord, TCoord, TCoord)>, from_coord: TCoord, to_coord: TCoord) -> bool
where 
    TCoord: Coordinate + Coordinate2D,
{
    const VERY_CLOSE: f64 = 1e-5;

    // If the start & end points are very close together, then we don't add any points
    if from_coord.is_near_to(&to_coord, VERY_CLOSE) {
        return false;
    }

    // The most recent point added to the list defines the tangent (use a butt endcap if the curve has no coordinates)
    let Some((_, cp2, ep)) = points.last() else { return butt_end_cap(points, from_coord, to_coord); };

    let diameter = from_coord.distance_to(&to_coord);
    let radius   = diameter / 2.0;
    let tangent  = (*ep - *cp2).to_unit_vector();

    // We draw three lines to create the endcap
    let p1 = from_coord + (tangent*radius);
    let p2 = to_coord + (tangent*radius);
    let p3 = to_coord;

    let l1 = line_to_bezier::<Curve<_>>(&(from_coord, p1));
    let l2 = line_to_bezier::<Curve<_>>(&(p1, p2));
    let l3 = line_to_bezier::<Curve<_>>(&(p2, p3));

    for line in [l1, l2, l3] {
        let (_, (cp1, cp2), ep) = line.all_points();
        points.push((cp1, cp2, ep));
    }

    true
}

///
/// Generates a thickened line along a path
///
/// The width describes how wide to make the resulting line. 
///
pub fn stroke_path<TPathFactory, TCoord>(path: &impl BezierPath<Point=TCoord>, width: f64, options: &StrokeOptions) -> Vec<TPathFactory>
where
    TPathFactory:   BezierPathFactory<Point=TCoord>,
    TCoord:         Coordinate + Coordinate2D,
{
    const VERY_CLOSE: f64 = 1e-5;

    // Half the width (we add and subtract this from the centerline)
    let half_width  = width/2.0;
    let join_fn     = options.join.join_function();

    // Create the list of points that make up the path
    let mut start_point                 = None;
    let mut points                      = vec![];
    let mut paths: Vec<TPathFactory>    = vec![];

    // Convert the path to curves
    let path_curves = path.to_curves::<Curve<TCoord>>();

    // Create subdivision options, using the width of the curve as a guide
    let subdivision_options = SubdivisionOffsetOptions::default()
        .with_min_distance(options.min_sample_distance)
        .with_max_error(options.accuracy)
        .with_max_distance(width * 20.0);

    // Draw forward
    for curve in path_curves.iter() {
        // Offset this curve using the subdivision algorithm
        stroke_edge(&mut start_point, &mut points, &curve, &subdivision_options, half_width, &join_fn);
    }

    if options.closed {
        // Close the stroke
        close_stroke(&start_point, &mut points, width, &join_fn);

        // Create a subpath from these curves
        paths.extend(create_path(&start_point, points));

        // Start a new path for the inner part of the stroke
        start_point = None;
        points      = vec![];
    }

    // Draw backwards (only add the end cap if we're not closing the path)
    let mut added_end_cap = if options.closed { true } else { false };

    for curve in path_curves.iter().rev().map(|curve| curve.reverse()) {
        if added_end_cap {
            // Add an offset edge to the curve
            stroke_edge(&mut start_point, &mut points, &curve, &subdivision_options, half_width, &join_fn);
        } else {
            // Don't add an endcap to a very short curve (which we determine by measuring the length covered by the control polygon)
            let (sp, (cp1, cp2), ep) = curve.all_points();
            let polygon_length = sp.distance_to(&cp1) + cp1.distance_to(&cp2) + cp2.distance_to(&ep);

            if polygon_length < VERY_CLOSE {
                continue;
            }

            // Add an endcap to the first point of the curve
            if let Some((_, _, last_point)) = points.last() {
                // Use the normal at the start of the curve to calculate where the initial point of the reverse section of the curve should go
                let last_point      = *last_point;
                let initial_normal  = curve.normal_at_pos(0.0).to_unit_vector();
                let curve_start     = curve.point_at_pos(0.0) + (initial_normal * half_width);

                end_cap(&mut points, last_point, curve_start, options.end_cap);
            }

            added_end_cap = true;

            // Stroke the curve as normal once this is done
            stroke_edge(&mut start_point, &mut points, &curve, &subdivision_options, half_width, &bevel_join);
        }
    }

    if options.closed {
        // Close the last part of the path
        close_stroke(&start_point, &mut points, width, &join_fn);
    } else {
        // Add start cap
        if let (Some(start_point), Some(end_point)) = (start_point, points.last().map(|(_, _, p)| p).copied()) {
            end_cap(&mut points, end_point, start_point.0, options.start_cap);
        }
    }

    // Generate the path
    paths.extend(create_path(&start_point, points));
    if !paths.is_empty() {
        if options.remove_interior_points {
            path_remove_interior_points(&paths, options.accuracy)
        } else {
            paths
        }
    } else {
        paths
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bevel_join_90_degrees() {
        let corner = bevel_join(Coord2(2.5, 2.5), (Coord2(2.0, 2.0), Coord2(1.0, 2.0)), (Coord2(3.0, 3.0), Coord2(3.0, 2.0)), 20.0);
        println!("{:?}", corner);

        let (sp, (_cp1, _cp2), ep) = corner.last().unwrap();
        assert!(ep.is_near_to(&Coord2(3.0, 3.0), 0.01), "End point is wrong (found {:?})", ep);
        assert!(sp.is_near_to(&Coord2(2.0, 2.0), 0.01), "Start point is wrong (found {:?})", sp);
    }

    #[test]
    fn rounded_join_90_degrees() {
        let corner = round_join(Coord2(2.0, 3.0), (Coord2(2.0, 2.0), Coord2(1.0, 2.0)), (Coord2(3.0, 3.0), Coord2(3.0, 4.0)), 20.0);
        println!("{:?}", corner);

        let (sp, (cp1, cp2), ep) = corner.last().unwrap();

        // Check the distance from the center point around the curve (should be 1 px away all the way around)
        let curve = Curve::from_points(*sp, (*cp1, *cp2), *ep);
        for t in 0..100 {
            let t           = (t as f64) / 100.0;
            let p           = curve.point_at_pos(t);
            let distance    = p.distance_to(&Coord2(2.0, 3.0));

            assert!((distance-1.0).abs() < 0.01, "Distance at t={} is {:?}", t, distance);
        }

        assert!(sp.is_near_to(&Coord2(2.0, 2.0), 0.01), "Start point is wrong (found {:?})", sp);
        assert!(ep.is_near_to(&Coord2(3.0, 3.0), 0.01), "End point is wrong (found {:?})", ep);
    }

    #[test]
    fn rounded_join_180_degrees() {
        let corner = round_join(Coord2(2.0, 2.5), (Coord2(2.0, 2.0), Coord2(1.0, 2.0)), (Coord2(2.0, 3.0), Coord2(1.0, 3.0)), 20.0);
        println!("{:?}", corner);

        let (sp, (_cp1, _cp2), ep) = corner.last().unwrap();
        assert!(ep.is_near_to(&Coord2(2.0, 3.0), 0.01), "End point is wrong (found {:?})", ep);
        assert!(sp.is_near_to(&Coord2(2.0, 2.0), 0.01), "Start point is wrong (found {:?})", sp);
    }

    #[test]
    fn rounded_join_reverse_direction_1() {
        let corner = round_join(Coord2(2.0, 2.5), (Coord2(2.0, 2.0), Coord2(1.0, 2.0)), (Coord2(2.0, 3.0), Coord2(1.0, 3.0)), 20.0);
        println!("{:?}", corner);

        let (sp, (cp1, cp2), ep) = corner.last().unwrap();

        // Check the distance from the center point around the curve (should be 1 px away all the way around)
        let curve = Curve::from_points(*sp, (*cp1, *cp2), *ep);
        for t in 0..100 {
            let t           = (t as f64) / 100.0;
            let p           = curve.point_at_pos(t);
            let distance    = p.distance_to(&Coord2(2.0, 2.5));

            assert!((distance-0.5).abs() < 0.01, "Distance at t={} is {:?}", t, distance);
        }

        assert!(sp.is_near_to(&Coord2(2.0, 2.0), 0.01), "Start point is wrong (found {:?})", sp);
        assert!(ep.is_near_to(&Coord2(2.0, 3.0), 0.01), "End point is wrong (found {:?})", ep);
    }

    #[test]
    fn rounded_join_reverse_direction_2() {
        let corner = round_join(Coord2(2.0, 2.5), (Coord2(2.0, 2.0), Coord2(1.0, 1.99)), (Coord2(2.0, 3.0), Coord2(1.0, 3.01)), 20.0);
        println!("{:?}", corner);

        let (sp, (cp1, cp2), ep) = corner.last().unwrap();

        // Check the distance from the center point around the curve (should be 1 px away all the way around)
        let curve = Curve::from_points(*sp, (*cp1, *cp2), *ep);
        for t in 0..100 {
            let t           = (t as f64) / 100.0;
            let p           = curve.point_at_pos(t);
            let distance    = p.distance_to(&Coord2(2.0, 2.5));

            assert!((distance-0.5).abs() < 0.01, "Distance at t={} is {:?}", t, distance);
        }

        assert!(sp.is_near_to(&Coord2(2.0, 2.0), 0.01), "Start point is wrong (found {:?})", sp);
        assert!(ep.is_near_to(&Coord2(2.0, 3.0), 0.01), "End point is wrong (found {:?})", ep);
    }

    #[test]
    fn rounded_join_near_flat_1() {
        let corner = round_join(Coord2(2.0, 3.0), (Coord2(2.0, 2.0), Coord2(1.0, 2.0)), (Coord2(2.1, 2.0), Coord2(3.0, 2.001)), 20.0);
        println!("{:?}", corner);

        let (sp, (cp1, cp2), ep) = corner.last().unwrap();

        // Check the distance from the center point around the curve (should be 1 px away all the way around)
        let curve = Curve::from_points(*sp, (*cp1, *cp2), *ep);
        for t in 0..100 {
            let t           = (t as f64) / 100.0;
            let p           = curve.point_at_pos(t);
            let distance    = p.distance_to(&Coord2(2.05, 2.0));

            assert!(distance < 0.05, "Distance at t={} is {:?}", t, distance);
        }

        assert!(sp.is_near_to(&Coord2(2.0, 2.0), 0.01), "Start point is wrong (found {:?})", sp);
        assert!(ep.is_near_to(&Coord2(2.1, 2.0), 0.01), "End point is wrong (found {:?})", ep);
    }

    #[test]
    fn rounded_join_near_flat_2() {
        let corner = round_join(Coord2(2.0, 3.0), (Coord2(2.0, 2.0), Coord2(1.0, 2.0)), (Coord2(2.0001, 2.0), Coord2(3.0, 2.001)), 20.0);
        println!("{:?}", corner);

        let (sp, (cp1, cp2), ep) = corner.last().unwrap();

        // Check the distance from the center point around the curve (should be 1 px away all the way around)
        let curve = Curve::from_points(*sp, (*cp1, *cp2), *ep);
        for t in 0..100 {
            let t           = (t as f64) / 100.0;
            let p           = curve.point_at_pos(t);
            let distance    = p.distance_to(&Coord2(2.00005, 2.0));

            assert!(distance < 0.0005, "Distance at t={} is {:?}", t, distance);
        }

        assert!(sp.is_near_to(&Coord2(2.0, 2.0), 0.01), "Start point is wrong (found {:?})", sp);
        assert!(ep.is_near_to(&Coord2(2.0001, 2.0), 0.01), "End point is wrong (found {:?})", ep);
    }

    #[test]
    fn rounded_join_near_flat_3() {
        let corner = round_join(Coord2(0.7801181077957153, -0.4963679909706116), (Coord2(0.7645138179983775, -0.49717221697902736), Coord2(0.8159842810030481, -1.495846734245832)), (Coord2(0.7644955725706675, -0.49609044332775987), Coord2(0.7822586211837961, 0.5037517812776866)), 20.0);
        println!("{:?}", corner);

        let (sp, (cp1, cp2), ep) = corner.last().unwrap();

        // Check the distance from the center point around the curve (should be 1 px away all the way around)
        let curve = Curve::from_points(*sp, (*cp1, *cp2), *ep);
        for t in 0..100 {
            let t           = (t as f64) / 100.0;
            let p           = curve.point_at_pos(t);
            let distance    = p.distance_to(&Coord2(0.7801181077957153, -0.4963679909706116));

            assert!(distance < 0.1, "Distance at t={} is {:?}", t, distance);
        }

        assert!(sp.is_near_to(&Coord2(0.7645138179983775, -0.49717221697902736), 0.01), "Start point is wrong (found {:?})", sp);
        assert!(ep.is_near_to(&Coord2(0.7644955725706675, -0.49609044332775987), 0.01), "End point is wrong (found {:?})", ep);
    }

    #[test]
    fn circle1() {
        // Found errors at these coordinates/widths
        let w = 3.0;
        let r = 20.0;
        let x = 100.0;
        let y = 200.0;

        // Create a circle path
        let circle = Circle::new(Coord2(x, y), r);
        let circle = circle.to_path::<SimpleBezierPath>();

        // Stroke the path to generate the circle
        let thick_path = stroke_path::<SimpleBezierPath, _>(&circle, w, 
            &StrokeOptions::default()
                .with_accuracy(0.002)
                .with_min_sample_distance(0.001)
                .with_start_cap(LineCap::Butt)
                .with_end_cap(LineCap::Butt)
                .with_join(LineJoin::Miter));

        // Every point in the path must be w/2 away the point in the center
        for curve in thick_path.iter().flat_map(|section| section.to_curves::<Curve<Coord2>>()) {
            // Don't test the end lines
            if curve.characteristics() == CurveCategory::Linear {
                continue;
            }

            println!("{:?}", curve);

            // Check points on the curve
            for t in 0..100 {
                let t = (t as f64)/100.0;
                let p = curve.point_at_pos(t);

                let d = p.distance_to(&Coord2(x, y));

                assert!((d-(r-(w/2.0))).abs() < 0.1 || (d-(r+(w/2.0))).abs() < 0.1, "d={} ({} or {})", d, r-(w/2.0), r+(w/2.0));
            }
        }
    }

    #[test]
    fn circle2a() {
        // Found errors at these coordinates/widths
        let w = 11.449928283691406;
        let r = 20.0;
        let x = 172.17343139648438;
        let y = 215.4249267578125;

        // Create a circle path
        let circle = Circle::new(Coord2(x, y), r);
        let circle = circle.to_path::<SimpleBezierPath>();

        // Stroke the path to generate the circle
        let thick_path = stroke_path::<SimpleBezierPath, _>(&circle, w, 
            &StrokeOptions::default()
                .with_accuracy(0.002)
                .with_min_sample_distance(0.001)
                .with_start_cap(LineCap::Butt)
                .with_end_cap(LineCap::Butt)
                .with_join(LineJoin::Round));

        // Every point in the path must be w/2 away the point in the center
        for curve in thick_path.iter().flat_map(|section| section.to_curves::<Curve<Coord2>>()) {
            // Don't test the end lines
            if curve.characteristics() == CurveCategory::Linear {
                continue;
            }

            println!("{:?}", curve);

            // Check points on the curve
            for t in 0..100 {
                let t = (t as f64)/100.0;
                let p = curve.point_at_pos(t);

                let d = p.distance_to(&Coord2(x, y));

                assert!((d-(r-(w/2.0))).abs() < 0.1 || (d-(r+(w/2.0))).abs() < 0.1, "d={} ({} or {})", d, r-(w/2.0), r+(w/2.0));
            }
        }
    }

    #[test]
    fn circle2b() {
        // As for circle2a but with Miter joins
        let w = 11.449928283691406;
        let r = 20.0;
        let x = 172.17343139648438;
        let y = 215.4249267578125;

        // Create a circle path
        let circle = Circle::new(Coord2(x, y), r);
        let circle = circle.to_path::<SimpleBezierPath>();

        // Stroke the path to generate the circle
        let thick_path = stroke_path::<SimpleBezierPath, _>(&circle, w, 
            &StrokeOptions::default()
                .with_accuracy(0.002)
                .with_min_sample_distance(0.001)
                .with_start_cap(LineCap::Butt)
                .with_end_cap(LineCap::Butt)
                .with_join(LineJoin::Miter));

        // Every point in the path must be w/2 away the point in the center
        for curve in thick_path.iter().flat_map(|section| section.to_curves::<Curve<Coord2>>()) {
            // Don't test the end lines
            if curve.characteristics() == CurveCategory::Linear {
                continue;
            }

            println!("{:?}", curve);

            // Check points on the curve
            for t in 0..100 {
                let t = (t as f64)/100.0;
                let p = curve.point_at_pos(t);

                let d = p.distance_to(&Coord2(x, y));

                assert!((d-(r-(w/2.0))).abs() < 0.1 || (d-(r+(w/2.0))).abs() < 0.1, "d={} ({} or {})", d, r-(w/2.0), r+(w/2.0));
            }
        }
    }
}
