use super::ColumnSampledContour;
use super::distance_field::*;
use super::sampled_contour::*;

use smallvec::*;

use std::cell::{RefCell};
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
#[derive(Clone)]
pub struct DaubBrushDistanceField<TDaub> {
    /// The size of this distance field, sufficient to contain all of the 'daubs'
    size: ContourSize,

    /// The 'daubs' that make up the brush stroke, and where they appear on the canvas. This is stored sorted by Y position
    /// to allow scanning downwards to find which 'daubs' influence which points
    daubs: Vec<(TDaub, ContourPosition)>,

    /// Indexed by y position and sorted by initial x position, the daubs that are on each line within the size of the distance field
    daubs_for_line: Vec<Vec<usize>>,

    /// If we're using the distance field in column mode, the daubs that appear in each column of the distance field
    daubs_for_column: RefCell<Option<Vec<Vec<usize>>>>,
}

struct DaubPosition {
    pos: Range<usize>,

    covered_pixels: Range<usize>,
}

impl<TDaub> DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    ///
    /// Creates a daub brush distance field from a list of daubs and their positions
    ///
    pub fn from_daubs(daubs: impl IntoIterator<Item=(TDaub, ContourPosition)>) -> DaubBrushDistanceField<TDaub> {
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

        // Figure out which daubs are on each line
        let daubs_for_line = Self::create_daubs_for_positions(daubs.iter().map(|(daub, pos)| {
            DaubPosition {
                pos:            (pos.1)..(pos.1 + daub.field_size().height()),
                covered_pixels: (pos.0)..(pos.0 + daub.field_size().width())
            }
        }).collect(), size.height());
        let daubs_for_column    = RefCell::new(None);

        DaubBrushDistanceField {
            size, daubs, daubs_for_line, daubs_for_column
        }
    }

    ///
    /// Fills in the 'daubs for column' cache
    ///
    fn calculate_daubs_for_column(&self) -> Vec<Vec<usize>> {
        // Get the indexes of the daubs sorted by x position
        let mut idx_sorted_by_x = (0..self.daubs.len()).collect::<Vec<_>>();
        idx_sorted_by_x.sort_by_key(|idx| self.daubs[*idx].1.0);

        // Figure out the daubs on each column
        let mut daubs_for_column = Self::create_daubs_for_positions(idx_sorted_by_x.iter().map(|idx| {
            let (daub, pos) = &self.daubs[*idx];

            DaubPosition {
                pos:            (pos.0)..(pos.0 + daub.field_size().width()),
                covered_pixels: (pos.1)..(pos.1 + daub.field_size().height())
            }
        }).collect(), self.size.width());

        // Remap the indexes into the daubs array
        for row in daubs_for_column.iter_mut() {
            for column in row.iter_mut() {
                *column = idx_sorted_by_x[*column];
            }
        }

        // Cache the daubs
        daubs_for_column
    }

    ///
    /// Creates the cache of daubs for each line or column in this brush stroke
    ///
    fn create_daubs_for_positions(ordered_daub_positions: Vec<DaubPosition>, size: usize) -> Vec<Vec<usize>> {
        let mut daubs_for_line  = Vec::with_capacity(size);
        let mut pos             = 0;
        let mut next_daub       = 0;
        let mut current_line    = Vec::<usize>::new();

        loop {
            // Stop caching once we reach the end of the brush
            if pos >= size {
                break;
            }

            // Remove any daubs that end before the current line
            current_line.retain(|daub_idx| ordered_daub_positions[*daub_idx].pos.end > pos);

            // Add any daubs that first appear at the current y position
            let mut new_daubs = vec![];

            while next_daub < ordered_daub_positions.len() && ordered_daub_positions[next_daub].pos.start == pos {
                new_daubs.push(next_daub);
                next_daub += 1;
            }

            // Order by x index
            new_daubs.sort_by(|a, b| ordered_daub_positions[*a].covered_pixels.start.cmp(&ordered_daub_positions[*b].covered_pixels.start));

            if current_line.len() == 0 {
                current_line = new_daubs;
            } else if new_daubs.len() > 0 {
                // Merge the daubs into one line
                let mut new_current_line    = vec![];
                let mut current_iter        = current_line.into_iter();
                let mut new_iter            = new_daubs.into_iter();

                let mut current_next        = current_iter.next();
                let mut new_next            = new_iter.next();
                
                loop {
                    match (current_next, new_next) {
                        (Some(current_idx), Some(new_idx)) => {
                            let current_x   = ordered_daub_positions[current_idx].covered_pixels.start;
                            let new_x       = ordered_daub_positions[new_idx].covered_pixels.start;

                            if current_x < new_x {
                                new_current_line.push(current_idx);
                                current_next = current_iter.next();
                            } else {
                                new_current_line.push(new_idx);
                                new_next = new_iter.next();
                            }
                        }

                        (Some(current_idx), None) => {
                            new_current_line.push(current_idx);
                            current_next = current_iter.next();
                        }

                        (None, Some(new_idx)) => {
                            new_current_line.push(new_idx);
                            new_next = new_iter.next();
                        }

                        (None, None) => { break; }
                    }
                }

                current_line = new_current_line;
            }

            // Add the daub indexes for the row/column to the results
            daubs_for_line.push(current_line.clone());

            // Prepare to process the next set of daubs
            pos += 1;
        }

        daubs_for_line
    }

}

