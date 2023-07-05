use super::sampled_contour::*;
use super::distance_field::*;

use crate::geo::*;
use crate::bezier::*;
use crate::bezier::path::*;

use smallvec::*;

use std::collections::{HashMap};

///
/// Describes a connected set of contour edges
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ContourEdgePair(ContourEdge, ContourEdge);

///
/// Describes the edges that can be connected by a cell in a contour
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ConnectedEdges {
    None,
    One(ContourEdgePair),
    Two(ContourEdgePair, ContourEdgePair),
}

impl ContourCell {
    ///
    /// Returns the edges connected by this cell (as a ContourEdge for the coordinates (0,0) to (1,1))
    ///
    #[inline]
    const fn connected_edges(&self) -> ConnectedEdges {
        match self.0 {
            0 | 15 => { ConnectedEdges::None }

            /* *-o
             * |/|
             * o-o */
            1  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::left(), ContourEdge::top())) }

            /* o-*
             * |\|
             * o-o */
            2  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::top(), ContourEdge::right())) }

            /* *-*
             * |-|
             * o-o */
            3  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::left(), ContourEdge::right())) }

            /* o-o
             * |\|
             * *-o */
            4  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::left(), ContourEdge::bottom())) }

            /* *-o
             * |||
             * *-o */
            5  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::top(), ContourEdge::bottom())) }

            /* o--*
             * |\\|
             * *--o */
            6  => { ConnectedEdges::Two(ContourEdgePair(ContourEdge::left(), ContourEdge::bottom()), ContourEdgePair(ContourEdge::top(), ContourEdge::right())) }

            /* *-*
             * |/|
             * *-o */
            7  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::bottom(), ContourEdge::right())) }

            /* o-o
             * |/|
             * o-* */
            8  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::bottom(), ContourEdge::right())) }

            /* *--o
             * |//|
             * o--* */
            9  => { ConnectedEdges::Two(ContourEdgePair(ContourEdge::left(), ContourEdge::top()), ContourEdgePair(ContourEdge::bottom(), ContourEdge::right())) }

            /* o-*
             * |||
             * o-* */
            10 => { ConnectedEdges::One(ContourEdgePair(ContourEdge::top(), ContourEdge::bottom())) }

            /* *-*
             * |\|
             * o-* */
            11 => { ConnectedEdges::One(ContourEdgePair(ContourEdge::left(), ContourEdge::bottom())) }

            /* o-o
             * |-|
             * *-* */
            12 => { ConnectedEdges::One(ContourEdgePair(ContourEdge::left(), ContourEdge::right())) }

            /* *-o
             * |\|
             * *-* */
            13 => { ConnectedEdges::One(ContourEdgePair(ContourEdge::top(), ContourEdge::right())) }

            /* o-*
             * |/|
             * *-* */
            14 => { ConnectedEdges::One(ContourEdgePair(ContourEdge::left(), ContourEdge::top())) }

            // Other values should not be valid
            _  => { unreachable!() }
        }
    }
}

///
/// Uses the marching squares algorithm to trace the edges represented by a sampled contour
///
pub fn trace_contours_from_samples(contours: &impl SampledContour) -> Vec<Vec<ContourEdge>> {
    // Hash map indicating which edges are connected to each other
    let mut edge_graph  = HashMap::<_, SmallVec<[_; 2]>>::new();
    let contour_size    = contours.contour_size();

    // Create the graph of connected edges
    for (pos, cell) in contours.edge_cell_iterator() {
        match cell.connected_edges() {
            ConnectedEdges::None => { }

            ConnectedEdges::One(ContourEdgePair(a, b)) => {
                let a = a.at_coordinates(contour_size, pos);
                let b = b.at_coordinates(contour_size, pos);

                edge_graph.entry(a).or_insert_with(|| smallvec![]).push(b);
                edge_graph.entry(b).or_insert_with(|| smallvec![]).push(a);
            }

            ConnectedEdges::Two(ContourEdgePair(a, b), ContourEdgePair(c, d)) => {
                let a = a.at_coordinates(contour_size, pos);
                let b = b.at_coordinates(contour_size, pos);
                let c = c.at_coordinates(contour_size, pos);
                let d = d.at_coordinates(contour_size, pos);

                edge_graph.entry(a).or_insert_with(|| smallvec![]).push(b);
                edge_graph.entry(b).or_insert_with(|| smallvec![]).push(a);

                edge_graph.entry(c).or_insert_with(|| smallvec![]).push(d);
                edge_graph.entry(d).or_insert_with(|| smallvec![]).push(c);
            }
        }
    }

    // Graph contains a set of non-intersecting loops: process them into sets of points
    let mut result = vec![];

    loop {
        // We can fetch any edge from the hash table to follow it around in a loop
        let (first_edge, next_edge) = if let Some((initial_edge, following_edges)) = edge_graph.iter().next() {
            (*initial_edge, following_edges[0])
        } else {
            break;
        };

        // Follow the loop to generate the points
        let mut edge_loop       = vec![first_edge];
        let mut previous_edge   = first_edge;
        let mut current_edge    = next_edge;

        // We remove the edges from the graph as we process them so we don't read them again
        edge_graph.remove(&first_edge);

        while current_edge != first_edge {
            // Add this edge to the loop
            edge_loop.push(current_edge);

            // Fetch the following edge (assumes we always generate full loops)
            let following = edge_graph.remove(&current_edge).unwrap();
            let next_edge = if following[0] != previous_edge {
                following[0]
            } else {
                following[1]
            };

            // Move on to the next item
            previous_edge   = current_edge;
            current_edge    = next_edge;
        }

        // Copy the first edge to the last edge
        edge_loop.push(edge_loop[0]);

        // Store the loop in the result
        result.push(edge_loop);
    }

    // Result is the final graph
    result
}

