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
