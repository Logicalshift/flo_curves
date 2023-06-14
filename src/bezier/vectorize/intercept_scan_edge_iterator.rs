use super::sampled_contour::*;

use smallvec::*;

use std::ops::{Range};

///
/// An iterator that finds the edges of a contour by calling the `intercepts_on_line()` function
///
pub struct InterceptScanEdgeIterator<'a, TContour>
where
    TContour: SampledContour,
{
    /// The contour that this is tracing the edges of
    contour: &'a TContour,

    /// The y pos of the current line
    ypos: usize,

    /// The preceding the current one
    previous_line: SmallVec<[Range<usize>; 4]>,

    /// The line following the current one
    current_line: SmallVec<[Range<usize>; 4]>,

    /// Index into the previous line of the current x position
    previous_pos: usize,

    /// Index into the current line of the current x position
    current_pos: usize,

    /// The next x position to return
    xpos: usize,
}

impl<'a, TContour> InterceptScanEdgeIterator<'a, TContour>
where
    TContour: SampledContour,
{
    ///
    /// Creates a new edge iterator at the top-left corner of a contour
    ///
    pub fn new(contour: &'a TContour) -> InterceptScanEdgeIterator<'a, TContour> {
        // Create an edge iterator in a neutral state
        let mut iterator = InterceptScanEdgeIterator {
            contour:        contour,
            ypos:           0,
            previous_line:  smallvec![],
            current_line:   smallvec![],
            previous_pos:   0,
            current_pos:    0,
            xpos:           0,
        };

        // Load the first line into the iterator
        iterator.load_line(0);

        iterator
    }

    ///
    /// Loads a line ahead of the current line into this contour
    ///
    fn load_line(&mut self, ypos: usize) {
        use std::mem;

        let height      = self.contour.contour_size().height();
        let mut ypos    = ypos;

        loop {
            // Move the current line into the previous line
            mem::swap(&mut self.previous_line, &mut self.current_line);

            // Load the next line from the contour
            if ypos < height {
                self.current_line = self.contour.intercepts_on_line(ypos);
            } else {
                self.current_line = smallvec![];
            }

            // Try to pick an x position to start at (one of the lines must be non-empty)
            let mut xpos = None;

            if self.previous_line.len() > 0 {
                xpos = Some(self.previous_line[0].start);
            }

            if self.current_line.len() > 0 {
                xpos = xpos.map_or(Some(self.current_line[0].start), |xpos| Some(xpos.max(self.current_line[0].start)));
            }

            if let Some(xpos) = xpos {
                // Found the next line
                self.previous_pos   = 0;
                self.current_pos    = 0;
                self.xpos           = xpos;
                self.ypos           = ypos;
                break;
            }

            // Try the next y position if we didn't find a match
            ypos += 1;

            if ypos > height {
                // No more lines in this shape
                self.previous_pos   = 0;
                self.current_pos    = 0;
                self.xpos           = 0;
                self.ypos           = height;
                break;
            }
        }
    }
}

impl<'a, TContour> Iterator for InterceptScanEdgeIterator<'a, TContour>
where
    TContour: SampledContour,
{
    type Item = (ContourPosition, ContourCell);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
