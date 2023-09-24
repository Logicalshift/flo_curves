use std::ops::{Range};

use smallvec::*;

///
/// Spatial data structure that allows addressing data by where it's located in a 1-dimensional space
///
#[derive(Clone)]
pub struct Space1D<TData> {
    /// The data stored in this structure
    values: Vec<TData>,

    /// Which data items are located where, sorted into order. The ranges are non-overlapping, so the same data item can be stored multiple times
    space: Vec<(Range<f64>, SmallVec<[usize; 2]>)>,
}

impl<TData> Space1D<TData> {
    ///
    /// Creates a new empty space
    ///
    pub fn empty() -> Self {
        Space1D {
            values:     vec![],
            space:      vec![],
        }
    }

    ///
    /// Creates a new space from a data iterator
    ///
    pub fn from_data(data: impl IntoIterator<Item=(Range<f64>, TData)>) -> Self {
        use std::mem;

        // Create the data structures
        let mut values              = vec![];
        let mut locations           = vec![];
        let mut overlapping_space   = vec![];

        // Save the initial set of data
        for (location, data) in data {
            let idx = values.len();

            values.push(data);
            locations.push(location.clone());
            overlapping_space.push((location, idx));
        }

        // Order the space by start position
        overlapping_space.sort_by(|a, b| a.0.start.total_cmp(&b.0.start));

        // Combine the spaces so they no longer overlap
        let mut combined_space: Vec<(Range<f64>, SmallVec<[usize; 2]>)> = vec![];

        // 'remaining' is a stack of ranges that have not yet been added to the result because they may overlap with the ranges we haven't inspected yet, sorted by end position in reverse order and split so that they don't overlap
        let mut remaining: Vec<(Range<f64>, SmallVec<[usize; 2]>)>      = vec![];
        let mut new_remaining: Vec<(Range<f64>, SmallVec<[usize; 2]>)>  = vec![];

        for (range, handle) in overlapping_space.into_iter() {
            // Because overlapping_space is ordered in start coordinate order, there can be no range inside 'remaining' that starts after this point (ie, everything that remains is either before or overlapping this range)

            // Pop and combine the values from the remaining list that end before the start of the current range (because they are non-overlapping, end and start positions are equivalent for ordering purposes)
            while let Some((last_range, last_handles)) = remaining.last() {
                if last_range.end > range.start {
                    // This range overlaps the new range
                    break;
                }

                // As the new range is beyond this range, there can't be any more ranges overlapping it, so we can add it to the result
                if let Some((range, handles)) = combined_space.last_mut() {
                    if range.start == last_range.start {
                        handles.extend(last_handles.iter().copied());
                    } else {
                        combined_space.push((last_range.clone(), last_handles.iter().copied().collect()));
                    }
                } else {
                    combined_space.push((last_range.clone(), last_handles.iter().copied().collect()));
                }

                remaining.pop();
            }

            // We'll update the range to be the remaining portion
            let mut range = range;
            new_remaining.clear();

            // Cut off any items that overlap but start before this range
            for (mut active_range, mut active_handles) in remaining.drain(..).rev() {
                if active_range.end > range.start && active_range.start < range.start {
                    // Push the part of the range that is before the current range to the result
                    combined_space.push((active_range.start..range.start, active_handles.clone()));
                    active_range.start = range.start;
                } 

                // This range overlaps the current range at the start (because remaining is ordered by end point)
                if range.start == range.end {
                    // Just move into the remaining list (the current range is consumed)
                    new_remaining.push((active_range, active_handles));
                } else if active_range.end <= range.end {
                    // The range is entirely consumed by the current range (which overlaps this region)
                    active_handles.push(handle);

                    // Remaining part of the current range starts after this range
                    range.start = active_range.end;

                    // Add to the result
                    new_remaining.push((active_range, active_handles));
                } else {
                    // The new range is entirely consumed by the remaining section
                    let mut range_handles = active_handles.clone();
                    range_handles.push(handle);

                    new_remaining.push((range.clone(), range_handles));
                    range.start = range.end;

                    // Consume the part of the active range that overlapped the new range 
                    active_range.start = range.end;
                    new_remaining.push((active_range, active_handles));
                }
            }

            if range.start != range.end {
                // This range has a region that overlaps nothing
                new_remaining.push((range, smallvec![handle]));
            }

            // We've got the 'in flight' ranges for the next iteration
            mem::swap(&mut remaining, &mut new_remaining);
            if remaining.len() > 1 {
                // Reverse so that the item on top of the 'remaining' list is the first to be removed
                remaining.reverse();
            }
        }

        // Add the other remaining ranges to the list
        combined_space.extend(remaining.drain(..).rev());

        Space1D {
            values: values,
            space:  combined_space,
        }
    }

    ///
    /// Returns the first point that overlaps a point
    ///
    #[inline]
    fn search(&self, point: f64) -> Result<usize, usize> {
        match self.space.binary_search_by(|(range, _)| range.start.total_cmp(&point)) 
        {
            Ok(idx)     => Ok(idx),
            Err(idx)    => {
                // idx = first range starting after this point
                if idx == 0 {
                    Err(0)
                } else {
                    let possible_range = &self.space[idx-1].0;
                    if possible_range.end > point {
                        // idx-1 contains this point
                        Ok(idx-1)
                    } else {
                        // idx is the first range 
                        Err(idx)
                    }
                }
            }
        }
    }

