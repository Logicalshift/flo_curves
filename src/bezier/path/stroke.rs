use super::path::*;

use crate::geo::*;
use crate::bezier::*;
use crate::line::*;

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

    /// The minimum difference in tangent between two samples used for fitting the resulting path
    min_tangent_difference: f64,

    /// The minimum distance between samples, when the minimum tangent is not reached
    min_sample_distance: f64,

    /// How two lines should be joined together
    join: LineJoin,

    /// How to start the line
    start_cap: LineCap,

    /// How to finish the line
    end_cap: LineCap,
}

impl Default for StrokeOptions {
    #[inline]
    fn default() -> Self {
        StrokeOptions {
            accuracy:               0.1,
            min_tangent_difference: 0.1,
            min_sample_distance:    0.1,
            join:                   LineJoin::Bevel,
            start_cap:              LineCap::Butt,
            end_cap:                LineCap::Butt,
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
    /// Sets the minimum difference in tangents between two samples
    ///
    /// Value is in radians. Lower values result in more accurate curves but generate more samples (so are slower)
    ///
    #[inline]
    pub fn with_min_tangent(mut self, min_tangent: f64) -> Self {
        self.min_tangent_difference = min_tangent;
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
}

///
/// Generates the edges for a single curve
///
fn stroke_edge<TCoord>(start_point: &mut Option<TCoord>, points: &mut Vec<(TCoord, TCoord, TCoord)>, curve: &Curve<TCoord>, subdivision_options: &SubdivisionOffsetOptions, width: f64, join: &impl Fn(TCoord, TCoord) -> (TCoord, (TCoord, TCoord), TCoord)) -> bool
where
    TCoord: Coordinate + Coordinate2D,
{
    let mut added_points = false;

    // Offset this curve using the subdivision algorithm
    if let Some(offset_curve) = offset_lms_subdivisions(curve, |_| width, |_| 0.0, &subdivision_options) {
        let initial_point = offset_curve[0].start_point();

        if let Some(start_point) = start_point {
            // Add a join to the existing curve using the join style
            let last_point = points.last().map(|(_, _, ep)| *ep).unwrap_or(*start_point);

            // TODO: support other join styles
            let (_, (cp1, cp2), ep) = join(last_point, initial_point);
            points.push((cp1, cp2, ep));

            added_points = true;
        } else {
            // Start a new curve
            *start_point = Some(initial_point);
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
/// Generates a thickened line along a path
///
/// The width describes how wide to make the resulting line. 
///
pub fn stroke_path<TPathFactory, TCoord>(path: impl BezierPath<Point=TCoord>, width: f64, options: &StrokeOptions) -> Option<TPathFactory>
where
    TPathFactory:   BezierPathFactory<Point=TCoord>,
    TCoord:         Coordinate + Coordinate2D,
{
    // Create the list of points that make up the path
    let mut start_point = None;
    let mut points      = vec![];

    // Convert the path to curves
    let path_curves = path.to_curves::<Curve<TCoord>>();

    // Create subdivision options, using the width of the curve as a guide
    let subdivision_options = SubdivisionOffsetOptions::default()
        .with_min_distance(options.min_sample_distance)
        .with_min_tangent(options.min_tangent_difference)
        .with_max_error(options.accuracy)
        .with_max_distance(width * 20.0);

    // Draw forward
    for curve in path_curves.iter() {
        // Offset this curve using the subdivision algorithm
        stroke_edge(&mut start_point, &mut points, &curve, &subdivision_options, width, &|start_point, end_point| {
            // TODO: support other join types
            line_to_bezier::<Curve<_>>(&(start_point, end_point)).all_points()
        });
    }

    // Draw backwards
    let mut added_end_cap = false;
    for curve in path_curves.iter().rev() {
        if !added_end_cap {
            added_end_cap = stroke_edge(&mut start_point, &mut points, &curve, &subdivision_options, -width, &|start_point, end_point| {
                // TODO: support end cap types
                line_to_bezier::<Curve<_>>(&(start_point, end_point)).all_points()
            });
        } else {
            stroke_edge(&mut start_point, &mut points, &curve, &subdivision_options, -width, &|start_point, end_point| {
                // TODO: support other join types
                line_to_bezier::<Curve<_>>(&(start_point, end_point)).all_points()
            });
        }
    }

    // Add start cap
    // TODO: support other cap types
    if let (Some(start_point), Some(end_point)) = (start_point, points.last().map(|(_, _, p)| p).copied()) {
        let (_, (cp1, cp2), ep) = line_to_bezier::<Curve<_>>(&(end_point, start_point)).all_points();
        points.push((cp1, cp2, ep));
    }

    // Result is the path if we generated at least 2 points
    if let Some(start_point) = start_point {
        if points.len() > 0 {
            Some(TPathFactory::from_points(start_point, points.into_iter()))
        } else {
            // Only generated one point
            None
        }
    } else {
        // Never generated a curve
        None
    }
}
