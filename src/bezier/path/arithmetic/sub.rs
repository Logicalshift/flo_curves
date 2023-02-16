use super::ray_cast::*;
use super::super::path::*;
use super::super::graph_path::*;
use super::super::super::super::geo::*;

impl<Point: Coordinate+Coordinate2D> GraphPath<Point, PathLabel> {
    ///
    /// Given a labelled graph path, marks exterior edges by subtracting `PathSource::Path2` from `PathSource::Path1`
    ///
    pub fn set_exterior_by_subtracting(&mut self) {
        // Use an even-odd winding rule (all edges are considered 'external')
        self.set_edge_kinds_by_ray_casting(|path_crossings| (path_crossings[0]&1) != 0 && (path_crossings[1]&1) == 0);
    }
}

///
/// Generates the path formed by subtracting two sets of paths
///
/// Each of the two paths passed into this function is assumed not to overlap themselves. IE, this does not perform self-intersection 
/// on either `path1` or `path2`. This provides both a performance optimisation and finer control over how self-intersecting paths are
/// handled. See `path_remove_interior_points()` and `path_remove_overlapped_points()` for a way to eliminate overlaps.
/// 
/// The input vectors represent the external edges of the path to subtract (a single BezierPath cannot have any holes in it, so a set of them
/// effectively represents a path intended to be rendered with an even-odd winding rule)
///
pub fn path_sub<POut>(path1: &Vec<impl BezierPath<Point=POut::Point>>, path2: &Vec<impl BezierPath<Point=POut::Point>>, accuracy: f64) -> Vec<POut>
where
    POut:           BezierPathFactory,
    POut::Point:    Coordinate+Coordinate2D,
{
    // If either path is empty, short-circuit by returning the other
    if path1.is_empty() {
        return path2.iter()
            .map(|path| POut::from_path(path))
            .collect();
    } else if path2.is_empty() {
        return path1.iter()
            .map(|path| POut::from_path(path))
            .collect();
    }

    // Create the graph path from the source side
    let mut merged_path = GraphPath::new();
    merged_path         = merged_path.merge(GraphPath::from_merged_paths(path1.iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));

    // Collide with the target side to generate a full path
    merged_path         = merged_path.collide(GraphPath::from_merged_paths(path2.iter().map(|path| (path, PathLabel(1, PathDirection::from(path))))), accuracy);
    merged_path.round(accuracy);

    // Set the exterior edges using the 'subtract' algorithm
    merged_path.set_exterior_by_subtracting();
    merged_path.heal_exterior_gaps();

    // Produce the final result
    merged_path.exterior_paths()
}
