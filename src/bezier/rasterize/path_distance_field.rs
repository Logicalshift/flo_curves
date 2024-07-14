use super::path_contour::*;
use super::marching_parabola_distance_field::*;
use crate::geo::*;
use crate::bezier::path::*;
use crate::bezier::vectorize::*;

use std::sync::*;

///
/// Approximates a distance field generated from a path
///
#[derive(Clone)]
pub struct PathDistanceField {
    path_contour:       Arc<PathContour>,
    distance_field:     Arc<MarchingParabolaDistanceField>,
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
        // The path contour can be used both as the actual path contour and as a way to determine if a point is inside the path
        let path_contour = PathContour::from_path(path, size);
        let path_contour = Arc::new(path_contour);

        // Compute the distance field using the marching parabolas algorithm
        let marching_parabolas = MarchingParabolaDistanceField::from_intercepts(size.0, size.1, 
            |x| path_contour.intercepts_on_column(x), 
            |y| path_contour.intercepts_on_line(y));
        let distance_field = Arc::new(marching_parabolas);

        PathDistanceField { path_contour, distance_field }
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

impl SampledSignedDistanceField for PathDistanceField {
    type Contour = PathContour;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.path_contour.contour_size()
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        self.distance_field.distance_at_point(pos)
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        &self.path_contour
    }
}
