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
    pub fn merge_with(&mut self, new_region: RectRegion) {
        // Nothing to do if the new region is empty
        if new_region.slices.is_empty() {
            return;
        }

        // Short circuit if we're empty too
        if self.slices.is_empty() {
            *self = new_region.clone();
            return;
        }

        // Update the region covered by the new slices
        self.x_region.start = self.x_region.start.min(new_region.x_region.start);
        self.x_region.end   = self.x_region.end.max(new_region.x_region.end);

        self.y_region.start = self.y_region.start.min(new_region.y_region.start);
        self.y_region.end   = self.y_region.end.max(new_region.y_region.end);

        // Merge/split each slice in turn
        let mut merge_scratch   = vec![];
        let mut new_slices      = Vec::with_capacity(self.slices.len().max(new_region.slices.len()));

        let mut new_region      = new_region;
        let mut our_slices      = self.slices.drain(..);
        let mut incoming_slices = new_region.slices.drain(..);

        let mut maybe_ours      = our_slices.next();
        let mut maybe_incoming  = incoming_slices.next();

        // For the case where multiple slices overlap, we need to track the y position we've reached in the 'larger' side
        let mut y_pos           = f64::MIN;

        loop {
            // Ours and the incoming slices are in order, so 'last_slice' is the slice before both 'ours' and 'incoming'
            if let (Some(ours), Some(incoming)) = (&maybe_ours, &maybe_incoming) {
                if ours.y_range.start < incoming.y_range.end && incoming.y_range.start < ours.y_range.end {
                    // These two slices overlap, so we need to split them up
                    let overlap_start   = ours.y_range.start.max(incoming.y_range.start);
                    let overlap_end     = ours.y_range.end.min(incoming.y_range.end);

                    debug_assert!(y_pos <= overlap_start);

                    // Copy the non-overlapping section from the start, if there is one (initial y positions can be the same, so this slice won't get generated)
                    if ours.y_range.start < incoming.y_range.start {
                        // Ours has a slice before incoming
                        let start_y = y_pos.max(ours.y_range.start);
                        let end_y   = incoming.y_range.start;

                        if end_y > start_y { new_slices.push(ours.clone_with_y_range(start_y..end_y)) }
                        // y_pos = end_y;
                    } else if incoming.y_range.start < ours.y_range.start {
                        // Incoming has a slice before ours
                        let start_y = y_pos.max(incoming.y_range.start);
                        let end_y   = ours.y_range.start;

                        if end_y > start_y { new_slices.push(incoming.clone_with_y_range(start_y..end_y)) }
                        // y_pos = end_y;
                    }

                    // Generate the overlap slice by merging the slices
                    let mut overlap = ours.clone_with_y_range(overlap_start..overlap_end);
                    overlap.merge(incoming, &mut merge_scratch);

                    new_slices.push(overlap);
                    y_pos = overlap_end;

                    // The next item is whichever item ends first (or both if they both end at the same point)
                    if ours.y_range.end < incoming.y_range.end {
                        maybe_ours = our_slices.next();
                    } else if incoming.y_range.end < ours.y_range.end {
                        maybe_incoming = incoming_slices.next();
                    } else {
                        // End points are equal, so advance both
                        maybe_ours      = our_slices.next();
                        maybe_incoming  = incoming_slices.next();
                    }
                } else if ours.y_range.start < incoming.y_range.start {
                    // No overlap, ours is first
                    let mut ours = maybe_ours.unwrap();
                    ours.y_range.start = ours.y_range.start.max(y_pos);

                    if ours.y_range.end > y_pos {
                        y_pos = ours.y_range.end;
                        new_slices.push(ours);
                    }

                    maybe_ours = our_slices.next();
                } else {
                    // No overlap, incoming must be first
                    let mut incoming = maybe_incoming.unwrap();
                    incoming.y_range.start = incoming.y_range.start.max(y_pos);

                    if incoming.y_range.end > y_pos {
                        y_pos = incoming.y_range.end;
                        new_slices.push(incoming);
                    }

                    maybe_incoming = incoming_slices.next();
                }
            } else if let Some(mut ours) = maybe_ours {
                // Only 'ours' left
                ours.y_range.start = ours.y_range.start.max(y_pos);

                if ours.y_range.start < ours.y_range.end { new_slices.push(ours); }
                maybe_ours = our_slices.next();
            } else if let Some(mut incoming) = maybe_incoming {
                // Only 'incoming' left
                incoming.y_range.start = incoming.y_range.start.max(y_pos);

                if incoming.y_range.start < incoming.y_range.end { new_slices.push(incoming); }
                maybe_incoming = incoming_slices.next();
            } else {
                // Both finished
                break;
            }
        }

        // new_slices now contains the merged set of slices
        drop(our_slices);
        self.slices = new_slices;

        // Combine slices if any end up containing the same values
        self.combine_matching_slices();
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

    ///
    /// Breaks this region down into a set of non-overlapping bounding boxes
    ///
    pub fn to_bounding_boxes<'a, TCoord>(&'a self) -> impl 'a + Iterator<Item=Bounds<TCoord>>
    where
        TCoord: Coordinate + Coordinate2D,
    {
        self.slices.iter()
            .flat_map(|slice| {
                slice.x_ranges.iter().map(move |x_range| {
                    let min = TCoord::from_components(&[x_range.start, slice.y_range.start]);
                    let max = TCoord::from_components(&[x_range.end, slice.y_range.end]);

                    Bounds::from_min_max(min, max)
                })
            })
    }

    ///
    /// Applies a transformation function to this RectRegion, returning a new axis-aligned region that covers the
    /// area that would be covered by this region if the specified transformation was applied.
    ///
    /// (As the new region is axis-aligned, it will cover a greater area the origin region)
    ///
    pub fn transform_axis_aligned<TCoord>(&self, transform_point: &impl Fn(TCoord) -> TCoord) -> Self
    where
        TCoord: Coordinate + Coordinate2D,
    {
        Self::from_bounds(self.to_bounding_boxes().map(|bbox| transform_bounding_box_axis_aligned(&bbox, transform_point)))
    }

    ///
    /// If we contain any slices that have matching x-ranges, then combine them into a single slice
    ///
    fn combine_matching_slices(&mut self) {
        if self.slices.is_empty() { return; }

        // Drain the slices and build a new vec
        let mut new_slices  = Vec::with_capacity(self.slices.len());
        let mut slices      = self.slices.drain(..);

        // The initial combined slice is the first in the list
        let mut combined_slice = slices.next().unwrap();

        while let Some(next_slice) = slices.next() {
            if combined_slice.can_combine_with(&next_slice) {
                // If the slices are combinable, then extend the y-range of the combined slice
                combined_slice.y_range.end = next_slice.y_range.end;
            } else {
                // If the slices are not combinable, then push the combined slice and continue with the next slice
                new_slices.push(combined_slice);
                combined_slice = next_slice;
            }
        }

        // Push the last slice
        new_slices.push(combined_slice);

        // Replace the slices with the combined set
        drop(slices);
        self.slices = new_slices;
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
            }

            if let Some(incoming) = maybe_incoming {
                if incoming.start <= last_range.end {
                    // 'incoming' overlaps the last_range, so consume it
                    last_range.end = incoming.end.max(last_range.end);
                    maybe_incoming = incoming_ranges.next();
                    continue;
                }
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
                // 'ours' and 'incoming' are both None so we're finished
                break;
            }
        }

        // Scratch now contains the merged ranges
        mem::swap(scratch, &mut self.x_ranges);
        scratch.clear();
    }

    ///
    /// Clones this region with a new y range set
    ///
    #[inline]
    fn clone_with_y_range(&self, new_y_range: Range<f64>) -> Self {
        Self {
            y_range: new_y_range,
            x_ranges: self.x_ranges.clone(),
        }
    }

    ///
    /// True if this slice can be combined with the other slice (joining their y ranges)
    ///
    #[inline]
    fn can_combine_with(&self, other_region: &RegionSlice) -> bool {
        if other_region.y_range.start != self.y_range.end {
            false
        } else if other_region.x_ranges.len() != self.x_ranges.len() {
            false
        } else {
            self.x_ranges == other_region.x_ranges
        }
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

#[cfg(test)]
mod test {
    use super::*;

    fn make_slice(x_ranges: Vec<Range<f64>>) -> RegionSlice {
        RegionSlice { y_range: 0.0..1.0, x_ranges }
    }

    #[test]
    fn merge_both_empty() {
        let mut a = make_slice(vec![]);
        let b     = make_slice(vec![]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert!(a.x_ranges.is_empty());
        assert!(scratch.is_empty(), "scratch should be cleared after merge");
    }

    #[test]
    fn merge_self_empty() {
        let mut a = make_slice(vec![]);
        let b     = make_slice(vec![1.0..3.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..3.0]);
        assert!(scratch.is_empty());
    }

    #[test]
    fn merge_other_empty() {
        let mut a = make_slice(vec![1.0..3.0]);
        let b     = make_slice(vec![]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..3.0]);
        assert!(scratch.is_empty());
    }

    #[test]
    fn merge_non_overlapping_ours_first() {
        let mut a = make_slice(vec![1.0..3.0]);
        let b     = make_slice(vec![5.0..7.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..3.0, 5.0..7.0]);
        assert!(scratch.is_empty());
    }

    #[test]
    fn merge_non_overlapping_incoming_first() {
        let mut a = make_slice(vec![5.0..7.0]);
        let b     = make_slice(vec![1.0..3.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..3.0, 5.0..7.0]);
        assert!(scratch.is_empty());
    }

    #[test]
    fn merge_overlapping_ours_first() {
        let mut a = make_slice(vec![1.0..5.0]);
        let b     = make_slice(vec![3.0..7.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..7.0]);
    }

    #[test]
    fn merge_overlapping_incoming_first() {
        let mut a = make_slice(vec![3.0..7.0]);
        let b     = make_slice(vec![1.0..5.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..7.0]);
    }

    #[test]
    fn merge_ours_contains_incoming() {
        let mut a = make_slice(vec![1.0..10.0]);
        let b     = make_slice(vec![3.0..7.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..10.0]);
    }

    #[test]
    fn merge_incoming_contains_ours() {
        let mut a = make_slice(vec![3.0..7.0]);
        let b     = make_slice(vec![1.0..10.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..10.0]);
    }

    /// Ranges touching at exactly one point (start == end of the other) should be merged
    #[test]
    fn merge_adjacent_ranges() {
        let mut a = make_slice(vec![1.0..3.0]);
        let b     = make_slice(vec![3.0..5.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..5.0]);
    }

    /// Two interleaved, non-overlapping ranges should all be preserved in sorted order
    #[test]
    fn merge_interleaved_non_overlapping() {
        let mut a = make_slice(vec![1.0..2.0, 5.0..6.0]);
        let b     = make_slice(vec![3.0..4.0, 7.0..8.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..2.0, 3.0..4.0, 5.0..6.0, 7.0..8.0]);
    }

    /// A single incoming range that bridges two of our ranges should collapse all three into one
    #[test]
    fn merge_incoming_bridges_two_of_ours() {
        let mut a = make_slice(vec![1.0..4.0, 6.0..8.0]);
        let b     = make_slice(vec![3.0..7.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..8.0]);
    }

    /// A single one-of-ours range that bridges two incoming ranges should collapse all three into one
    #[test]
    fn merge_ours_bridges_two_incoming() {
        let mut a = make_slice(vec![3.0..7.0]);
        let b     = make_slice(vec![1.0..4.0, 6.0..8.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..8.0]);
    }

    /// Identical ranges should produce a single copy
    #[test]
    fn merge_identical_ranges() {
        let mut a = make_slice(vec![2.0..5.0]);
        let b     = make_slice(vec![2.0..5.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![2.0..5.0]);
    }

    #[test]
    fn merge_many_overlapping_ranges_1() {
        let mut a = make_slice(vec![1.0..100.0]);
        let b     = make_slice(vec![2.0..3.0, 4.0..5.0, 6.0..7.0, 8.0..9.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..100.0]);
        assert!(scratch.is_empty());
    }

    #[test]
    fn merge_many_overlapping_ranges_2() {
        let mut a = make_slice(vec![2.0..3.0, 4.0..5.0, 6.0..7.0, 8.0..9.0]);
        let b     = make_slice(vec![1.0..100.0]);
        let mut scratch = vec![];

        a.merge(&b, &mut scratch);

        assert_eq!(a.x_ranges, vec![1.0..100.0]);
        assert!(scratch.is_empty());
    }
}