impl<TDaub> SampledContour for DaubBrushDistanceField<TDaub> 
where
    TDaub: SampledSignedDistanceField,
{
    #[inline]
    fn contour_size(&self) -> ContourSize {
        self.size
    }

    fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        let mut intercepts: SmallVec<[Range<f64>; 4]> = smallvec![];

        // Fetch the daubs at this y position
        if y >= 0.0 && y < self.size.1 as f64 {
            let line_daubs = &self.daubs_for_line[y.floor() as usize];

            // Scan the intercepts left-to-right to build up the intercepts on this line
            let mut to_remove = vec![];

            for daub_idx in line_daubs.iter().copied() {
                let (daub, pos) = &self.daubs[daub_idx];
                let posx        = pos.0 as f64;
                let posy        = pos.1 as f64;

                for intercept in daub.as_contour().intercepts_on_line(y - posy).into_iter() {
                    // Strip empty ranges if they occur
                    if intercept.start >= intercept.end { continue; }

                    // Offset the intercept by the position of this daub
                    let intercept = (posx + intercept.start)..(posx + intercept.end);

                    // In general, intercepts move left to right, so we should overlap the end of vec in general
                    if intercepts.len() == 0 {
                        // First intercept
                        intercepts.push(intercept);
                    } else if intercepts[intercepts.len()-1].end.floor() < intercept.start.ceil() {
                        // Beyond the end of the last intercept
                        intercepts.push(intercept);
                    } else {
                        // Might overlap one of the intercepts towards the end of the list
                        let mut found_overlap = false;
                        let overlap_intercept = intercept.start..intercept.end;

                        for idx in (0..intercepts.len()).into_iter().rev() {
                            if intercepts[idx].end.floor() < intercept.start.ceil() {
                                // All the remaining ranges are before the start of this one
                                intercepts.insert(idx+1, intercept);

                                found_overlap = true;
                                break;
                            } else if intercepts[idx].start.floor() <= intercept.end.ceil() && intercepts[idx].end.ceil() >= intercept.start.floor() {
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

                                found_overlap = true;
                                break;
                            }
                        }

                        if !found_overlap {
                            // Intercept must be at the start of the list
                            intercepts.insert(0, overlap_intercept);
                        }
                    }
                }
            }
        }

        intercepts
    }
}

impl<TDaub> ColumnSampledContour for DaubBrushDistanceField<TDaub> 
where
    TDaub:          SampledSignedDistanceField,
    TDaub::Contour: ColumnSampledContour,
{
    fn intercepts_on_column(&self, x: f64) -> SmallVec<[Range<f64>; 4]> {
        let mut intercepts: SmallVec<[Range<f64>; 4]> = smallvec![];

        // Fetch the daubs at this y position
        if x >= 0.0 && x < self.size.width() as f64 {
            let mut daubs_for_column = self.daubs_for_column.borrow_mut();
            let daubs_for_column = if let Some(daubs_for_column) = &*daubs_for_column {
                daubs_for_column
            } else {
                *daubs_for_column = Some(self.calculate_daubs_for_column());
                daubs_for_column.as_ref().unwrap()
            };

            let line_daubs = &daubs_for_column[x.floor() as usize];

            // Scan the intercepts left-to-right to build up the intercepts on this line
            let mut to_remove = vec![];

            for daub_idx in line_daubs.iter().copied() {
                let (daub, pos) = &self.daubs[daub_idx];
                let posx        = pos.0 as f64;
                let posy        = pos.1 as f64;

                for intercept in daub.as_contour().intercepts_on_column(x - posx).into_iter() {
                    // Strip empty ranges if they occur
                    if intercept.start >= intercept.end { continue; }

                    // Offset the intercept by the position of this daub
                    let intercept = (posy + intercept.start)..(posy + intercept.end);

                    // In general, intercepts move left to right, so we should overlap the end of vec in general
                    if intercepts.len() == 0 {
                        // First intercept
                        intercepts.push(intercept);
                    } else if intercepts[intercepts.len()-1].end.floor() < intercept.start.ceil() {
                        // Beyond the end of the last intercept
                        intercepts.push(intercept);
                    } else {
                        // Might overlap one of the intercepts towards the end of the list
                        let mut found_overlap = false;
                        let overlap_intercept = intercept.start..intercept.end;

                        for idx in (0..intercepts.len()).into_iter().rev() {
                            if intercepts[idx].end.floor() < intercept.start.ceil() {
                                // All the remaining ranges are before the start of this one
                                intercepts.insert(idx+1, intercept);

                                found_overlap = true;
                                break;
                            } else if intercepts[idx].start.floor() <= intercept.end.ceil() && intercepts[idx].end.ceil() >= intercept.start.floor() {
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

                                found_overlap = true;
                                break;
                            }
                        }

                        if !found_overlap {
                            // Intercept must be at the start of the list
                            intercepts.insert(0, overlap_intercept);
                        }
                    }
                }
            }
        }

        intercepts
    }
}

impl<TDaub> SampledSignedDistanceField for DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    type Contour = DaubBrushDistanceField<TDaub>;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.size
    }

    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        // Distance is the minimum of all the daubs that overlap this point
        let mut distance = f64::MAX;

        if pos.1 < self.size.1 {
            for daub_idx in self.daubs_for_line[pos.1].iter().copied() {
                let (daub, ContourPosition(x, y)) = &self.daubs[daub_idx];

                // Ignore daubs that occur after the position we're interested in
                if *x > pos.0 {
                    break;
                }

                // Check for overlap
                let ContourSize(w, _) = daub.field_size();
                if x+w <= pos.0 {
                    continue;
                }

                // Fetch the distance from the daub
                let this_distance = daub.distance_at_point(ContourPosition(pos.0 - *x, pos.1 - *y));

                // The lowest distance of all the overlapping daubs is the distance for this point
                distance = f64::min(distance, this_distance);
            }
        }

        distance
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        self
    }
}
