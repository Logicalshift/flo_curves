use super::sampled_contour::*;

use crate::geo::*;

use itertools::*;
use smallvec::*;

use std::ops::{Range};

#[derive(Clone)]
pub struct RegionSlice {
    /// The y range covered by this region
    y_range: Range<f64>,

    /// The x ranges covered by this region, in order and non-overlapping
    x_ranges: Vec<Range<f64>>,
}

///
/// Describes a region made by composing a set of (possibly overlapping) rectangles
///
/// This can be queried by y position to determine the regions that are inside or outside the region
///
#[derive(Clone)]
pub struct RectRegion {
    /// The horizontal slices that make up this region
    slices: Vec<RegionSlice>,

    /// The X coordinates covered by this region
    x_region: Range<f64>,

    /// The y coordinates covered by this region
    y_region: Range<f64>,
}

impl RectRegion {
    ///
    /// Creates a rectangle region that represents the union of a set of bounds
    ///
    pub fn from_bounds<TCoord>(bounds: impl IntoIterator<Item=Bounds<TCoord>>) -> Self
    where 
        TCoord: Coordinate + Coordinate2D,
    {
        // Order the bounding boxes by their initial y coordinate
        let mut ordered_bounds = bounds.into_iter().sorted_by(|a, b| a.min().y().total_cmp(&b.min().y())).peekable();

        // The slices we've generated for this region
        let mut slices: Vec<RegionSlice> = vec![];

        // The last Y position where we generated a slice
        let Some((mut last_y, mut x_region, mut y_region)) = ordered_bounds.peek()
            .map(|bounds| (bounds.min().y(), bounds.min().x()..bounds.max().x(), bounds.min().y()..bounds.max().y())) 
            else { return RectRegion { slices: vec![], x_region: 0.0..0.0, y_region: 0.0..0.0 } };

        // The set of active bounds for the current region
        let mut active_bounds: Vec<Bounds<TCoord>>  = vec![];
        let mut last_bounds: Option<Bounds<TCoord>> = None;

        while let Some(next_bounds) = ordered_bounds.next() {
            // If the next bounds are at a different y coordinate from the last bounds, then add them to the slices
            if let Some(last_bounds) = last_bounds {
                if last_bounds.min().y() != next_bounds.min().y() {
                    // Generate a slice for the region from the last slice to 'last_bounds'
                    Self::process_slices(&mut slices, &mut active_bounds, last_y, next_bounds.min().y());
                    last_y = next_bounds.min().y();
                }
            }

            // This becomes part of the pending bounds
            active_bounds.push(next_bounds);
            x_region.start  = x_region.start.min(next_bounds.min().x());
            x_region.end    = x_region.end.max(next_bounds.max().x());
            y_region.end    = y_region.end.max(next_bounds.max().y());

            // These are the new 'last bounds'
            last_bounds = Some(next_bounds);
        }

        // Process the bounds in 'active_bounds' (all the way to the end this time)
        Self::process_slices(&mut slices, &mut active_bounds, last_y, f64::MAX);

        RectRegion { slices, x_region, y_region }
    }

    ///
    /// Processes a set of active bounds to generate the slices
    ///
    fn process_slices<TCoord>(slices: &mut Vec<RegionSlice>, active_bounds: &mut Vec<Bounds<TCoord>>, min_y: f64, max_y: f64)
    where 
        TCoord: Coordinate + Coordinate2D,
    {
        use std::iter;

        // Remove any bounds that have finished before min_y, so active_bounds only contains the bounding boxes that overlap this region somehow
        active_bounds.retain(|bounds| bounds.max().y() > min_y);

        // Nothing to do if there are no remaining items in the active list
        if active_bounds.is_empty() {
            return;
        }

        // Order the active bounds by x position so we can generate the ranges
        active_bounds.sort_by(|a, b| a.min().x().total_cmp(&b.min().x()));

        // Slices are determined by the end positions of the y coordinates
        let y_positions = active_bounds.iter()
            .map(|bounds| bounds.max().y())
            .filter(|y_pos| *y_pos < max_y)
            .sorted_by(|a, b| a.total_cmp(b))
            .chain(iter::once(max_y));

        // Create slices starting at min_y
        let mut last_y = min_y;
        for end_y in y_positions {
            // Get the slices in this region (last_y..end_y), ordered by x coordinate (because we sorted active_bounds earlier)
            let mut x_slices = active_bounds.iter()
                .filter(|bounds| bounds.max().y() > last_y)
                .map(|bounds| bounds.min().x()..bounds.max().x());

            if let Some(mut current_region) = x_slices.next() {
                // Combine overlapping bounds and make the slice
                let mut x_ranges = vec![];

                while let Some(next_region) = x_slices.next() {
                    if next_region.start <= current_region.end {
                        // Bounds overlap
                        current_region.end = current_region.end.max(next_region.end);
                    } else {
                        // Bounds do not overlap
                        x_ranges.push(current_region);
                        current_region = next_region;
                    }
                }

                // Add the last region
                x_ranges.push(current_region);

                // Create the slice
                slices.push(RegionSlice { y_range: last_y..end_y, x_ranges: x_ranges });
            } else {
                // Can't be any more regions to process
                break;
            }

            // This is the new end_y
            last_y = end_y;
        }
    }

