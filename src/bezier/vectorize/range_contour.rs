use super::sampled_contour::*;

use smallvec::*;

use std::ops::{Range};

///
/// A range contour is a type of sampled contour that stores the 'inside' by marking ranges of pixels
///
/// This version performs samples along the y axis only
///
/// It has an unlimited 'canvas' size, although is inefficient if filled regions are not joined together.
/// It samples discretely in the y axis (at 1-unit boundaries), but the ranges on the x axis are stored
/// using floating point values, so can be in any position.
///
/// This is useful for constructing contours by combining many shapes. It can be used to track which
/// region of a canvas is invalid, or as a way to add or subtract large numbers of vector shapes
/// quickly (with less precision than the path arithmetic functions)
///
#[derive(Clone)]
pub struct RangeContour {
    /// The y position of the first line with intercepts on it
    min_y: i64,

    /// The maximum X position seen in the intercepts
    max_x: f64,

    /// The intercepts along each row (at y position min_y + index)
    intercepts: Vec<Vec<Range<f64>>>,
}

impl Default for RangeContour {
    fn default() -> Self {
        RangeContour { 
            min_y:      0, 
            max_x:      0.0,
            intercepts: vec![]
        }
    }
}

impl RangeContour {
    ///
    /// Creates a range contour from an existing one
    ///
    pub fn from_contour(contour: &impl SampledContour) -> Self {
        let source_size     = contour.contour_size();

        let mut min_y       = 0;
        let mut max_x       = 0.0f64;
        let mut intercepts  = vec![];

        // Iterate over every integer y position
        for y in 0..source_size.1 {
            let line_intercepts = contour.intercepts_on_line(y as _);

            let last_intercept = if let Some(last_intercept) = line_intercepts.last() {
                last_intercept.clone()
            } else {
                if intercepts.is_empty() {
                    // No intercepts so far, we don't need to store intercepts for this line
                    min_y += 1;
                } else {
                    // Just add an empty line
                    intercepts.push(vec![]);
                }
                continue;
            };

            // Add these intercepts
            max_x = max_x.max(last_intercept.end);
            intercepts.push(line_intercepts.into_iter().collect::<Vec<_>>());
        }

        // Combine into the contour
        RangeContour {
            min_y, max_x, intercepts
        }
    }

    ///
    /// Retrieves the intercepts for a particular y position (as a reference, so this is faster than the SampledContour version which copies the intercepts)
    ///
    #[inline]
    pub fn get_intercepts(&self, y_pos: i64) -> Option<&Vec<Range<f64>>> {
        if y_pos < 0 {
            None
        } else {
            self.intercepts.get(y_pos as usize)
        }
    }

    ///
    /// Adds to the intercepts vec so we can cover the specified y range
    ///
    fn extend_y_range(&mut self, min_y: i64, max_y: i64) {
        // If there are no intercepts, then set the initial minimum y position of the contour
        if self.intercepts.is_empty() {
            self.min_y = min_y;
        }
        
        // Add extra lines to the start of the intercepts to accomodate the new contour
        if min_y < self.min_y {
            let extra_at_start = self.min_y - min_y;
            self.intercepts.splice(0..0, (0..extra_at_start as usize).into_iter().map(|_| vec![]));

            self.min_y = min_y;
        }

        // Add extra lines to the end of the intercepts to accomodate the new contour
        if max_y > (self.min_y + self.intercepts.len() as i64) {
            let extra_at_end = (max_y - self.min_y) - self.intercepts.len() as i64;
            self.intercepts.extend((0..extra_at_end as usize).into_iter().map(|_| vec![]));
        }
    }

    ///
    /// Adds a contour into this contour, offsetting the x and y positions of the source by the specified amount
    ///
    pub fn add_contour(&mut self, contour: &impl SampledContour, offset: (f64, f64)) {
        // Start with the size of the contour (they start at 0,0)
        let source_size = contour.contour_size();

        // Get the y range that we're going to scan
        let source_min_y = offset.1.floor() as i64;
        let source_max_y = (offset.1 + source_size.1 as f64).ceil() as i64;

        self.extend_y_range(source_min_y, source_max_y);

        // Vec containing the new intercepts (which we swap around to avoid extra allocations)
        let mut new_intercepts = vec![];

        // Scan the contour and combine the ranges
        for y_pos in 0..source_size.1 {
            // Get the intercepts for this line
            let intercepts = contour.intercepts_on_line(y_pos as _);

            // Short-circuit the case where the intercepts are empty
            if intercepts.is_empty() { continue; }

            // Refill the intercepts on the line from the left (they're in ascending order)
            let line                        = (y_pos as i64 - self.min_y) as usize;
            let mut intercepts              = intercepts.into_iter();
            let Some(mut current_intercept) = intercepts.next() else { continue; };

            current_intercept.start += offset.0;
            current_intercept.end   += offset.0;

            // Drain the intercepts from the old line as we process them
            let mut old_intercepts = self.intercepts[line].drain(..);

            'outer: loop {
                if let Some(old_intercept) = old_intercepts.next() {
                    if old_intercept.end < current_intercept.start {
                        // The old intercept ends before the current intercept starts
                        new_intercepts.push(old_intercept);
                        continue;
                    }

                    // If the old intercept starts after the current intercept ends, then push from the 'current' list until we get an overlap
                    while old_intercept.start > current_intercept.end {
                        self.max_x = self.max_x.max(current_intercept.end);
                        new_intercepts.push(current_intercept);

                        if let Some(next_intercept) = intercepts.next() {
                            // Inspect the next intercept
                            current_intercept = next_intercept;

                            current_intercept.start += offset.0;
                            current_intercept.end   += offset.0;
                        } else {
                            // Entire new range fit before the old intercept
                            while let Some(old_intercept) = old_intercepts.next() {
                                new_intercepts.push(old_intercept);
                            }

                            // All done (both iterators depleted)
                            break 'outer;
                        }
                    }

                    // current_intercept overlaps old_intercept: combine the two
                    current_intercept.start = current_intercept.start.min(old_intercept.start);
                    current_intercept.end   = current_intercept.end.max(old_intercept.end);
                } else {
                    // No more intercepts in the old list: just fill with the intercepts from the 'current' list
                    loop {
                        self.max_x = self.max_x.max(current_intercept.end);
                        new_intercepts.push(current_intercept);

                        let Some(next_intercept) = intercepts.next() else { break; };
                        current_intercept = next_intercept;

                        current_intercept.start += offset.0;
                        current_intercept.end   += offset.0;
                    }

                    break;
                }
            }

            // Swap the new intercepts into the intercepts list (this also gives us a new empty vec for the next line from the one we just drained)
            use std::mem;
            drop(old_intercepts);
            mem::swap(&mut new_intercepts, &mut self.intercepts[line]);
        }
    }
}

impl SampledContour for RangeContour {
    fn contour_size(&self) -> ContourSize {
        // Use the maximum x/y coordinates to return a size (anything in the negative realm isn't included in the size)
        let max_y = self.min_y + self.intercepts.len() as i64;
        let max_y = (max_y as f64).max(0.0);

        let max_x = self.max_x.max(0.0);

        ContourSize(max_x.ceil() as _, max_y.ceil() as _)
    }

    fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        let y = y.round() as i64 - self.min_y;

        self.get_intercepts(y)
            .map(|intercepts| intercepts.iter().cloned().collect())
            .unwrap_or(smallvec![])
    }
}
