use crate::bezier::vectorize::*;

use std::collections::{HashMap};

///
/// Produces an approximation of a distance field for a shape
///
/// This works by starting at a set of points that are 0-distance, then for any point that borders a known point
/// it will pick the closest distance, iteratively until a requested distance is found. This is good for cases
/// where a distance field that is close to a shape is desired. Accuracy is reduced and errors may appear further
/// away from the shape.
///
pub struct SampledApproxDistanceFieldCache {
    /// The size of the distance field (we stop generating at x=0, y=0 and thes bounds)
    size: ContourSize,

    /// Known points that are at 0 distance from the shape
    zero_points: Vec<(f64, f64)>,

    /// Points with distances derviced from the zero points (hashmap maps positions to a distance and an index into the zero_points structure)
    cached_points: HashMap<(usize, usize), (f64, usize)>,

    /// Points that are waiting to be calculated (these have neighbours in the cached_points structure)
    waiting_points: Vec<(usize, usize)>,
}