///
/// Creates a bezier path from a sampled set of contours
///
/// All samples are placed at the middle of the edge, so a fit accuracy > 1.0 should be used to smooth out the shape (1.5 is a good value).
/// A distance field can be used to get sub-pixel accuracy (see `trace_contours_from_distance_field()`) if that's needed.
///
pub fn trace_paths_from_samples<TPathFactory>(contours: &impl SampledContour, accuracy: f64) -> Vec<TPathFactory>
where
    TPathFactory:           BezierPathFactory,
    TPathFactory::Point:    Coordinate + Coordinate2D,
{
    // Trace out the contours
    let contour_size    = contours.contour_size();
    let contours        = trace_contours_from_samples(contours);

    // Convert the edges into points, then fit curves against the points (using low accuracy)
    contours.into_iter()
        .map(|edges| edges.into_iter().map(|edge| edge.to_coords(contour_size)).collect::<Vec<_>>())
        .filter_map(|points| {
            let curves = fit_curve_loop::<Curve<TPathFactory::Point>>(&points, accuracy)?;
            Some(TPathFactory::from_points(curves[0].start_point(), curves.into_iter().map(|curve| {
                let (cp1, cp2)  = curve.control_points();
                let end_point   = curve.end_point();

                (cp1, cp2, end_point)
            })))
        })
        .collect()
}

///
/// Traces contours from a distance field using the marching squares algorithm
///
pub fn trace_contours_from_distance_field<TCoord>(distance_field: &impl SampledSignedDistanceField) -> Vec<Vec<TCoord>> 
where
    TCoord: Coordinate + Coordinate2D,
{
    // Trace the edges
    let field_size  = distance_field.field_size();
    let contours    = distance_field.as_contour();
    let loops       = trace_contours_from_samples(contours);

    #[inline]
    fn edge_coord_to_field_coord(pos: ContourPosition) -> ContourPosition {
        ContourPosition(pos.0-1, pos.1-1)
    }

    // Every edge will have a point that can be considered as having '0' distance, which we can find by linear interpolation
    loops.into_iter()
        .map(|edge_loop| {
            edge_loop.into_iter()
                .map(|edge| {
                    // Read the from/to coordinates of this edge
                    let (from, to)      = edge.to_contour_coords(field_size);

                    // Read the distances at the edge points (edges count from 1)
                    let from_distance   = if from.0 > 0 && from.1 > 0   { distance_field.distance_at_point(edge_coord_to_field_coord(from)) } else { f64::MAX };
                    let to_distance     = if to.0 > 0 && to.1 > 0       { distance_field.distance_at_point(edge_coord_to_field_coord(to)) } else { f64::MAX };

                    // Interpolate to find the '0' coordinate
                    let zero_point      = if from_distance != to_distance {
                        from_distance / (from_distance - to_distance)
                    } else {
                        0.5
                    };

                    // If the zero point is calculated correctly it should be between 0 and 1
                    // Rounding errors at the very edge of things might push this beyond 1 however, so we allow the value to get as high as 2 here
                    debug_assert!(zero_point >= -2.0 && zero_point <= 2.0, "Zero point out of range, {:?} {:?} {:?} {:?} {:?}", zero_point, from_distance, to_distance, from, to);

                    let x = ((to.0 as f64) - (from.0 as f64)) * zero_point + (from.0 as f64);
                    let y = ((to.1 as f64) - (from.1 as f64)) * zero_point + (from.1 as f64);

                    TCoord::from_components(&[x, y])
                }).collect()
        }).collect()
}

///
/// Creates a bezier path from a sampled set of contours
///
pub fn trace_paths_from_distance_field<TPathFactory>(distance_field: &impl SampledSignedDistanceField, accuracy: f64) -> Vec<TPathFactory>
where
    TPathFactory:           BezierPathFactory,
    TPathFactory::Point:    Coordinate + Coordinate2D,
{
    // Trace out the contours
    let contours    = trace_contours_from_distance_field(distance_field);

    // Convert the edges into points, then fit curves against the points (using low accuracy)
    contours.into_iter()
        .filter_map(|points| {
            let curves = fit_curve_loop::<Curve<TPathFactory::Point>>(&points, accuracy)?;
            Some(TPathFactory::from_points(curves[0].start_point(), curves.into_iter().map(|curve| {
                let (cp1, cp2)  = curve.control_points();
                let end_point   = curve.end_point();

                (cp1, cp2, end_point)
            })))
        })
        .collect()
}
