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
    contour: DynRayCastContour
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
                let bounds                  = curve.bounding_box::<Bounds<_>>();
                let (sp, (cp1, cp2), ep)    = curve.all_points();

                let curve_x = Curve::from_points(sp.x(), (cp1.x(), cp2.x()), ep.x());
                let curve_y = Curve::from_points(sp.y(), (cp1.y(), cp2.y()), ep.y());
                (curve_x, curve_y, bounds)
            })
            .collect::<Vec<_>>();

        // Generate a ray-cast con
        let contour = DynRayCastContour::new(Box::new(move |y| {
                // Iterate through all of the curves to find the intercepts
                let mut intercepts = vec![];

                for (curve_x, curve_y, bounding_box) in curves.iter() {
                    // Skip curves if they don't intercept this contour
                    if y < bounding_box.min().y() || y > bounding_box.max().y() { continue; }

                    // Solve the intercepts on the y axis
                    let (w1, (w2, w3), w4)  = curve_y.all_points();
                    let curve_intercepts    = solve_basis_for_t(w1, w2, w3, w4, y);

                    // Add the intercepts to the list that we've been generating (we ignore t=0 as there should be a corresponding intercept at t=1 on the previous curve)
                    intercepts.extend(curve_intercepts.into_iter().filter(|t| *t > 0.0).map(|t| curve_x.point_at_pos(t)));
                }

                // Order the intercepts to generate ranges
                intercepts.sort_unstable_by(|a, b| a.total_cmp(b));

                // Each tuple represents a range that is within the shape
                return intercepts.into_iter()
                    .tuples()
                    .map(|(start, end)| start..end)
                    .collect();
            }),
            size);

        PathContour { 
            contour
        }
    }
}

impl<'a> SampledContour for &'a PathContour {
    type EdgeCellIterator = <&'a DynRayCastContour as SampledContour>::EdgeCellIterator;

    #[inline]
    fn contour_size(self) -> ContourSize {
        (&self.contour).contour_size()
    }

    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        (&self.contour).edge_cell_iterator()
    }

    #[inline]
    fn intercepts_on_line(self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        (&self.contour).intercepts_on_line(y)
    }
}