use crate::bezier::vectorize::*;
use crate::geo::*;

use std::collections::{HashMap};

///
/// Produces an approximation of a distance field for a shape
///
/// This uses an algorithm that assumes that the closest point to a given pixel is also the closest point of one of its neighbours.
/// This makes it possible to construct the distance field very quickly from sampled points around the perimeter of the shape but
/// produces reduced accuracy away from the edge.
///
#[derive(Clone)]
pub struct SampledApproxDistanceFieldCache {
    /// The size of the distance field (we stop generating at x=0, y=0 and thes bounds)
    size: ContourSize,

    /// Known points that are at 0 distance from the shape
    zero_points: Vec<(f64, f64)>,

    /// Points with distances derviced from the zero points (hashmap maps positions to a distance and an index into the zero_points structure)
    /// Distances are stored as 'distance squared' and are negative for points that are considered to be inside the shape.
    cached_points: HashMap<ContourPosition, (f64, usize)>,

    /// Points that are waiting to be calculated (these have neighbours in the cached_points structure)
    waiting_points: Vec<ContourPosition>,
}

impl SampledApproxDistanceFieldCache {
    ///
    /// Begins populating the distance field cache from a list of points on the perimeter of the shape and a function to determine if a point is 
    /// inside the shape or not.
    ///
    /// The samples should be around 1 pixel distant from each other: closer samples will work but many will likely not contribute to the final
    /// shape, and samples that are further apart will produce larger distortions in the distance field.
    ///
    pub fn from_points<TPoint>(perimeter_samples: impl IntoIterator<Item=TPoint>, is_inside: impl Fn(f64, f64) -> bool, size: ContourSize) -> Self 
    where
        TPoint: Coordinate2D,
    {
        let width   = size.width() as f64;
        let height  = size.height() as f64;

        // The zero points are used as reference points
        let zero_points = perimeter_samples.into_iter()
            .map(|point| (point.x(), point.y()))
            .collect::<Vec<_>>();

        // Cached points are known on the distance field, waiting points are points which have not distributed their distance to their
        // neighbours yet
        let mut cached_points   = HashMap::<ContourPosition, (f64, usize)>::new();
        let mut waiting_points  = vec![];

        for idx in 0..zero_points.len() {
            // Fetch the next point
            let (sample_x, sample_y) = zero_points[idx];

            // Measure the distances for the points around the current point
            for y_offset in -1..=1 {
                // Use the offset point
                let point_y = sample_y + (y_offset as f64);
                if point_y < 0.0 || point_y >= height {
                    continue;
                }

                for x_offset in -1..=1 {
                    // Use the offset point (ignore samples outside of the size boundary)
                    let point_x = sample_x + (x_offset as f64);
                    if point_x < 0.0 || point_x >= width {
                        continue;
                    }

                    // Sample this position, determine if it's inside or not
                    let pos         = ContourPosition(point_x as usize, point_y as usize);
                    let pos_inside  = is_inside(pos.x() as _, pos.y() as _);

                    let offset_x    = sample_x - point_x;
                    let offset_y    = sample_y - point_y;
                    let distance    = offset_x*offset_x + offset_y*offset_y;

                    // Update the cache at this point
                    if let Some((existing_distance, existing_idx)) = cached_points.get_mut(&pos) {
                        // Replace the existing point if this one is closer
                        if distance < existing_distance.abs() {
                            let distance = if pos_inside { -distance } else { distance };

                            *existing_distance  = distance;
                            *existing_idx       = idx;
                        }
                    } else {
                        // Haven't seen this point yet, so this is the closest perimeter point to it
                        let distance = if pos_inside { -distance } else { distance };
                        cached_points.insert(pos, (distance, idx));

                        // As we haven't seen this point before, add to the waiting points
                        waiting_points.push(pos);
                    }
                }
            }
        }

        SampledApproxDistanceFieldCache { size, zero_points, cached_points, waiting_points }
    }
}
