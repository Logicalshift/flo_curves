use super::distance_field::*;
use super::sampled_contour::*;

use smallvec::*;

///
/// A distance field to a circle with a particular radius
///
#[derive(Clone, Copy, PartialEq)]
pub struct CircularDistanceField {
    radius:         f64,
    int_radius_x:   f64,
    int_radius_y:   f64,
    diameter:       usize,
}

///
/// Finds the edge samples for a circular distance field
///
pub struct CircularDistanceFieldEdgeIterator {
    diameter:       usize,
    int_radius_x:   f64,
    int_radius_y:   f64,
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
        let int_radius  = radius.ceil() + 1.0;
        let diameter    = (int_radius as usize) * 2 + 1;

        CircularDistanceField {
            radius:         radius,
            int_radius_x:   int_radius,
            int_radius_y:   int_radius,
            diameter:       diameter,
        }
    }

    ///
    /// Gives the circle a non-linear offset, from between -1.0 to 1.0
    ///
    #[inline]
    pub fn with_center_offset(self, x: f64, y: f64) -> Self {
        CircularDistanceField {
            radius:         self.radius,
            int_radius_x:   self.int_radius_x + x,
            int_radius_y:   self.int_radius_y + y,
            diameter:       self.diameter,
        }
    }
}

impl SampledContour for CircularDistanceField {
    /// Iterator that visits all of the cells in this contour
    type EdgeCellIterator = CircularDistanceFieldEdgeIterator;

    #[inline]
    fn size(self) -> ContourSize {
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
    fn size(self) -> ContourSize {
        ContourSize(self.diameter, self.diameter)
    }

    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        let pos_x       = pos.0 as f64;
        let pos_y       = pos.1 as f64;
        let offset_x    = pos_x - self.int_radius_x;
        let offset_y    = pos_y - self.int_radius_y;

        (offset_x*offset_x + offset_y*offset_y) <= (self.radius*self.radius)
    }

    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        CircularDistanceFieldEdgeIterator {
            diameter:       self.diameter,
            int_radius_x:   self.int_radius_x,
            int_radius_y:   self.int_radius_y,
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
    fn size(self) -> ContourSize {
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
    fn size(self) -> ContourSize {
        ContourSize(self.diameter, self.diameter)
    }

    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        let pos_x       = pos.0 as f64;
        let pos_y       = pos.1 as f64;
        let offset_x    = pos_x - self.int_radius_x;
        let offset_y    = pos_y - self.int_radius_y;

        (offset_x*offset_x + offset_y*offset_y).sqrt() - self.radius
    }

    #[inline]
    fn as_contour(self) -> Self::Contour { self }
}

impl CircularDistanceFieldEdgeIterator {
    #[inline]
    fn point_is_inside_from_center(&self, offset_x: f64, offset_y: f64) -> bool {
        (offset_x*offset_x + offset_y*offset_y) <= self.radius_sq
    }
}

impl Iterator for CircularDistanceFieldEdgeIterator {
    type Item = (ContourPosition, ContourCell);

    fn next(&mut self) -> Option<Self::Item> {
        // Return a sample if one is already present in the iterator
        if !self.samples.is_empty() {
            return self.samples.pop();
        }

        // Find the samples for the next line
        let (xpos, ypos) = loop {
            // Finished returning all of the samples once we reach the end of the circle
            if self.ypos > self.diameter {
                return None;
            }

            // Get the y position to process. The initial y point is 'above' the circle so we detect the top-most edges
            let ypos    = self.ypos as f64 - 1.0;
            let ypos    = ypos - self.int_radius_y;

            // Advance the y position regardless of if there's a sample here
            self.ypos += 1;

            // The sample ypos is where we take the initial sample from. For edge on the negative side, we sample the 'lower' row and then flip for the bottom half so that the circle will be moving to the right
            let sample_ypos = if ypos <= 0.0 { ypos + 1.0 } else { ypos };

            if ypos == 0.0 {
                break (self.radius, ypos);
            }

            // At the top of the circle, move downwards to capture the first row where everything is 'below' the current position
            let ypos_sq = sample_ypos * sample_ypos;

            if ypos_sq > self.radius_sq {
                continue;
            }

            // Solve for the x positions along this y position (this is the positive version)
            let xpos_positive = (self.radius_sq - ypos_sq).sqrt();

            // This is the first x position on this scanline
            break (xpos_positive, ypos);
        };

        // Build up the edge samples on this row
        let mut samples: SmallVec<[_; 8]> = smallvec![];

        // We can treat our xpos and ypos as a sample location
        let mut sample_x = xpos.floor();
        let sample_y     = ypos.floor();

        // At least one of the surrounding points will be inside and at least one will be outside (making the optimiser do some work here removing things like the extra int_radius calculations)
        // For the negative sample one of bl or br should contain the sample
        let mut tl = self.point_is_inside_from_center(sample_x, sample_y);
        let mut tr = self.point_is_inside_from_center(sample_x+1.0, sample_y);
        let mut bl = self.point_is_inside_from_center(sample_x, sample_y+1.0);
        let mut br = self.point_is_inside_from_center(sample_x+1.0, sample_y+1.0);

        debug_assert!((tl || tr || bl || br) && (!tl || !tr || !bl || !br), "Picked a sample that was not on an edge at {:?}: {:?}", (xpos, ypos), (tl, tr, bl, br));

        // This should form the initial sample
        samples.push((ContourPosition((sample_x + self.int_radius_x + 1.0) as usize, (sample_y + self.int_radius_y + 1.0) as usize), ContourCell::from_corners(tl, tr, bl, br)));

        // There may be more edges on the riht of the sample we found. If y is -ve, then we'll be following an edge at the bottom, and if y is +ve then we'll be following an edge at the top
        debug_assert!((ypos >= 0.0 && (tl || tr) || (ypos <= 0.0 && (bl || br))));

        // Move to the right to fill in the rest of the line
        loop {
            // Sample one to the right. We can re-use the top-right and bottom-right samples from the last pixel
            sample_x -= 1.0;

            tr = tl;
            br = bl;
            tl = self.point_is_inside_from_center(sample_x, sample_y);
            bl = self.point_is_inside_from_center(sample_x, sample_y+1.0);

            if (!tl && !bl && !tr && !br) || (tl && bl && tr && br) || sample_x < 0.0 {
                // Stop once we reach empty space or the inside of the circle
                break;
            }

            // Push the next contour item
            samples.push((ContourPosition((sample_x + self.int_radius_x + 1.0) as usize, (sample_y + self.int_radius_y + 1.0) as usize), ContourCell::from_corners(tl, tr, bl, br)));
        }

        // Mirror to generate the full line
        let len         = samples.len();
        let mid_point_x = self.int_radius_x as usize + 1;
        for idx in 0..len {
            let (pos, cell) = samples[len-1-idx];
            let pos         = ContourPosition(mid_point_x - (pos.0-mid_point_x) - 1 , pos.1);

            samples.push((pos, cell.mirror_horiz()));
        }

        // Store the samples and pop the top one
        self.samples = samples;
        self.samples.pop()
    }
}
