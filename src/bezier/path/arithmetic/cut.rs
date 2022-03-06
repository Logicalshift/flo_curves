use super::ray_cast::*;
use super::super::path::*;
use super::super::graph_path::*;
use super::super::super::super::geo::*;

///
/// The result of a path cut operation
///
pub struct PathCut<P: BezierPathFactory> {
    /// The path that was inside the 'cut' path
    pub interior_path: Vec<P>,

    /// The path that was outside of the 'cut' path
    pub exterior_path: Vec<P>
}

///
/// Cuts a path (`path1`) into two along another path (`path2`), returning the part of `path1` that was interior to `path2` and
/// the part that was exterior in one operation
///
pub fn path_cut<P1: BezierPath, P2: BezierPath, POut: BezierPathFactory>(path1: &Vec<P1>, path2: &Vec<P2>, accuracy: f64) -> PathCut<POut>
where   P1::Point:  Coordinate+Coordinate2D,
        P2:         BezierPath<Point=P1::Point>,
        POut:       BezierPathFactory<Point=P1::Point> {
    // If path1 is empty, then there are no points in the result. If path2 is empty, then all points are exterior
    if path1.is_empty() {
        return PathCut { 
            interior_path: vec![], 
            exterior_path: vec![] 
        };
    } else if path2.is_empty() {
        return PathCut { 
            interior_path: vec![], 
            exterior_path: path1.iter().map(|path| POut::from_path(path)).collect() 
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
    let interior_path = merged_path.exterior_paths();

    // TODO: we can use the same raycasting operation to detect the interior and exterior points simultaneously but the current design
    // doesn't allow us to represent this in the data for the edges (this would speed up the 'cut' operation as only half the ray-casting
    // operations would be required, though note that the merge and collide operation is likely to be more expensive than this overall)

    // The exterior edges are those found by subtracting the second path from the first
    merged_path.reset_edge_kinds();
    merged_path.set_exterior_by_subtracting();
    merged_path.heal_exterior_gaps();

    // Fetch the exterior path
    let exterior_path = merged_path.exterior_paths();

    PathCut {
        interior_path,
        exterior_path
    }
}
