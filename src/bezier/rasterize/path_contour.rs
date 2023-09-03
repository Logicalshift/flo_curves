use super::ray_cast_contour::*;
use crate::bezier::*;
use crate::bezier::path::*;
use crate::bezier::vectorize::*;

use itertools::*;
use smallvec::*;

use std::ops::{Range};

///
/// Provides an implementation of the `SampledContour` interface for a bezier path
///
pub struct PathContour {
    /// The size of this contour
    size: ContourSize,

    /// The curves in the path
    curves: Vec<(Curve<f64>, Curve<f64>, Bounds<Coord2>)>,
}

impl PathContour {
    ///
    /// Creates a new path contour, which will produce a scan-converted contour for the specified path. The path will
    /// be processed with an even-odd winding rule. 
    ///
    pub fn from_path<TPath>(path: Vec<TPath>, size: ContourSize) -> Self
    where
        TPath:          'static + BezierPath,
        TPath::Point:   Coordinate + Coordinate2D,
    {
        // Convert the path to individual curves
        let curves = path.iter()
            .flat_map(|path| path.to_curves::<Curve<_>>())
            .map(|curve| {
                let Bounds(min, max)        = curve.bounding_box::<Bounds<_>>();
                let bounds                  = Bounds(Coord2::from_coordinate(min), Coord2::from_coordinate(max));
                let (sp, (cp1, cp2), ep)    = curve.all_points();

                let curve_x = Curve::from_points(sp.x(), (cp1.x(), cp2.x()), ep.x());
                let curve_y = Curve::from_points(sp.y(), (cp1.y(), cp2.y()), ep.y());
                (curve_x, curve_y, bounds)
            })
            .collect::<Vec<_>>();

        PathContour { 
            curves, size,
        }
    }

    ///
    /// Creates a contour that has the specified path at the center
    ///
    /// The coordinate returned is the offset of the resulting contour (add to the coordinates to get the coordinates on the original path)
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

        // The size of the contour is the size of the path with a 2px border
        let width   = size.x().ceil() + 2.0;
        let height  = size.y().ceil() + 2.0;
        let size    = ContourSize(width as _, height as _);

        // Create the contour
        let contour = Self::from_path(path, size);

        (contour, offset)
    }
}

impl SampledContour for PathContour {
    #[inline]
    fn contour_size(&self) -> ContourSize {
        self.size
    }

    #[inline]
    fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        raycast_intercepts_on_line(&|y| {
            // Iterate through all of the curves to find the intercepts
            let mut intercepts = vec![];

            let only_one_curve = self.curves.len() == 1;

            for (curve_x, curve_y, bounding_box) in self.curves.iter() {
                // Skip curves if they don't intercept this contour
                if y < bounding_box.min().y() || y > bounding_box.max().y() { continue; }

                // Solve the intercepts on the y axis
                let (w1, (w2, w3), w4)  = curve_y.all_points();
                let curve_intercepts    = solve_basis_for_t(w1, w2, w3, w4, y);

                // Add the intercepts to the list that we've been generating (we ignore t=0 as there should be a corresponding intercept at t=1 on the previous curve)
                // If there's only one curve forming a closed shape, then this isn't true (the 'following' curve is the same curve)
                intercepts.extend(curve_intercepts.into_iter().filter(|t| *t > 0.0 || only_one_curve).map(|t| curve_x.point_at_pos(t)));
            }

            // Order the intercepts to generate ranges
            intercepts.sort_unstable_by(|a, b| a.total_cmp(b));

            // Each tuple represents a range that is within the shape
            return intercepts.into_iter()
                .tuples()
                .map(|(start, end)| start..end)
                .collect();
        }, y, 1.0, self.size.width())
    }
}

impl ColumnSampledContour for PathContour {
    #[inline]
    fn intercepts_on_column(&self, x: f64) -> SmallVec<[Range<f64>; 4]> {
        raycast_intercepts_on_line(&|x| {
            // Iterate through all of the curves to find the intercepts
            let mut intercepts = vec![];

            for (curve_x, curve_y, bounding_box) in self.curves.iter() {
                // Skip curves if they don't intercept this contour
                if x < bounding_box.min().x() || x > bounding_box.max().x() { continue; }

                // Solve the intercepts on the x axis
                let (w1, (w2, w3), w4)  = curve_x.all_points();
                let curve_intercepts    = solve_basis_for_t(w1, w2, w3, w4, x);

                // Add the intercepts to the list that we've been generating (we ignore t=0 as there should be a corresponding intercept at t=1 on the previous curve)
                intercepts.extend(curve_intercepts.into_iter().filter(|t| *t > 0.0).map(|t| curve_y.point_at_pos(t)));
            }

            // Order the intercepts to generate ranges
            intercepts.sort_unstable_by(|a, b| a.total_cmp(b));

            // Each tuple represents a range that is within the shape
            return intercepts.into_iter()
                .tuples()
                .map(|(start, end)| start..end)
                .collect();
        }, x, 1.0, self.size.height())
    }
}