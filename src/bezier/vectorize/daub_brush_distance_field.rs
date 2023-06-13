use super::distance_field::*;
use super::sampled_contour::*;

use std::cell::{RefCell};

use smallvec::*;

use std::ops::{Range};

///
/// Describes a shape as a distance field made up by 'daubing' discrete brush shapes over a canvas
///
/// Each brush shape - 'daub' - is itself a distance field, and can be placed at any integer position on the canvas (to 
/// position at subpixels, they will need to be separately resampled). By combining these shapes, a distance field 
/// describing a brush stroke can be constructed, which can be converted into a vector using 
/// `trace_contours_from_distance_field()`
///
/// Note that for just creating a thick line, `offset_lms_sampling()` is much faster but it can only offset along a
/// fixed distance along the normal of the curve, so it doesn't produce good results if the offset is changing across
/// the span of the curve or if the curve is not particularly smooth. `offset_scaling()` is also available as an even
/// faster alternative for the simple cases, but is even more limited in terms of what it can produce.
///
/// This provides the most general purpose approach to generating vectors from brush strokes or other patterns.
///
pub struct DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    /// The size of this distance field, sufficient to contain all of the 'daubs'
    size: ContourSize,

    /// The 'daubs' that make up the brush stroke, and where they appear on the canvas. This is stored sorted by Y position
    /// to allow scanning downwards to find which 'daubs' influence which points
    daubs: Vec<(TDaub, ContourPosition)>,

    /// The scanline cache is used to improve the performance of the `intercepts_on_line()` function by tracking what we found on the previous line
    scanline_cache: RefCell<Option<ScanlineCache>>,
}

///
/// Caches the indexes of the daubs that are on a particular scanline
///
struct ScanlineCache {
    /// The y position of the scanline
    ypos: usize,

    /// The index of the first daub that has not been included in the cache
    /// As the daubs are in y order, this will 
    next_daub: usize,

    /// The indexes of the daubs on this scanline, ordered by x position
    scanline_daubs: Vec<usize>,

    /// A preallocated vec ready to load the daubs for the next line
    idle_daubs: Vec<usize>,
}

///
/// Represents an iterator over the edges in a daub brush distance field
///
struct EdgeIterator<TContour>
where
    TContour: SampledContour,
{
    contour:        TContour,
    iterator:       TContour::EdgeCellIterator,
    daub_position:  ContourPosition,
    size:           ContourSize,
    lookahead:      (ContourPosition, ContourCell),
    finished:       bool,
}

///
/// Iterates over the edges in a daub brush distance field
///
pub struct DaubBrushContourIterator<'a, TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    /// The distance field that is being iterated over
    distance_field: &'a DaubBrushDistanceField<TDaub>,

    /// The index of the next daub to start (in scanline order: the iterators are generally unordered otherwise)
    next_daub_idx: usize,

    /// Edge iterators, position of the corresponding daub, and the iterator for the following cells if there are any
    edge_iterators: Vec<EdgeIterator<TDaub::Contour>>,

    /// The edge iterators that are on a future scanline (these are generally iterators that have moved down a scanline)
    future_scanline_iterators: Vec<EdgeIterator<TDaub::Contour>>,

    /// The scanline that we're collecting edges for
    current_scanline: usize,
}

impl ScanlineCache {
    ///
    /// Creates a new daub scanline cache, initialised at ypos = 0
    ///
    pub fn new<TDaub>(daubs: &Vec<(TDaub, ContourPosition)>) -> Self {
        let ypos                = 0;
        let mut next_daub       = 0;
        let mut scanline_daubs  = vec![];
        let idle_daubs          = vec![];

        // Read the daubs on the first scanline
        while next_daub < daubs.len() && daubs[next_daub].1.1 <= ypos {
            scanline_daubs.push(next_daub);
            next_daub += 1;
        }

        // Order by x position
        scanline_daubs.sort_by(|a, b| daubs[*a].1.0.cmp(&daubs[*b].1.0));

        ScanlineCache {
            ypos, next_daub, scanline_daubs, idle_daubs
        }
    }