    ///
    /// Returns the data items that are at a particular point
    ///
    #[inline]
    pub fn data_at_point<'a>(&'a self, point: f64) -> impl 'a + Iterator<Item=&'a TData> {
        if let Ok(idx) = self.search(point) {
            // Found a range matching this point
            let data = self.space[idx].1.iter()
                .map(move |handle| &self.values[*handle]);

            Some(data).into_iter().flatten()
        } else {
            // No ranges match this point
            None.into_iter().flatten()
        }
    }

    ///
    /// Finds all of the data that overlaps a particular range
    ///
    pub fn data_in_region<'a>(&'a self, range: Range<f64>) -> impl 'a + Iterator<Item=&'a TData> {
        // Search for the first ares that is covered by this range
        let mut idx = match self.search(range.start) { Ok(idx) => idx, Err(idx) => idx };

        // Create the result in a vec, use a bit field to indicate which values are used
        let num_words           = (self.values.len() / 64) + 1;
        let mut used_handles    = vec![0u64; num_words];
        let mut result          = vec![];

        loop {
            // Stop once we get to the end of the space
            if idx >= self.space.len() { break; }

            let (space_range, handles) = &self.space[idx];

            // Also stop once we've covered all of the ranges
            if space_range.start >= range.end {
                break;
            }

            // Add any data we haven't seen before to the result
            for handle in handles.iter().copied() {
                let word    = handle >> 6;
                let bit     = handle & 63;
                let mask    = 1u64<<bit;

                if used_handles[word]&mask == 0 {
                    result.push(&self.values[handle]);
                    used_handles[word] |= mask;
                }
            }

            // Move to the next range
            idx += 1;
        }

        result.into_iter()
    }

    ///
    /// Returns the non-overlapping regions and data that makes up this space (data may appear multiple times if it's in several regions)
    ///
    #[inline]
    pub fn all_regions<'a>(&'a self) -> impl 'a + Iterator<Item=(Range<f64>, SmallVec<[&'a TData; 2]>)> {
        self.space.iter()
            .map(move |(range, handles)| (range.clone(), handles.iter().map(|handle| &self.values[*handle]).collect()))
    }

    ///
    /// Returns the non-overlapping regions and data that makes up this space (data may appear multiple times if it's in several regions)
    ///
    #[inline]
    pub fn regions_in_range<'a>(&'a self, range: Range<f64>) -> impl 'a + Iterator<Item=(Range<f64>, SmallVec<[&'a TData; 2]>)> {
        let start_idx = match self.search(range.start) { Ok(idx) => idx, Err(idx) => idx };

        self.space.iter()
            .skip(start_idx)
            .take_while(move |(space_range, _)| space_range.start < range.end)
            .map(move |(range, handles)| (range.clone(), handles.iter().map(|handle| &self.values[*handle]).collect()))
    }

    ///
    /// Checks that the representation of the space within this object is valid
    ///
    #[cfg(test)]
    pub fn verify(&self) {
        // Nothing to do if the space is so short it can't be incorrect
        if self.space.len() <= 1 { return; }

        // Ranges must not overlap or go backwards
        let mut last_range = self.space[0].0.clone();
        assert!(last_range.start != last_range.end, "First range has 0 length {:?}", last_range);

        for (range, _) in self.space.iter().skip(1) {
            assert!(range.start >= last_range.end, "New range starts before last range ends ({:?} vs {:?})", last_range, range);
            assert!(range.start != range.end, "Range has 0 length ({:?} vs {:?})", last_range, range);

            last_range = range.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn random() {
        let mut rng = StdRng::from_seed([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);

        for _ in 0..2000 {
            let num_sections    = rng.gen_range(10..100);
            let sections        = (0..num_sections)
                .map(|section| {
                    let start = rng.gen_range(0.0..100.0);
                    let len = rng.gen_range(0.0..100.0);

                    (start..(start+len), section)
                })
                .collect::<Vec<_>>();
            let space = Space1D::from_data(sections.iter().cloned());

            space.verify();
            assert!(space.space.len() >= num_sections);

            for (range, idx) in sections.iter() {
                let data_at_mid = space.data_at_point((range.start + range.end)/2.0).collect::<Vec<_>>();

                assert!(data_at_mid.len() > 0, "No data found at middle of {:?}", range);
                assert!(data_at_mid.contains(&idx), "Could not find section index {} for middle of range {:?} (found {:?} instead)", idx, range, data_at_mid);

                let data_at_end = space.data_at_point(range.end - 0.001).collect::<Vec<_>>();

                assert!(data_at_end.len() > 0, "No data found at end of {:?}", range);
                assert!(data_at_end.contains(&idx), "Could not find section index {} for end of range {:?} (found {:?} instead)", idx, range, data_at_mid);

                let regions_in_range = space.regions_in_range(range.clone()).collect::<Vec<_>>();
                assert!(regions_in_range.iter().all(|(_, handles)| handles.contains(&idx)), "Over the whole range, found a section that's missing this handle ({}: {:?})", idx, regions_in_range);
            }

            let all_data = space.data_in_region(0.0..201.0).collect::<Vec<_>>();
            assert!(all_data.len() == num_sections, "Retrieved {} items for the whole range (should have been {}) - {:?}", all_data.len(), num_sections, all_data);
            assert!((0..num_sections).all(|section_id| all_data.contains(&&section_id)));
        }
    }
}
