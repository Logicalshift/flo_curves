use super::path_contour::*;
use super::sampled_approx_distance_field_cache::*;
use crate::geo::*;
use crate::bezier::*;
use crate::bezier::path::*;
use crate::bezier::vectorize::*;

use std::rc::{Rc};
use std::cell::{RefCell};

///
/// Approximates a distance field generated from a path
///
pub struct PathDistanceField {
    path_contour:           Rc<PathContour>,
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
        let path_clone  = path.clone();
        let points      = path_clone.iter()
            .flat_map(|subpath| {
                subpath.to_curves::<Curve<_>>()
                    .into_iter()
                    .flat_map(|curve| {
                        walk_curve_evenly_map(curve, 1.0, 0.1, |section| section.point_at_pos(1.0))
                    })
            });

        // The path contour can be used both as the actual path contour and as a way to determine if a point is inside the path
        let path_contour = PathContour::from_path(path, size);
        let path_contour = Rc::new(path_contour);

        // Also need a 'point is inside' function (here just a basic 'count crossings' function)
        let inside_contour  = path_contour.clone();
        let point_is_inside = move |x, y| {
            for range in inside_contour.intercepts_on_line(y) {
                if range.start > x {
                    return false;
                }

                if range.start <= x && range.end > x {
                    return true;
                }
            }

            return false;
        };

        // The approximate distance field uses distances to points to estimate the distance at each point (cheaper than actually calculating the nearest point on every path, but less accurate)
        let approx_distance_field = SampledApproxDistanceFieldCache::from_points(points, point_is_inside, size);
        let approx_distance_field = RefCell::new(approx_distance_field);

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