    ///
    /// Moves forward in y position until the specified line is reached
    ///
    pub fn move_to_line<TDaub>(&mut self, daubs: &Vec<(TDaub, ContourPosition)>, new_ypos: usize) 
    where
        TDaub: SampledSignedDistanceField,
    {
        use std::mem;

        // Nothing to do if we're already on the specified line
        if self.ypos == new_ypos {
            return;
        }

        // Build a list of the indexes of the daubs that are new to the next line
        let mut new_daubs   = vec![];
        let num_daubs       = daubs.len();

        while self.next_daub < num_daubs {
            let (possible_daub, pos) = &daubs[self.next_daub];

            // Stop once we find a daub that's beyond the next point
            if pos.1 > new_ypos {
                break;
            }
            
            // Add the daub if its bounding box is not before the current position
            let max_y = pos.1 + possible_daub.field_size().height();

            if max_y > new_ypos {
                new_daubs.push(self.next_daub);
            }

            self.next_daub += 1;
        }

        // Order the new daubs so they can be mixed in to the existing ones
        new_daubs.sort_by(|a, b| daubs[*a].1.0.cmp(&daubs[*b].1.0));

        // Fill the idle structure from the new daubs and the existing daubs (we can just merge them together and eliminate any finished daubs as we go)
        self.idle_daubs.clear();

        let mut new_daubs       = new_daubs.into_iter();
        let mut old_daubs       = self.scanline_daubs.iter();
        let mut maybe_next_new  = new_daubs.next();
        let mut maybe_next_old  = old_daubs.next();

        loop {
            // Skip the old daub if it ends at this position
            if let Some(next_old) = maybe_next_old {
                let (daub, pos) = &daubs[*next_old];
                let max_y = pos.1 + daub.field_size().height();

                if max_y <= new_ypos {
                    // We've moved beyond the end of this daub (keep going)
                    maybe_next_old = old_daubs.next();
                    continue;
                }
            }

            match (maybe_next_old, maybe_next_new) {
                (Some(next_old), Some(next_new)) => {
                    // Decide whether or not to add the old or the new item next
                    let (_old_daub, old_pos) = &daubs[*next_old];
                    let (_new_daub, new_pos) = &daubs[next_new];

                    if new_pos.x() > old_pos.x() {
                        // New daub is before the old one
                        self.idle_daubs.push(next_new);
                        maybe_next_new = new_daubs.next();
                    } else {
                        // Old daub is before the new one
                        self.idle_daubs.push(*next_old);
                        maybe_next_old = old_daubs.next();
                    }
                }

                (Some(next_old), None) => {
                    // Add the next old item to the idle list
                    self.idle_daubs.push(*next_old);

                    // Move to the next item, then continue (so that the check above is made on the next item)
                    maybe_next_old = old_daubs.next();
                }

                (None, Some(next_new)) => {
                    // Add the remaining new items and stop
                    self.idle_daubs.push(next_new);

                    while let Some(next_new) = new_daubs.next() {
                        self.idle_daubs.push(next_new);
                    }
                    break;
                }

                (None, None) => { break; }
            }
        }

        // Swap the idle list for the active scanline daubs
        mem::swap(&mut self.scanline_daubs, &mut self.idle_daubs);
        self.ypos = new_ypos;
    }
}

impl<TDaub> DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    ///
    /// Creates a daub brush distance field from a list of daubs and their positions
    ///
    pub fn from_daubs(daubs: impl IntoIterator<Item=(TDaub, ContourPosition)>) -> DaubBrushDistanceField<TDaub> {
        let scanline_cache = RefCell::new(None);

        // Collect the daubs
        let mut daubs   = daubs.into_iter().collect::<Vec<_>>();

        // Size is the outer extent of all the daubs
        let size        = daubs.iter()
            .fold(ContourSize(0, 0), |last_size, (next_daub, next_pos)| {
                let ContourPosition(x, y)       = next_pos;
                let ContourSize(w, h)           = last_size;

                let ContourSize(daub_w, daub_h) = next_daub.field_size();
                let daub_w                      = x + daub_w;
                let daub_h                      = y + daub_h;

                ContourSize(usize::max(w, daub_w), usize::max(h, daub_h))
            });

        // Sort the daubs by y position
        daubs.sort_by_key(|(_, ContourPosition(_, y))| *y);

        DaubBrushDistanceField {
            size, daubs, scanline_cache
        }
    }
}

