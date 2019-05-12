use super::add::*;
use super::sub::*;
use super::chain_add::*;
use super::intersect::*;
use super::super::path::*;
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
pub fn path_combine<Point, P: BezierPathFactory<Point=Point>>(operation: PathCombine<P>, accuracy: f64) -> Vec<P>
where Point: Coordinate+Coordinate2D {
    // TODO: it's probably possible to combine add, subtract and intersect into a single ray-casting operation using a similar technique to how path_add_chain works

    match operation {
        PathCombine::Path(result)               => result,
        PathCombine::RemoveInteriorPoints(path) => path_remove_interior_points(&path, accuracy),
        PathCombine::Add(paths)                 => path_add_chain(&paths.into_iter().map(|path| path_combine(path, accuracy)).collect(), accuracy),

        PathCombine::Subtract(paths)            => {
            let mut path_iter   = paths.into_iter();
            let result          = path_iter.next().unwrap_or_else(|| PathCombine::Path(vec![]));
            let mut result      = path_combine(result, accuracy);

            while let Some(to_subtract) = path_iter.next() {
                let to_subtract = path_combine(to_subtract, accuracy);
                result          = path_sub(&result, &to_subtract, accuracy);
            }

            result
        }

        PathCombine::Intersect(paths)            => {
            let mut path_iter       = paths.into_iter();
            let result              = path_iter.next().unwrap_or_else(|| PathCombine::Path(vec![]));
            let mut result          = path_combine(result, accuracy);

            while let Some(to_intersect) = path_iter.next() {
                let to_intersect    = path_combine(to_intersect, accuracy);
                result              = path_intersect(&result, &to_intersect, accuracy);
            }

            result
        }
    }
}
