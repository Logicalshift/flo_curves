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

    /// The curves in the path (divided into x and y portions)
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
            .filter(|curve| !curve_is_tiny(curve))
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

/// Intermediate structure used to represent an intercept from intercepts_on_line
#[derive(Debug)]
struct ContourIntercept {
    curve_idx:  usize,
    t:          f64,
    x_pos:      f64,
}

impl PathContour {
    ///
    /// Returns true if two indices indicate neighboring curves
    ///
    #[inline]
    fn curves_are_neighbors(&self, idx1: usize, idx2: usize) -> bool {
        if idx1+1 == idx2 || idx2+1 == idx1 {
            true
        } else if idx1 == 0 && idx2 == self.curves.len()-1 {
            true
        } else if idx2 == 0 && idx1 == self.curves.len()-1 {
            true
        } else {
            false
        }
    }

    ///
    /// Returns the length of the control polygon for a bezier curve
    ///
    fn control_polygon_length(x_curve: &impl BezierCurve<Point=f64>, y_curve: &impl BezierCurve<Point=f64>) -> f64 {
        let (spx, (cp1x, cp2x), epx) = x_curve.all_points();
        let (spy, (cp1y, cp2y), epy) = y_curve.all_points();

        Coord2(spx, spy).distance_to(&Coord2(cp1x, cp1y))
            + Coord2(cp1x, cp1y).distance_to(&Coord2(cp2x, cp2y))
            + Coord2(cp2x, cp2y).distance_to(&Coord2(epx, epy))
    }

    ///
    /// Returns true if two points are on the same side of the curve, from the point of view of a horizontal ray
    ///
    fn points_are_same_side_horiz(&self, prev: &ContourIntercept, next: &ContourIntercept) -> bool {
        let prev_curve_y = &self.curves[prev.curve_idx].1;
        let next_curve_y = &self.curves[next.curve_idx].1;

        let (w1, (w2, w3), w4)  = prev_curve_y.all_points();
        let (d1, d2, d3)        = derivative4(w1, w2, w3, w4);
        let prev_tangent        = de_casteljau3(prev.t, d1, d2, d3);

        let (w1, (w2, w3), w4)  = next_curve_y.all_points();
        let (d1, d2, d3)        = derivative4(w1, w2, w3, w4);
        let next_tangent        = de_casteljau3(next.t, d1, d2, d3);

        prev_tangent.signum() == next_tangent.signum()
    }

    ///
    /// Removes any places where a ray has intercepted the path twice
    ///
    /// This is mainly at points where two path sections join, where we solve twice at two very close x-positions
    ///
    #[inline]
    fn remove_duplicate_intercepts(&self, intercepts: &mut Vec<ContourIntercept>) {
        // How close two points must be to each other to be considered the same
        const MIN_DISTANCE: f64 = 1e-6;

        let mut idx = 0;
        while idx < intercepts.len() {
            let next_idx = if idx < intercepts.len()-1 { idx + 1 } else { 0 };

            let prev = &intercepts[idx];
            let next = &intercepts[next_idx];

            let prev_curve_idx = prev.curve_idx;
            let next_curve_idx = next.curve_idx;

            if self.curves_are_neighbors(prev_curve_idx, next_curve_idx) && (prev.x_pos - next.x_pos).abs() <= MIN_DISTANCE {
                // These two curves and points are close to each other: get the two curve sections
                let section_1 = if prev.t < 0.5 { 
                    (self.curves[prev_curve_idx].0.section(0.0, prev.t), self.curves[prev_curve_idx].1.section(0.0, prev.t)) 
                } else {
                    (self.curves[prev_curve_idx].0.section(prev.t, 1.0), self.curves[prev_curve_idx].1.section(prev.t, 1.0)) 
                };
                let section_2 = if next.t < 0.5 { 
                    (self.curves[next_curve_idx].0.section(0.0, next.t), self.curves[next_curve_idx].1.section(0.0, next.t)) 
                } else {
                    (self.curves[next_curve_idx].0.section(next.t, 1.0), self.curves[next_curve_idx].1.section(next.t, 1.0)) 
                };

                // Calculate the control polygon length
                let control_polygon_length = Self::control_polygon_length(&section_1.0, &section_1.1) + Self::control_polygon_length(&section_2.0, &section_2.1);

                if control_polygon_length <= MIN_DISTANCE && self.points_are_same_side_horiz(prev, next) {
                    // This curve is very short, so remove it
                    intercepts.remove(idx);
                } else {
                    // x positions are almost the same but the curve has a long arc length, so we keep it (eg it's a loop)
                    idx += 1;
                }
            } else {
                idx += 1;
            }
        }
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

            for (idx, (curve_x, curve_y, bounding_box)) in self.curves.iter().enumerate() {
                // Skip curves if they don't intercept this contour
                if y < bounding_box.min().y() || y > bounding_box.max().y() { continue; }

                // Solve the intercepts on the y axis
                let (w1, (w2, w3), w4)  = curve_y.all_points();
                let curve_intercepts    = solve_basis_for_t(w1, w2, w3, w4, y);

                // Add the intercepts to the list that we've been generating (we ignore t=0 as there should be a corresponding intercept at t=1 on the previous curve)
                // If there's only one curve forming a closed shape, then this isn't true (the 'following' curve is the same curve)
                intercepts.extend(curve_intercepts.into_iter()
                    .filter(|t| *t > 0.0 || only_one_curve)
                        .map(|t| ContourIntercept {
                            curve_idx:  idx,
                            t:          t,
                            x_pos:      curve_x.point_at_pos(t)
                        }));
            }

            // Order the intercepts to generate ranges
            intercepts.sort_unstable_by(|a, b| a.x_pos.total_cmp(&b.x_pos));
            self.remove_duplicate_intercepts(&mut intercepts);

            debug_assert!(intercepts.len() <= 1 || intercepts.len()%2 == 0, "Found an uneven number of intercepts ({:?}, y={})", intercepts, y);

            // Each tuple represents a range that is within the shape
            return intercepts.into_iter()
                .tuples()
                .map(|(start, end)| (start.x_pos)..(end.x_pos))
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