impl<'a, TDaub> SampledContour for &'a DaubBrushDistanceField<TDaub> 
where
    TDaub: SampledSignedDistanceField,
{
    type EdgeCellIterator = DaubBrushContourIterator<'a, TDaub>;

    #[inline]
    fn contour_size(self) -> ContourSize {
        self.size
    }

    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        self.distance_at_point(pos) <= 0.0
    }

    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        // Create the iterator
        let mut iterator = DaubBrushContourIterator {
            distance_field:             self,
            next_daub_idx:              0,
            edge_iterators:             vec![],
            future_scanline_iterators:  vec![],
            current_scanline:           0,
        };

        iterator.start_scanline();

        iterator
    }

    fn intercepts_on_line(self, y: usize) -> SmallVec<[Range<usize>; 4]> {
        // Create or fetch the cache
        let mut cache = self.scanline_cache.borrow_mut();
        let cache     = if let Some(cache) = &mut *cache {
            cache
        } else {
            *cache = Some(ScanlineCache::new(&self.daubs));
            cache.as_mut().unwrap()
        };

        // Update the cache to contain the daubs on the current line
        if cache.ypos > y {
            *cache = ScanlineCache::new(&self.daubs);
        }
        cache.move_to_line(&self.daubs, y);

        // Scan the intercepts left-to-right to build up the intercepts on this line
        let mut intercepts: SmallVec<[Range<usize>; 4]> = smallvec![];
        let mut to_remove = vec![];

        for daub_idx in cache.scanline_daubs.iter().copied() {
            let (daub, pos) = &self.daubs[daub_idx];

            for intercept in daub.as_contour().intercepts_on_line(y - pos.1).into_iter() {
                // Strip empty ranges if they occur
                if intercept.start == intercept.end-1 { continue; }

                // Offset the intercept by the position of this daub
                let intercept = (pos.0 + intercept.start)..(pos.0 + intercept.end);

                // In general, intercepts move left to right, so we should overlap the end of vec in general
                if intercepts.len() == 0 {
                    // First intercept
                    intercepts.push(intercept);
                } else if intercepts[intercepts.len()-1].end < intercept.start {
                    // Beyond the end of the last intercept
                    intercepts.push(intercept);
                } else {
                    // Might overlap one of the intercepts towards the end of the list
                    for idx in (0..intercepts.len()).into_iter().rev() {
                        if intercepts[idx].end < intercept.start {
                            // All the remaining ranges are before the start of this one
                            break;
                        } else if intercepts[idx].start <= intercept.end && intercepts[idx].end >= intercept.start {
                            // Ranges overlap
                            intercepts[idx].end = intercepts[idx].end.max(intercept.end);

                            if intercept.start < intercepts[idx].start {
                                // Range extends to the left
                                intercepts[idx].start = intercept.start;

                                // If a range start expands to the left, there may be preceding ranges that overlap this one
                                to_remove.clear();

                                for overlap_idx in (0..idx).into_iter().rev() {
                                    if intercepts[overlap_idx].start <= intercepts[idx].end && intercepts[overlap_idx].end >= intercepts[idx].start {
                                        to_remove.push(overlap_idx);
                                        intercepts[idx].start   = intercepts[idx].start.min(intercepts[overlap_idx].start);
                                        intercepts[idx].end     = intercepts[idx].end.max(intercepts[overlap_idx].end);
                                    } else {
                                        break;
                                    }
                                }

                                for remove_idx in to_remove.iter() {
                                    intercepts.remove(*remove_idx);
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        intercepts
    }
}

impl<'a, TDaub> SampledSignedDistanceField for &'a DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    type Contour = &'a DaubBrushDistanceField<TDaub>;

    #[inline]
    fn field_size(self) -> ContourSize {
        self.size
    }

    fn distance_at_point(self, pos: ContourPosition) -> f64 {
        // Distance is the minimum of all the daubs that overlap this point
        let mut distance = f64::MAX;

        for (daub, ContourPosition(x, y)) in self.daubs.iter() {
            // The daubs are sorted in order, so a daub that starts beyond the current point means that all the future daubs also start beyond that point
            if *y > pos.1 {
                break;
            }

            // Ignore daubs that occur before this position too
            if *x > pos.0 {
                continue;
            }

            // Check for overlap
            let ContourSize(w, h) = daub.field_size();
            if x+w <= pos.0 {
                continue;
            }
            if y+h <= pos.1 {
                continue;
            }

            // Fetch the distance from the daub
            let this_distance = daub.distance_at_point(ContourPosition(pos.0 - *x, pos.1 - *y));

            // The lowest distance of all the overlapping daubs is the distance for this point
            distance = f64::min(distance, this_distance);
        }

        distance
    }

    #[inline]
    fn as_contour(self) -> Self::Contour {
        self
    }
}

impl<'a, TDaub> DaubBrushContourIterator<'a, TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    ///
    /// Starts a new scanline by looking at the existing iterators, if there are any
    ///
    /// Will try to start on the current scanline if it can.
    ///
    /// If the 'current' iterator list is empty after this call, then there are no more edges in the contour
    ///
    fn start_scanline(&mut self) {
        use std::mem;

        // Move any existing edge iterators into the future iterator
        if !self.edge_iterators.is_empty() {
            self.future_scanline_iterators.extend(self.edge_iterators.drain(..));
        }

        loop {
            // If there's no more data to process, stop
            if self.edge_iterators.is_empty() && self.future_scanline_iterators.is_empty() && self.next_daub_idx >= self.distance_field.daubs.len() {
                return;
            }

            // Start iterating any daubs that are added by the current scanline (have a start y position at or before the current scanline)
            while self.next_daub_idx < self.distance_field.daubs.len() && self.distance_field.daubs[self.next_daub_idx].1.y() <= self.current_scanline {
                // Start the new iterator
                let contour             = self.distance_field.daubs[self.next_daub_idx].0.as_contour();
                let mut new_iterator    = contour.edge_cell_iterator();

                // Need the first cell to fill in the iterator
                if let Some(peek_cell) = new_iterator.next() {
                    let edge_iterator = EdgeIterator {
                        contour:        contour,
                        iterator:       new_iterator,
                        daub_position:  self.distance_field.daubs[self.next_daub_idx].1,
                        size:           self.distance_field.daubs[self.next_daub_idx].0.field_size(),
                        lookahead:      peek_cell,
                        finished:       false,
                    };

                    self.future_scanline_iterators.push(edge_iterator);
                }

                // Move on to the next daub
                self.next_daub_idx += 1;
            }

            // If any of the existing iterators or the future iterators are on the current scanline, then we're ready to return edges
            // We rely on each iterator returning edges in order from the top-left here: if an edge moves backwards (horizontally or vertically)
            // this algorithm will fail to produce good results
            let future_scanline_iterators = mem::take(&mut self.future_scanline_iterators);

            let (activated_edges, future_edges) = future_scanline_iterators.into_iter().partition::<Vec<_>, _>(|edge| {
                let ypos = edge.lookahead.0.y() + edge.daub_position.y();
                ypos == self.current_scanline
            });

            self.edge_iterators             = activated_edges;
            self.future_scanline_iterators  = future_edges;

            if !self.edge_iterators.is_empty() {
                // Order by start position, in reverse order (as we'll remove items left-to-right this reduces the amount of re-ordering done to the vec)
                self.edge_iterators.sort_by(|edge_a, edge_b| {
                    edge_b.daub_position.x().cmp(&edge_a.daub_position.x())
                });

                return;
            }

            // Pick a new scanline and try again: earliest in the current iterators or future daubs
            self.current_scanline += 1; // TODO: can probably look at the edges/future daubs and pick the smallest available y position instead of doing this
        }
    }

    ///
    /// Reads the value in a cell at a particular position
    ///
    fn read_overlapping_cell(&self, xpos: usize, ypos: usize) -> bool {
        let mut is_filled = false;

        // We assume that all iterators cover the y position as that should be guaranteed
        for edge_iterator in self.edge_iterators.iter().rev() {
            if edge_iterator.daub_position.x() > xpos {
                // All future daubs appear after 'xpos'
                break;
            }

            if xpos >= edge_iterator.daub_position.x() + edge_iterator.size.width() {
                // Does not overlap this daub
                continue;
            }

            let pos             = ContourPosition(xpos - edge_iterator.daub_position.x(), ypos - edge_iterator.daub_position.y());
            let contour_filled  = edge_iterator.contour.point_is_inside(pos);
            is_filled = is_filled || contour_filled;

            // Once the 'is_filled' flag is set it can't be unset
            if is_filled {
                break;
            }
        }

        return is_filled;
    }
}

impl<'a, TDaub> Iterator for DaubBrushContourIterator<'a, TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    type Item = (ContourPosition, ContourCell);

    fn next(&mut self) -> Option<(ContourPosition, ContourCell)> {
        loop {
            // Move to the next scanline if there are no iterators at present
            if self.edge_iterators.is_empty() {
                self.current_scanline += 1;
                self.start_scanline();

                // If there are still no iterators, we've fnished iterating the edges
                if self.edge_iterators.is_empty() {
                    return None;
                }
            }

            // The edge iterators are ordered backwards from the start, by start position, which lets us 
            // Search for the earliest position that we have and edge for
            let mut edge_iterators  = self.edge_iterators.iter().rev();
            let edge_iterator       = edge_iterators.next().unwrap();

            let mut earliest_x      = edge_iterator.lookahead.0.x() + edge_iterator.daub_position.x();

            for edge_iterator in edge_iterators {
                // Stop once we find an iterator that cannot overlap the position of the earliest xpos
                if edge_iterator.daub_position.x() > earliest_x {
                    break;
                }

                // Fetch the x position and update the earliest x position
                let xpos    = edge_iterator.lookahead.0.x() + edge_iterator.daub_position.x();
                earliest_x  = earliest_x.min(xpos);
            }

            // Generate a cell for this position by reading all of the contours that contain this position
            let tl = if self.current_scanline > 0 && earliest_x > 0 { self.read_overlapping_cell(earliest_x-1, self.current_scanline-1) } else { false };
            let tr = if self.current_scanline > 0 { self.read_overlapping_cell(earliest_x, self.current_scanline-1) } else { false };
            let bl = if earliest_x > 0 { self.read_overlapping_cell(earliest_x-1, self.current_scanline) } else { false };
            let br = self.read_overlapping_cell(earliest_x, self.current_scanline);

            // Advance any corresponding iterators, possibly moving them to the future or removing them
            let mut to_remove: SmallVec<[_; 4]> = smallvec![];

            for (idx, edge_iterator) in self.edge_iterators.iter_mut().enumerate().rev() {
                // Stop once we find an iterator that cannot overlap the position of the earliest xpos
                if edge_iterator.daub_position.x() > earliest_x {
                    break;
                }

                // Only advancing the edges that match with the point that we're reading
                if edge_iterator.lookahead.0.x() + edge_iterator.daub_position.x() != earliest_x {
                    continue;
                }

                // Advance this iterator
                if let Some(next_lookahead) = edge_iterator.iterator.next() {
                    // Update the lookahead
                    edge_iterator.lookahead = next_lookahead;

                    if edge_iterator.lookahead.0.y() + edge_iterator.daub_position.y() != self.current_scanline {
                        // This item has no more edges (or filled pixels) on the current scanline
                        to_remove.push(idx);
                    }
                } else {
                    // Remove this iterator
                    edge_iterator.finished = true;
                    to_remove.push(idx);
                }
            }

            // Remove/future the values in to_remove
            for idx in to_remove.into_iter() {
                // As the iterators are in reverse order these will also be in reverse order (so the index won't change due to removing an item)
                let edge_iterator = self.edge_iterators.remove(idx);

                if !edge_iterator.finished {
                    self.future_scanline_iterators.push(edge_iterator);
                }
            }

            // Skip entirely filled cells (we shouldn't find any entirely empty cells)
            if tl && tr && bl && br {
                continue;
            }

            // Return the cell we found
            return Some((ContourPosition(earliest_x, self.current_scanline), ContourCell::from_corners(tl, tr, bl, br)));
        }
    }
}

