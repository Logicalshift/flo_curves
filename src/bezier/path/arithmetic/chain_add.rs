use super::super::super::super::geo::{Coordinate, Coordinate2D};
use super::super::graph_path::GraphPath;
use super::super::path::{BezierPath, BezierPathFactory};
use super::ray_cast::{PathDirection, PathLabel};

///
/// Adds multiple paths in a single operation
///
pub fn path_add_chain<P: BezierPath, POut: BezierPathFactory>(
    paths: &[Vec<P>],
    accuracy: f64,
) -> Vec<POut>
where
    P::Point: Coordinate + Coordinate2D,
    POut: BezierPathFactory<Point = P::Point>,
{
    // Build up the graph path from the supplied list
    let mut merged_path = GraphPath::new();

    for (path_idx, path) in paths.iter().enumerate() {
        let path_idx = path_idx as u32;
        merged_path = merged_path.collide(
            GraphPath::from_merged_paths(
                path.iter()
                    .map(|path| (path, PathLabel(path_idx, PathDirection::from(path)))),
            ),
            accuracy,
        );
    }

    merged_path.round(accuracy);

    // Set the exterior edges using the 'add' algorithm (all edges are considered 'external' here)
    merged_path.set_edge_kinds_by_ray_casting(|path_crossings| {
        for count in path_crossings.iter() {
            if (count & 1) != 0 {
                return true;
            }
        }
        false
    });
    merged_path.heal_exterior_gaps();

    // Produce the final result
    merged_path.exterior_paths()
}
