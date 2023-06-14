use super::brush_stroke::*;
use super::distance_field::*;
use super::sampled_contour::*;
use super::intercept_scan_edge_iterator::*;
use crate::geo::*;

use smallvec::*;

use std::ops::{Range};

///
/// A distance field to a circle with a particular radius
///
#[derive(Clone, Copy, PartialEq)]
pub struct CircularDistanceField {
    radius:         f64,
    center_x:       f64,
    center_y:       f64,
    diameter:       usize,
}

impl CircularDistanceField {
    ///
    /// Creates a new sampled distance field for a circle with the specified radius
    ///
    #[inline]
    pub fn with_radius(radius: f64) -> Self {
        let radius      = if radius < 0.0 { 0.0 } else { radius };
        let center      = radius.ceil() + 1.0;
        let diameter    = (center as usize) * 2 + 1;

        CircularDistanceField {
            radius:         radius,
            center_x:       center,
            center_y:       center,
            diameter:       diameter,
        }
    }

    ///
    /// Gives the circle a non-linear offset, from between 0.0 to 1.0
    ///
    #[inline]
    pub fn with_center_offset(self, x: f64, y: f64) -> Self {
        let center_x = self.center_x + x;
        let center_y = self.center_y + y;

        CircularDistanceField {
            radius:         self.radius,
            center_x:       center_x,
            center_y:       center_y,
            diameter:       ((center_x.max(center_y)).floor() as usize) * 2 + 1,
        }
    }

    ///
    /// Returns a circular distance field and an offset that will create a circle centered at the specified position
    ///
    /// All of the points within the resulting circle must be at positive coordinates (ie, `x-radius` and `y-radius` must
    /// be positive values). This is intended to be used as input to the `DaubBrushDistanceField` type to create brush
    /// strokes out of many circle.
    ///
    pub fn centered_at_position(pos: impl Coordinate + Coordinate2D, radius: f64) -> Option<(CircularDistanceField, ContourPosition)> {
        if radius <= 0.0 { return None; }

        let circle = CircularDistanceField::with_radius(radius);

        let x = pos.x() - circle.center_x - 1.0;
        let y = pos.y() - circle.center_y - 1.0;

        debug_assert!(x-radius >= 0.0);
        debug_assert!(y-radius >= 0.0);

        if x < 0.0 || y < 0.0 { return None; }

        let offset_x = x - x.floor();
        let offset_y = y - y.floor();

        let circle      = circle.with_center_offset(offset_x, offset_y);
        let position    = ContourPosition(x.floor() as usize, y.floor() as usize);

        Some((circle, position))
    }
}

impl SampledContour for CircularDistanceField {
    /// Iterator that visits all of the cells in this contour
    type EdgeCellIterator = InterceptScanEdgeIterator<Self>;

    #[inline]
    fn contour_size(self) -> ContourSize {
        ContourSize(self.diameter, self.diameter)
    }

    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        (&self).point_is_inside(pos)
    }

    #[inline]
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        // Don't really want to pass in Self here as this will cause a lot of copying...
        todo!()
        // (&self).edge_cell_iterator()
    }

    #[inline]
    fn intercepts_on_line(self, y: usize) -> SmallVec<[Range<usize>; 4]> {
        (&self).intercepts_on_line(y)
    }
}

impl<'a> SampledContour for &'a CircularDistanceField {
    /// Iterator that visits all of the cells in this contour
    type EdgeCellIterator = InterceptScanEdgeIterator<&'a CircularDistanceField>;

    #[inline]
    fn contour_size(self) -> ContourSize {
        ContourSize(self.diameter, self.diameter)
    }

    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        let pos_x       = pos.0 as f64;
        let pos_y       = pos.1 as f64;
        let offset_x    = pos_x - self.center_x;
        let offset_y    = pos_y - self.center_y;

        (offset_x*offset_x + offset_y*offset_y) <= (self.radius*self.radius)
    }

    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        InterceptScanEdgeIterator::new(self)
    }

    #[inline]
    fn intercepts_on_line(self, ypos: usize) -> SmallVec<[Range<usize>; 4]> {
        let y = (ypos as f64) - self.center_y;

        if y.abs() <= self.radius {
            let intercept   = ((self.radius*self.radius) - (y*y)).sqrt();
            let min_x       = self.center_x - intercept;
            let max_x       = self.center_x + intercept;
            let min_x_ceil  = min_x.ceil();
            let max_x_floor = (max_x + 1.0).floor();

            let min_x_ceil = if min_x_ceil - min_x > 0.999999 {
                // Could be rounding error :-/
                if self.point_is_inside(ContourPosition(min_x_ceil as usize - 1, ypos)) {
                    min_x_ceil - 1.0
                } else {
                    min_x_ceil
                }
            } else {
                min_x_ceil
            };

            let max_x_floor = if max_x_floor - max_x > 0.99999 {
                // Another possible rounding error
                if !self.point_is_inside(ContourPosition(max_x_floor as usize - 1, ypos)) {
                    max_x_floor - 1.0
                } else {
                    max_x_floor
                }
            } else if max_x_floor - max_x < 0.00001 {
                // Final rounding error
                if self.point_is_inside(ContourPosition(max_x_floor as usize, ypos)) {
                    max_x_floor + 1.0
                } else {
                    max_x_floor
                }
            } else {
                max_x_floor
            };

            let min_x = min_x_ceil as usize;
            let max_x = max_x_floor as usize;

            if min_x == max_x {
                smallvec![]
            } else {
                smallvec![min_x..max_x]
            }
        } else {
            smallvec![]
        }
    }
}

impl BrushDistanceField for CircularDistanceField {
    #[inline]
    fn create_daub(centered_at: impl Coordinate + Coordinate2D, radius: f64) -> Option<(Self, ContourPosition)> {
        Self::centered_at_position(centered_at, radius)
    }
}

impl SampledSignedDistanceField for CircularDistanceField {
    type Contour = CircularDistanceField;

    #[inline]
    fn field_size(self) -> ContourSize {
        ContourSize(self.diameter, self.diameter)
    }

    #[inline]
    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        (&self).distance_at_point(pos)
    }

    #[inline]
    fn as_contour(self) -> Self::Contour { self }
}

impl<'a> SampledSignedDistanceField for &'a CircularDistanceField {
    type Contour = &'a CircularDistanceField;

    #[inline]
    fn field_size(self) -> ContourSize {
        ContourSize(self.diameter, self.diameter)
    }

    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        let pos_x       = pos.0 as f64;
        let pos_y       = pos.1 as f64;
        let offset_x    = pos_x - self.center_x;
        let offset_y    = pos_y - self.center_y;

        (offset_x*offset_x + offset_y*offset_y).sqrt() - self.radius
    }

    #[inline]
    fn as_contour(self) -> Self::Contour { self }
}
