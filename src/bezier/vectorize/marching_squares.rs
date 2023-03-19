use super::sampled_contour::*;
use crate::geo::*;

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
            5  => { ConnectedEdges::One(ContourEdgePair(ContourEdge::left(), ContourEdge::top())) }

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
/// Uses the marching squares algorithm to trace the paths represented by a sampled contour
///
pub fn trace_contours_from_samples<TCoord>(contours: impl SampledContour) -> impl Iterator<Item=Vec<ContourEdge /* TCoord */>>
where
    TCoord: Coordinate2D,
{
    // Hash map indicating which edges are connected to each other
    let mut edge_graph = HashMap::<_, SmallVec<[_; 2]>>::new();

    // Create the graph of connected edges
    for (pos, cell) in contours.edge_cell_iterator() {
        match cell.connected_edges() {
            ConnectedEdges::None => { }

            ConnectedEdges::One(ContourEdgePair(a, b)) => {
                edge_graph.entry(a).or_insert_with(|| smallvec![]).push(b);
                edge_graph.entry(b).or_insert_with(|| smallvec![]).push(a);
            }

            ConnectedEdges::Two(ContourEdgePair(a, b), ContourEdgePair(c, d)) => {
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

        // Store the loop in the result
        result.push(edge_loop);
    }

    // Result is the final graph
    result.into_iter()
}
