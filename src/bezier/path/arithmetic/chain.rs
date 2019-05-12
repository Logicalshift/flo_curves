use super::arithmetic::*;
use super::super::path::*;
use super::super::graph_path::*;
use super::super::super::super::geo::*;

///
/// Description of an arithmetic operation to perform on a bezier path
///
#[derive(Clone, Debug)]
pub enum PathCombine<P: BezierPath>
where P::Point : Coordinate+Coordinate2D {
    /// Sets the result to a particular path
    Path(Vec<P>),

    /// Sets the result to a path with its interior points removed
    RemoveInteriorPoints(Vec<P>),

    /// Adds a series of paths
    Add(Vec<PathCombine<P>>),

    /// Subtracts a series of paths (from the first path)
    Subtract(Vec<PathCombine<P>>),

    /// Intersects a series a paths (with the first path)
    Intersect(Vec<PathCombine<P>>)
}

///
/// Performs a series of path combining operations to generate an output path
///
pub fn path_combine<Point, P1: BezierPath<Point=Point>, POut: BezierPathFactory<Point=Point>>(operation: PathCombine<P1>, accuracy: f64) -> Vec<POut>
where Point: Coordinate+Coordinate2D {
    // Perform the combination to build a graph pah
    let combined = GraphPath::combine(operation, accuracy);

    // Convert to the final result
    combined.exterior_paths()
}

impl<Point> GraphPath<Point, PathLabel>
where   Point: Coordinate+Coordinate2D {
    ///
    /// Performs a path combining operation on this path, setting the exterior edges of the graph accordingly
    ///
    pub fn combine<P>(operation: PathCombine<P>, accuracy: f64) -> GraphPath<Point, PathLabel> 
    where P: BezierPath<Point=Point> {
        use self::PathCombine::*;

        match operation {
            Path(paths)                 => {
                GraphPath::from_merged_paths(paths.iter()
                    .map(|path| (path, PathLabel(1, PathDirection::from(path)))))
            },

            RemoveInteriorPoints(paths) => {
                // Create the graph path from the source side
                let mut merged_path = GraphPath::from_merged_paths(paths.iter()
                    .map(|path| (path, PathLabel(0, PathDirection::from(path)))));

                // Collide the path with itself to find the intersections
                merged_path.self_collide(accuracy);

                // Set the exterior edges using the 'add' algorithm
                merged_path.set_exterior_by_removing_interior_points();
                merged_path.heal_exterior_gaps();

                merged_path
            },

            Add(ops)                => {
                // The first graph path is the result of the first operation
                let mut ops     = ops.into_iter();
                let mut result  = GraphPath::combine(ops.next().unwrap(), accuracy);

                loop {
                    if let Some(to_add) = ops.next() {
                        // Prepare to add the next path
                        result.prepare_for_next_operation(0);

                        // Generate the path to add in
                        let mut to_add = GraphPath::combine(to_add, accuracy);
                        to_add.prepare_for_next_operation(1);

                        // Add in the next path to build up the result
                        result = result.collide(to_add, accuracy);
                        result.set_exterior_by_adding();
                    } else {
                        // Finished
                        break;
                    }
                }

                result.heal_exterior_gaps();
                result
            },

            Subtract(ops)           => {
                // The first graph path is the result of the first operation
                let mut ops     = ops.into_iter();
                let mut result  = GraphPath::combine(ops.next().unwrap(), accuracy);

                loop {
                    if let Some(to_subtract) = ops.next() {
                        // Prepare to subtract the next path
                        result.prepare_for_next_operation(0);

                        // Generate the path to subtract
                        let mut to_subtract = GraphPath::combine(to_subtract, accuracy);
                        to_subtract.prepare_for_next_operation(1);

                        // Subtract the next path to build up the result
                        result = result.collide(to_subtract, accuracy);
                        result.set_exterior_by_subtracting();
                    } else {
                        // Finished
                        break;
                    }
                }

                result.heal_exterior_gaps();
                result
            },

            Intersect(ops)          => {
                // The first graph path is the result of the first operation
                let mut ops     = ops.into_iter();
                let mut result  = GraphPath::combine(ops.next().unwrap(), accuracy);

                loop {
                    if let Some(to_intersect) = ops.next() {
                        // Prepare to intersect the next path
                        result.prepare_for_next_operation(0);

                        // Generate the path to intersect
                        let mut to_intersect = GraphPath::combine(to_intersect, accuracy);
                        to_intersect.prepare_for_next_operation(1);

                        // Intersect the next path to build up the result
                        result = result.collide(to_intersect, accuracy);
                        result.set_exterior_by_intersecting();
                    } else {
                        // Finished
                        break;
                    }
                }

                result
            }
        }
    }

    ///
    /// Prepares an existing graph path for the next operation on it, by setting all labels to Path1 and removing any interior points
    ///
    fn prepare_for_next_operation(&mut self, source: u32) {
        // Remove any interior edges from this path
        self.heal_exterior_gaps();
        self.remove_interior_edges();

        // Set all edges to the specified source and as uncategorized
        self.update_all_edge_labels(|PathLabel(_old_source, direction), _kind| (PathLabel(source, direction), GraphPathEdgeKind::Uncategorised));
    }
}
