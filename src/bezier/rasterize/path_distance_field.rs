use super::path_contour::*;
use super::sampled_approx_distance_field_cache::*;
use crate::geo::*;
use crate::bezier::*;
use crate::bezier::intersection::*;
use crate::bezier::path::*;
use crate::bezier::vectorize::*;

use std::cell::{RefCell};

///
/// Approximates a distance field generated from a path
///
pub struct PathDistanceField {
    path_contour:           PathContour,
    approx_distance_field:  RefCell<SampledApproxDistanceFieldCache>,
}

impl PathDistanceField {
    ///
    /// Creates a (approximated) distance field from a bezier path
    ///
    pub fn from_path<TPath>(path: Vec<TPath>, size: ContourSize) -> Self
    where
        TPath:          'static + BezierPath,
        TPath::Point:   Coordinate + Coordinate2D,
    {
        // Generate the distance field cache: need to walk the perimeter of the curve to find evenly-spaced points
        let offset = TPath::Point::from_components(&[1.0, 1.0]);
        let points = path.iter()
            .flat_map(|subpath| {
                subpath.to_curves::<Curve<_>>()
                    .into_iter()
                    .flat_map(|curve| {
                        walk_curve_evenly_map(curve, 1.0, 0.1, |section| section.point_at_pos(1.0) + offset)
                    })
            });

        // Also need a 'point is inside' function (here just a basic 'count crossings' function)
        let curves = path.iter().flat_map(|subpath| subpath.to_curves::<Curve<_>>()).collect::<Vec<_>>();
        let point_is_inside = move |x, y| {
            let p1  = TPath::Point::from_components(&[x, y]);
            let p2  = TPath::Point::from_components(&[x-1.0, y]);
            let ray = (p1, p2);

            // Count crossings on the negative side of the line, and not at the t=0 end of the curve (as those will match a t=1 collision)
            let crossings = curves.iter()
                .flat_map(|curve| curve_intersects_ray(curve, &ray))
                .filter(|(curve_t, line_t, _)| *curve_t > 0.0 && *line_t < 0.0)
                .count();

            // Even crossings are outside, odd crossings are inside
            (crossings%2) == 0
        };

        let approx_distance_field = SampledApproxDistanceFieldCache::from_points(points, point_is_inside, size);
        let approx_distance_field = RefCell::new(approx_distance_field);

        // Path contour is used to compute the intercepts
        let path_contour = PathContour::from_path(path, size);

        PathDistanceField { path_contour, approx_distance_field }
    }
}

impl<'a> SampledSignedDistanceField for &'a PathDistanceField {
    type Contour = &'a PathContour;

    #[inline]
    fn field_size(self) -> ContourSize {
        self.path_contour.contour_size()
    }

    #[inline]
    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        let distance_squared = self.approx_distance_field.borrow_mut().distance_squared_at_point(pos);

        if distance_squared < 0.0 {
            -((-distance_squared).sqrt())
        } else {
            distance_squared.sqrt()
        }
    }

    #[inline]
    fn as_contour(self) -> Self::Contour {
        &self.path_contour
    }
}
