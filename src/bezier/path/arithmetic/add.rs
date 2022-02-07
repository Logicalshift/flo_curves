use super::super::super::super::geo::*;
use super::super::graph_path::*;
use super::super::path::*;
use super::ray_cast::*;

//
// This uses a simple ray casting algorithm to perform the addition
//
// Basic idea is to cast a ray at an edge which is currently uncategorised, and mark the edges it crosses as interior or
// exterior depending on whether or not we consider it as crossing into or out of the final shape.
//

impl<Point: Coordinate + Coordinate2D> GraphPath<Point, PathLabel> {
    ///
    /// Given a labelled graph path, marks exterior edges by adding `PathSource::Path1` and `PathSource::Path2`
    ///
    pub fn set_exterior_by_adding(&mut self) {
        // Use an even-odd winding rule (all edges are considered 'external')
        self.set_edge_kinds_by_ray_casting(|path_crossings| {
            (path_crossings[0] & 1) != 0 || (path_crossings[1] & 1) != 0
        });
    }

    ///
    /// Given a path that intersects itself (ie, only contains SourcePath::Path1), discovers the 'true' exterior edge.
    ///
    pub fn set_exterior_by_removing_interior_points(&mut self) {
        // All points inside the path are considered 'interior' (non-zero winding rule)
        self.set_edge_kinds_by_ray_casting(|path_crossings| {
            path_crossings[0] != 0 || path_crossings[1] != 0
        });
    }
}

///
/// Generates the path formed by adding two sets of paths
///
/// The input vectors represent the external edges of the path to add (a single BezierPath cannot have any holes in it, so a set of them
/// effectively represents a path intended to be rendered with an even-odd winding rule)
///
pub fn path_add<P1: BezierPath, P2: BezierPath, POut: BezierPathFactory>(
    path1: &Vec<P1>,
    path2: &Vec<P2>,
    accuracy: f64,
) -> Vec<POut>
where
    P1::Point: Coordinate + Coordinate2D,
    P2: BezierPath<Point = P1::Point>,
    POut: BezierPathFactory<Point = P1::Point>,
{
    // If either path is empty, short-circuit by returning the other
    if path1.len() == 0 {
        return path2.iter().map(|path| POut::from_path(path)).collect();
    } else if path2.len() == 0 {
        return path1.iter().map(|path| POut::from_path(path)).collect();
    }

    // Create the graph path from the source side
    let mut merged_path = GraphPath::new();
    merged_path = merged_path.merge(GraphPath::from_merged_paths(
        path1
            .into_iter()
            .map(|path| (path, PathLabel(0, PathDirection::from(path)))),
    ));

    // Collide with the target side to generate a full path
    merged_path = merged_path.collide(
        GraphPath::from_merged_paths(
            path2
                .into_iter()
                .map(|path| (path, PathLabel(1, PathDirection::from(path)))),
        ),
        accuracy,
    );
    merged_path.round(accuracy);

    // Set the exterior edges using the 'add' algorithm
    merged_path.set_exterior_by_adding();
    merged_path.heal_exterior_gaps();

    // Produce the final result
    merged_path.exterior_paths()
}

///
/// Generates the path formed by removing any interior points from an existing path. This considers only the outermost edges of the
/// path to be the true edges, so if there are sub-paths inside an outer path, they will be removed.
///
/// This is a strict version of the 'non-zero' winding rule. It's useful for things like a path that was generated from a brush stroke
/// and might self-overlap: this can be passed a drawing of a loop made by overlapping the ends and it will output two non-overlapping
/// subpaths.
///
/// See `path_remove_overlapped_points()` for a version that considers all edges within the path to be exterior edges.
///
pub fn path_remove_interior_points<P1: BezierPath, POut: BezierPathFactory>(
    path: &Vec<P1>,
    accuracy: f64,
) -> Vec<POut>
where
    P1::Point: Coordinate + Coordinate2D,
    POut: BezierPathFactory<Point = P1::Point>,
{
    // Create the graph path from the source side
    let mut merged_path = GraphPath::new();
    merged_path = merged_path.merge(GraphPath::from_merged_paths(
        path.into_iter()
            .map(|path| (path, PathLabel(0, PathDirection::from(path)))),
    ));

    // Collide the path with itself to find the intersections
    merged_path.self_collide(accuracy);
    merged_path.round(accuracy);

    // Set the exterior edges by considering all points inside an edge as 'interior'
    merged_path.set_exterior_by_removing_interior_points();
    merged_path.heal_exterior_gaps();

    // Produce the final result
    let result = merged_path.exterior_paths();
    test_assert!(result.len() != 0 || path.len() == 0);

    result
}

///
/// Generates the path formed by removing any interior points from an existing path. This considers all edges to be exterior edges
/// and will remove those that are obscured by another part of the path.
///
/// This works like the 'even-odd' winding rule.
///
/// If a path self-intersects, this will leave a hole behind. See `path_remove_interior_points()` for a way to remove the interior
/// parts of a path so that all points inside the boundary are filled. This function is useful for cleaning up paths from other
/// sources, then the other arithmetic operations can be used to reshape the resulting path.
///
/// Note that calling 'subtract' is a more reliable way to cut a hole in an existing path than relying on a winding rule, as
/// winding rules presuppose you can tell if a subpath is inside or outside of an existing path.
///
pub fn path_remove_overlapped_points<P1: BezierPath, POut: BezierPathFactory>(
    path: &Vec<P1>,
    accuracy: f64,
) -> Vec<POut>
where
    P1::Point: Coordinate + Coordinate2D,
    POut: BezierPathFactory<Point = P1::Point>,
{
    // Create the graph path from the source side
    let mut merged_path = GraphPath::new();
    merged_path = merged_path.merge(GraphPath::from_merged_paths(
        path.into_iter()
            .map(|path| (path, PathLabel(0, PathDirection::from(path)))),
    ));

    // Collide the path with itself to find the intersections
    merged_path.self_collide(accuracy);
    merged_path.round(accuracy);

    // Set the exterior edges using the 'add' algorithm
    merged_path.set_exterior_by_adding();
    merged_path.heal_exterior_gaps();

    // Produce the final result
    let result = merged_path.exterior_paths();
    test_assert!(result.len() != 0 || path.len() == 0);

    result
}
