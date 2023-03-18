use super::sampled_contour::*;
use crate::geo::*;

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
pub fn trace_contours_from_samples<TCoord>(contours: impl SampledContour) /* -> impl Iterator<Item=Vec<TCoord>> */
where
    TCoord: Coordinate2D,
{
    todo!()
}
