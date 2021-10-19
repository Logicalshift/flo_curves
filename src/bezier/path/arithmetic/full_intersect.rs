use super::ray_cast::*;
use super::super::path::*;
use super::super::graph_path::*;
use super::super::super::super::geo::*;

///
/// The result of a path cut operation
///
#[derive(Clone, Debug)]
pub struct PathIntersection<P: BezierPathFactory> {
    /// The path that was intersecting between the two paths
    pub intersecting_path: Vec<P>,

    /// The path that was outside of the 'cut' path for the two input paths
    pub exterior_paths: [Vec<P>; 2]
}

///
/// Intersects two paths, returning both the path that is the intersection and the paths that are outside
///
pub fn path_full_intersect<P1: BezierPath, P2: BezierPath, POut: BezierPathFactory>(path1: &Vec<P1>, path2: &Vec<P2>, accuracy: f64) -> PathIntersection<POut>
where   P1::Point:  Coordinate+Coordinate2D,
        P2:         BezierPath<Point=P1::Point>,
        POut:       BezierPathFactory<Point=P1::Point> {
    // If path1 is empty, then there are no points in the result. If path2 is empty, then all points are exterior
    if path1.len() == 0 {
        return PathIntersection { 
            intersecting_path:  vec![], 
            exterior_paths:     [vec![], path2.iter().map(|path| POut::from_path(path)).collect()]
        };
    } else if path2.len() == 0 {
        return PathIntersection { 
            intersecting_path:  vec![], 
            exterior_paths:     [path1.iter().map(|path| POut::from_path(path)).collect(), vec![]]
        };
    }

    // Create the graph path from the source side
    let mut merged_path = GraphPath::new();
    merged_path         = merged_path.merge(GraphPath::from_merged_paths(path1.into_iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));

    // Collide with the target side to generate a full path
    merged_path         = merged_path.collide(GraphPath::from_merged_paths(path2.into_iter().map(|path| (path, PathLabel(1, PathDirection::from(path))))), accuracy);
    merged_path.round(accuracy);

    // The interior edges are those found by intersecting the second path with the first
    merged_path.set_exterior_by_intersecting();
    merged_path.heal_exterior_gaps();

    // Fetch the interior path
    let intersecting_path = merged_path.exterior_paths();

    // TODO: we can use the same raycasting operation to detect the interior and exterior points simultaneously but the current design
    // doesn't allow us to represent this in the data for the edges (this would speed up the 'cut' operation as only half the ray-casting
    // operations would be required, though note that the merge and collide operation is likely to be more expensive than this overall)

    // The exterior edges are those found by subtracting the second path from the first
    merged_path.reset_edge_kinds();
    merged_path.set_exterior_by_subtracting();
    merged_path.heal_exterior_gaps();

    // This will be the part of path 1 that excludes path 2
    let exterior_from_path_1 = merged_path.exterior_paths();

    // Invert the subtraction operation
    // TODO: it would be faster to re-use the existing merged paths here, but this will fail to properly generate a subtracted paths
    // in the case where edges of the two paths overlap.
    let mut merged_path = GraphPath::new();
    merged_path         = merged_path.merge(GraphPath::from_merged_paths(path2.into_iter().map(|path| (path, PathLabel(0, PathDirection::from(path))))));
    merged_path         = merged_path.collide(GraphPath::from_merged_paths(path1.into_iter().map(|path| (path, PathLabel(1, PathDirection::from(path))))), accuracy);
    merged_path.round(accuracy);

    merged_path.set_exterior_by_subtracting();
    merged_path.heal_exterior_gaps();

    // This will be the part of path 2 that excludes path1
    let exterior_from_path_2 = merged_path.exterior_paths();

    PathIntersection {
        intersecting_path:  intersecting_path,
        exterior_paths:     [exterior_from_path_1, exterior_from_path_2]
    }
}