    ///
    /// Merges this region with another region
    ///
    pub fn add_region(&mut self, new_region: &RectRegion) {
        // TODO
    }

    ///
    /// Retrieves the bounds for everything in this region
    ///
    #[inline]
    pub fn bounds<TCoord>(&self) -> Bounds<TCoord>
    where 
        TCoord: Coordinate + Coordinate2D,
    {
        Bounds(TCoord::from_components(&[self.x_region.start, self.y_region.start]), TCoord::from_components(&[self.x_region.end, self.y_region.end]))
    }

    ///
    /// Returns the 'slices' that make up this region. These are ordered by y position (and are non-overlapping)
    ///
    #[inline]
    pub fn slices(&self) -> &[RegionSlice] {
        &*self.slices
    }
}

impl RegionSlice {
    ///
    /// The x-ranges covered by this slice (ordered by x position and non-overlapping)
    ///
    #[inline]
    pub fn x_ranges(&self) -> &[Range<f64>] {
        &self.x_ranges
    }

    ///
    /// The y range covered by this slice
    ///
    #[inline]
    pub fn y_range(&self) -> Range<f64> {
        self.y_range.clone()
    }

    ///
    /// Merges another slice with this slice. Scratch space should be empty when this is called (and will be 
    /// empty once this returns). We don't consider the y-ranges with this operation
    ///
    fn merge(&mut self, other_slice: &RegionSlice, scratch: &mut Vec<Range<f64>>) {
        use std::mem;

        debug_assert!(scratch.is_empty());

        // Iterate on the ranges in the other slice
        let mut our_ranges      = self.x_ranges.iter();
        let mut incoming_ranges = other_slice.x_ranges.iter();

        let mut maybe_ours      = our_ranges.next();
        let mut maybe_incoming  = incoming_ranges.next();

        // The 'last_range' is the combined range that we're building
        let mut last_range = if let (Some(ours), Some(incoming)) = (maybe_ours, maybe_incoming) {
            if ours.start < incoming.start {
                // 'Ours' is first
                maybe_ours = our_ranges.next();
                ours.clone()
            } else {
                // 'Incoming' is first
                maybe_incoming = incoming_ranges.next();
                incoming.clone()
            }
        } else if let Some(ours) = maybe_ours {
            // No incoming
            maybe_ours = our_ranges.next();
            ours.clone()
        } else if let Some(incoming) = maybe_incoming {
            // No 'ours'
            maybe_incoming = incoming_ranges.next();
            incoming.clone()
        } else {
            return;
        };

        loop {
            // Merge from the two sides into last_range if they overlap
            if let Some(ours) = maybe_ours {
                if ours.start <= last_range.end {
                    // 'ours' overlaps the last_range, so consume it
                    last_range.end = ours.end.max(last_range.end);
                    maybe_ours = our_ranges.next();
                    continue;
                }
            } else if let Some(incoming) = maybe_incoming {
                if incoming.start <= last_range.end {
                    // 'incoming' overlaps the last_range, so consume it
                    last_range.end = incoming.end.max(last_range.end);
                    maybe_incoming = incoming_ranges.next();
                    continue;
                }
            } else {
                // 'ours' and 'incoming' are both None so we're finished after adding the last_range to the end of the new list of ranges
                scratch.push(last_range);
                break;
            }

            // Ranges don't overlap: add the last range to the result
            scratch.push(last_range);

            // Pick a new last range
            last_range = if let (Some(ours), Some(incoming)) = (maybe_ours, maybe_incoming) {
                if ours.start < incoming.start {
                    // 'ours' is first
                    maybe_ours = our_ranges.next();
                    ours.clone()
                } else {
                    // 'incoming' is first
                    maybe_incoming = incoming_ranges.next();
                    incoming.clone()
                }
            } else if let Some(ours) = maybe_ours {
                // Only 'ours' is left
                maybe_ours = our_ranges.next();
                ours.clone()
            } else if let Some(incoming) = maybe_incoming {
                // Only 'incoming' is left
                maybe_incoming = incoming_ranges.next();
                incoming.clone()
            } else {
                // Unreachable: the 'None, None' case is dealt with above
                unreachable!()
            }
        }

        // Scratch now contains the merged ranges
        mem::swap(scratch, &mut self.x_ranges);
        scratch.clear();
    }
}

impl SampledContour for RectRegion {
    fn contour_size(&self) -> ContourSize {
        let max_x = self.x_region.end.max(0.0).ceil();
        let max_y = self.y_region.end.max(0.0).ceil();

        ContourSize(max_x as _, max_y as _)
    }

    fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        // Search by the end of the slice (so both indexes are the index of the first slice that contains y and ends after y)
        let slice_idx = self.slices.binary_search_by(|slice| slice.y_range.end.total_cmp(&y));
        let slice_idx = match slice_idx { Ok(slice) => slice + 1, Err(slice) => slice };

        // Return the x ranges from the slice that we found
        if let Some(slice) = self.slices.get(slice_idx) {
            if slice.y_range.start <= y {
                slice.x_ranges.iter().cloned().collect()
            } else {
                smallvec![]
            }
        } else {
            smallvec![]
        }
    }
}
