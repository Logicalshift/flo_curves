use super::path_contour::*;
use super::sampled_approx_distance_field_cache::*;
use crate::geo::*;
use crate::bezier::*;
use crate::bezier::path::*;
use crate::bezier::vectorize::*;

use smallvec::*;

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
            .flat_map(|(curve_idx, curve)| walk_curve_evenly_map(*curve, 0.5, 0.1, move |section| (curve_idx, section.point_at_pos(1.0), section.t_for_t(1.0))))
            .flat_map(|(curve_idx, point_guess, t_guess)| refine_nearby_points(&curves[curve_idx], point_guess, t_guess));

        // The path contour can be used both as the actual path contour and as a way to determine if a point is inside the path
        let path_contour = PathContour::from_path(path, size);
        let path_contour = Rc::new(path_contour);

        // The approximate distance field uses distances to points to estimate the distance at each point (cheaper than actually calculating the nearest point on every path, but less accurate)
        let approx_distance_field = SampledApproxDistanceFieldCache::from_points(points, (0..size.height()).map(|y| path_contour.intercepts_on_line(y as _)), size);
        let approx_distance_field = RefCell::new(approx_distance_field);

        PathDistanceField { path_contour, approx_distance_field }
    }

    ///
    /// Creates a distance field that has the specified path at the center
    ///
    /// The coordinate returned is the offset of the resulting distance field (add to the coordinates to get the coordinates on the original path)
    ///
    pub fn center_path<TPath>(path: Vec<TPath>, border: usize) -> (Self, TPath::Point) 
    where
        TPath:          'static + BezierPath + BezierPathFactory,
        TPath::Point:   Coordinate + Coordinate2D,
    {
        // Figure out the bounding box of the path
        let bounds = path.iter()
            .map(|subpath| subpath.bounding_box::<Bounds<_>>())
            .reduce(|a, b| a.union_bounds(b))
            .unwrap_or_else(|| Bounds::empty());

        // Offset is the lower-left corner of the bounding box
        let border  = TPath::Point::from_components(&[border as f64, border as f64]);
        let offset  = bounds.min() - border;
        let size    = bounds.max() - bounds.min();
        let size    = size + (border * 2.0);

        // Allow a 1px border around the path
        let offset  = offset - TPath::Point::from_components(&[1.0, 1.0]);

        // Move the path so that its lower bound is at 1,1
        let mut path = path;
        path.iter_mut().for_each(|subpath| {
            let new_subpath = subpath.map_points(|p| p - offset);
            *subpath        = new_subpath;
        });

        // The size of the distance field is the size of the path with a 2px border
        let width   = size.x().ceil() + 2.0;
        let height  = size.y().ceil() + 2.0;
        let size    = ContourSize(width as _, height as _);

        // Create the distance field
        let distance_field = Self::from_path(path, size);

        (distance_field, offset)
    }
}

///
/// Takes a known point on the curve and refines both it and the surrounding points
///
fn refine_nearby_points<TPoint>(curve: &Curve<TPoint>, point_guess: TPoint, t_guess: f64) -> SmallVec<[(ContourPosition, TPoint); 9]> 
where
    TPoint: Coordinate + Coordinate2D,
{
    // Refine the central point to start with
    let mut points                  = smallvec![];
    let (center_pos, center_point)  = if let Some(point) = refine_closest_point(curve, point_guess, t_guess) { point } else { return smallvec![] };

    points.push((center_pos, center_point));

    // Create the surrounding points
    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            if y_offset == 0 && x_offset == 0 { continue; }

            let x_offset = x_offset as f64;
            let y_offset = y_offset as f64;

            // Try to refine a guess here
            let new_point = TPoint::from_components(&[center_point.x() + x_offset, center_point.y() + y_offset]);
            if let Some(new_point) = refine_closest_point(curve, new_point, t_guess) {
                points.push(new_point);
            }
        }
    }

    points
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
        // TODO: singularities and very tight curves might produce absurd answers here (`nearest_point_on_curve_bezier_root_finder` will work in these cases)
        // TODO: at the very ends of the curve, it's possible the nearest point is on a different curve. This might not matter if both curves are inspected for the same grid point
        let refined_t = nearest_point_on_curve_newton_raphson_with_estimate(curve, &TPoint::from_components(&[grid_x, grid_y]), t_guess, 3);

        // Generate this as the nearest point
        let nearest_point = curve.point_at_pos(refined_t);

        Some((grid_position, nearest_point))
    }
}

impl SampledSignedDistanceField for PathDistanceField {
    type Contour = PathContour;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.path_contour.contour_size()
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        let distance_squared = self.approx_distance_field.borrow_mut().distance_squared_at_point(pos);

        if distance_squared < 0.0 {
            -((-distance_squared).sqrt())
        } else {
            distance_squared.sqrt()
        }
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        &self.path_contour
    }
}
