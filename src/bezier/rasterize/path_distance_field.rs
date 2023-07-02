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
        let curves      = path.iter().flat_map(|subpath| subpath.to_curves::<Curve<_>>()).collect::<Vec<_>>();
        let points      = curves.iter().enumerate()
            .flat_map(|(curve_idx, curve)| walk_curve_evenly_map(*curve, 0.1, 0.1, move |section| (curve_idx, section.point_at_pos(1.0), section.t_for_t(1.0))))
            .flat_map(|(curve_idx, point_guess, t_guess)| refine_closest_point(&curves[curve_idx], point_guess, t_guess));

        // The path contour can be used both as the actual path contour and as a way to determine if a point is inside the path
        let path_contour = PathContour::from_path(path, size);
        let path_contour = Rc::new(path_contour);

        // The approximate distance field uses distances to points to estimate the distance at each point (cheaper than actually calculating the nearest point on every path, but less accurate)
        let approx_distance_field = SampledApproxDistanceFieldCache::from_points(points, (0..size.height()).map(|y| path_contour.intercepts_on_line(y as _)), size);
        let approx_distance_field = RefCell::new(approx_distance_field);

        PathDistanceField { path_contour, approx_distance_field }
    }
}

///
/// Takes a known point on the curve, and uses it to refine a nearby point that's exactly on the grid and close to this point
///
fn refine_closest_point<TPoint>(curve: &Curve<TPoint>, point_guess: TPoint, t_guess: f64) -> Option<(ContourPosition, TPoint)>
where
    TPoint: Coordinate + Coordinate2D,
{
    // Grid position is the rounded version of the point
    let grid_x = point_guess.x().round();
    let grid_y = point_guess.y().round();

    if grid_x < 0.0 || grid_y < 0.0 {
        // Outside of the bounds of the path
        None
    } else {
        // Grid position
        let grid_position = ContourPosition(grid_x as _, grid_y as _);

        // Create a refined t value using a few rounds of newton-raphson that's near to the grid point
        let refined_t = nearest_point_on_curve_newton_raphson_with_estimate(curve, &TPoint::from_components(&[grid_x, grid_y]), t_guess, 3);

        // Generate this as the nearest point
        let nearest_point = curve.point_at_pos(refined_t);

        Some((grid_position, nearest_point))
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
        // TODO: maybe store the intercept ranges for the whole shape and use those to determine if a distance is negative or positive? The current approach does not work and 
        // I'm not sure if it's fixable: even if the approach coule be made to work, it'd break in any situation where there's a gap in the points
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
