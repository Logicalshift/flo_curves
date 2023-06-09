use super::distance_field::*;
use super::sampled_contour::*;
use crate::geo::*;

use smallvec::*;

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

///
/// Finds the edge samples for a circular distance field
///
pub struct CircularDistanceFieldEdgeIterator {
    diameter:       usize,
    center_x:       f64,
    center_y:       f64,
    radius:         f64,
    radius_sq:      f64,
    ypos:           usize,
    samples:        SmallVec<[(ContourPosition, ContourCell); 8]>,
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
    type EdgeCellIterator = CircularDistanceFieldEdgeIterator;

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
        (&self).edge_cell_iterator()
    }
}

impl<'a> SampledContour for &'a CircularDistanceField {
    /// Iterator that visits all of the cells in this contour
    type EdgeCellIterator = CircularDistanceFieldEdgeIterator;

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
        CircularDistanceFieldEdgeIterator {
            diameter:       self.diameter,
            center_x:       self.center_x,
            center_y:       self.center_y,
            radius:         self.radius,
            radius_sq:      self.radius * self.radius,
            ypos:           0,
            samples:        smallvec![],
        }
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

impl CircularDistanceFieldEdgeIterator {
    #[inline]
    fn point_is_inside(&self, offset_x: f64, offset_y: f64) -> bool {
        let offset_x    = offset_x - self.center_x;
        let offset_y    = offset_y - self.center_y;

        (offset_x*offset_x + offset_y*offset_y) <= (self.radius*self.radius)
    }

    ///
    /// Given a point where the circle interesects the current y position on the left-hand side, fills in the edge samples for the
    /// entire line.
    ///
    #[inline]
    fn fill_samples(&mut self, x_intersection: f64, ypos: f64) {
        let mut sample_x = x_intersection.floor();
        let mut sample_y = ypos.floor();

        if self.point_is_inside(sample_x, sample_y) || self.point_is_inside(sample_x, sample_y+1.0) {
            // We want the edge to lie in the middle of the first cell (in this case, x_intersection == x_intersection.floor(), which would put the edge on the outside of the cell)
            sample_x -= 1.0;
        }

        // Create the first sample
        let tl = self.point_is_inside(sample_x, sample_y);
        let tr = self.point_is_inside(sample_x+1.0, sample_y);
        let bl = self.point_is_inside(sample_x, sample_y+1.0);
        let br = self.point_is_inside(sample_x+1.0, sample_y+1.0);

        // Create the initial position and edge
        let mut pos  = ContourPosition(sample_x as usize + 1, sample_y as usize + 1);
        let mut cell = ContourCell::from_corners(tl, tr, bl, br);

        if cell.is_empty() {
            // Edge case: there's no sample on this line in spite of the intersection
            return;
        }

        debug_assert!(!cell.is_full());

        // Push the left-hand side samples by iterating until the cell is filled or we reach the middle
        while sample_x < self.center_x {
            // Add the current sample
            self.samples.push((pos, cell));

            // Move the sample to the next position
            pos.0    += 1;
            sample_x += 1.0;
            cell     = cell.shift_left();

            let tr   = self.point_is_inside(sample_x+1.0, sample_y);
            let br   = self.point_is_inside(sample_x+1.0, sample_y+1.0);
            cell     = cell.merge(ContourCell::from_corners(false, tr, false, br));

            // Stop if the cell is filled (we've reached all of the corners)
            if cell.is_full() {
                break;
            }
        }

        // The right-hand samples start at the mirror position from where the cell was filled (we may need to step one point further left)
        sample_x = if sample_x > self.center_x { self.center_x.ceil() } else { (2.0*self.center_x - sample_x).floor() };

        let tl = self.point_is_inside(sample_x, sample_y);
        let tr = self.point_is_inside(sample_x+1.0, sample_y);
        let bl = self.point_is_inside(sample_x, sample_y+1.0);
        let br = self.point_is_inside(sample_x+1.0, sample_y+1.0);

        let mut pos  = ContourPosition(sample_x as usize + 1, sample_y as usize + 1);
        let mut cell = ContourCell::from_corners(tl, tr, bl, br);

        // Move to the right if the cell is initially full
        if cell.is_full() {
            pos.0    += 1;
            sample_x += 1.0;
            cell     = cell.shift_left();

            let tr   = self.point_is_inside(sample_x+1.0, sample_y);
            let br   = self.point_is_inside(sample_x+1.0, sample_y+1.0);
            cell     = cell.merge(ContourCell::from_corners(false, tr, false, br));
        }

        debug_assert!(!cell.is_full());

        while !cell.is_empty() {
            // Add the current sample
            self.samples.push((pos, cell));

            // Move the sample to the next position
            pos.0    += 1;
            sample_x += 1.0;
            cell     = cell.shift_left();

            let tr   = self.point_is_inside(sample_x+1.0, sample_y);
            let br   = self.point_is_inside(sample_x+1.0, sample_y+1.0);
            cell     = cell.merge(ContourCell::from_corners(false, tr, false, br));
        }
    }
}

impl Iterator for CircularDistanceFieldEdgeIterator {
    type Item = (ContourPosition, ContourCell);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Return a sample if one is already present in the iterator
            if !self.samples.is_empty() {
                return self.samples.pop();
            }

            // Once the y position moves outside of the region where the circle exists, we are finished
            if self.ypos > self.diameter {
                return None;
            }

            // Retrieve the y position for the current line
            let ypos        = self.ypos as f64;
            let is_top_half = ypos <= (self.center_y - 1.0);

            // Each cell consists of the 'current' and the 'following' line. For the top half of the circle, the 'following' line defines the intersection position
            let test_ypos   = if is_top_half { ypos + 1.0 } else { ypos };

            // Advance the y position
            self.ypos += 1;

            // Compute the intersection position at this y position. This is the start of the LHS of the edge (can mirror around the center point to get the other side)
            // Check that the y position is inside the circle, and skip this line if not
            let x_intersection = if self.radius_sq - ((test_ypos - self.center_y) * (test_ypos - self.center_y)) >= 0.0 {
                // test_ypos intersects the line
                self.center_x - (self.radius_sq - ((test_ypos - self.center_y) * (test_ypos - self.center_y))).sqrt()
            } else if self.radius_sq - (((test_ypos + 1.0) - self.center_y) * ((test_ypos + 1.0) - self.center_y)) >= 0.0 {
                // (Rare) test_ypos does not intersect the line but the next line does
                self.center_x - (self.radius_sq - (((test_ypos + 1.0) - self.center_y) * ((test_ypos + 1.0) - self.center_y))).sqrt()
            } else {
                // Skip the line if it does not intercept the circle
                continue;
            };

            self.fill_samples(x_intersection, ypos);
            self.samples.reverse();
        }
    }
}
