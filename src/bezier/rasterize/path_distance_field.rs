use super::path_contour::*;
use super::sampled_approx_distance_field_cache::*;
use crate::geo::*;
use crate::bezier::*;
use crate::bezier::path::*;
use crate::bezier::vectorize::*;

use std::cell::{RefCell};

///
/// Approximates a distance field generated from a path
///
pub struct PathDistanceField {
    path_contour: PathContour,
    approx_distance_field: RefCell<SampledApproxDistanceFieldCache>,
}

impl PathDistanceField {
    ///
    /// 
    ///
    pub fn from_path<TPath>(path: Vec<TPath>, size: ContourSize) -> Self
    where
        TPath:          'static + BezierPath,
        TPath::Point:   Coordinate + Coordinate2D,
    {
        // Generate the distance field cache: need to walk the perimeter of the curve to find evenly-spaced points
        let points = path.iter()
            .flat_map(|subpath| {
                subpath.to_curves::<Curve<_>>()
                    .into_iter()
                    .flat_map(|curve| {
                        walk_curve_evenly(&curve, 1.0, 0.1)
                            .skip(1)
                            .map(|section| section.point_at_pos(0.5))
                    })
            });

        // Path contour is used to compute the intercepts

        todo!()
    }
}
