use crate::geo::*;

use smallvec::*;

use std::cmp::{Ordering};

///
/// Sweeps a set of objects with bounding boxes to find the potential collisions between them
///
/// The objects must be sorted into order by their min-x position, with the lowest first
///
pub fn sweep_self<'a, TItem, BoundsIter>(ordered_items: BoundsIter) -> impl 'a+Iterator<Item=(&'a TItem, &'a TItem)>
where
BoundsIter:     'a+Iterator<Item=&'a TItem>,
TItem:          'a+HasBoundingBox,
TItem::Point:   Coordinate2D {
    SweepSelfIterator {
        bounds_iterator:    ordered_items,
        pending:            smallvec![],
        by_max_x:           Vec::new()
    }
}

///
/// Sweeps two sets of objects to find the collisions between them
///
/// This will only collide between objects in src and objects in tgt. Both must be sorted into order by
/// their min-x position, with the lowest first
///
pub fn sweep_against<'a, TItem, SrcBoundsIter, TgtBoundsIter>(src: SrcBoundsIter, tgt: TgtBoundsIter) -> impl 'a+Iterator<Item=(&'a TItem, &'a TItem)>
where
SrcBoundsIter:  'a+Iterator<Item=&'a TItem>,
TgtBoundsIter:  'a+Iterator<Item=&'a TItem>,
TItem:          'a+HasBoundingBox,
TItem::Point:   Coordinate2D {
    SweepAgainstIterator {
        src_iterator:   src,
        tgt_iterator:   tgt,
        pending:        smallvec![],
        src_by_max_x:   Vec::new(),
        tgt_by_max_x:   Vec::new()
    }
}

///
/// Iterator that performs the sweep operation
///
struct SweepSelfIterator<'a, TItem, BoundsIter>
where
BoundsIter:     'a+Iterator<Item=&'a TItem>,
TItem:          'a+HasBoundingBox,
TItem::Point:   Coordinate2D {
    /// Iterator, ordered by minimum X position, that returns the items to be checked for overlaps
    bounds_iterator: BoundsIter,

    /// Collided items that are pending a return
    pending: SmallVec<[(&'a TItem, &'a TItem); 16]>,

    /// Items currently under consideration for collisions, reverse ordered by their maximum X coordinate
    /// (reverse ordered so we can remove the earliest items by popping them)
    by_max_x: Vec<(Bounds<TItem::Point>, &'a TItem)>
}

impl<'a, TItem, BoundsIter> Iterator for SweepSelfIterator<'a, TItem, BoundsIter>
where
BoundsIter:     'a+Iterator<Item=&'a TItem>,
TItem:          'a+HasBoundingBox,
TItem::Point:   Coordinate2D {
    type Item = (&'a TItem, &'a TItem);

    fn next(&mut self) -> Option<Self::Item> {
        // If there is a pending item, return that
        if let Some(next) = self.pending.pop() {
            return Some(next);
        }

        // Attempt to fill the pending queue by reading from the bounds iterator
        loop {
            // Read the next item and retrieve its bounding box
            let next_item   = if let Some(next_item) = self.bounds_iterator.next() {
                next_item
            } else {
                // No more items to read, and the pending queue is empty
                return None;
            };

            // Fetch the bounding box
            let next_bounds = next_item.get_bounding_box::<Bounds<_>>();

            // Remove elements from the front of the by_max_x list until the closest ends after where this item begins
            // As the bounds_iterator is ordered by the min_x, we'll never see anything that's before this point again here
            let min_x       = next_bounds.min().x();
            while let Some((earliest_x, _item)) = self.by_max_x.last() {
                if earliest_x.max().x() >= min_x {
                    break;
                }

                self.by_max_x.pop();
            }

            // Check for collisions against the remaining items
            // TODO: having something ordered by 'y' coordinate too would be useful when most items overlap in the x-coordinate to further improve performance (but this complicates removing items once done)
            for (item_bounds, item) in self.by_max_x.iter() {
                if item_bounds.overlaps(&next_bounds) {
                    self.pending.push((item, next_item));
                }
            }

            // Insert the new item into the 'by_max_x' list
            // TODO: possible that something like a btree is much more efficient here when there are a lot of items to process
            let max_x   = next_bounds.max().x();
            let index   = self.by_max_x.binary_search_by(|(bounds, _item)| {
                let item_max_x = bounds.max().x();

                if item_max_x > max_x {
                    Ordering::Less
                } else if item_max_x == max_x {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }).unwrap_or_else(|idx| idx);

            self.by_max_x.insert(index, (next_bounds, next_item));

            // If there are now some pending items, return the first of them
            if let Some(next) = self.pending.pop() {
                return Some(next);
            }
        }
    }
}

///
/// Iterator that performs the sweep operation
///
struct SweepAgainstIterator<'a, TItem, SrcIterator, TgtIterator>
where
SrcIterator:    'a+Iterator<Item=&'a TItem>,
TgtIterator:    'a+Iterator<Item=&'a TItem>,
TItem:          'a+HasBoundingBox,
TItem::Point:   Coordinate2D {
    /// Iterator, ordered by minimum X position
    src_iterator: SrcIterator,

    /// Iterator, ordered by minimum X position, that returns the items to be checked for overlaps
    tgt_iterator: TgtIterator,

    /// Collided items that are pending a return
    pending: SmallVec<[(&'a TItem, &'a TItem); 16]>,

    /// Source items that have not yet been swept away, ordered by maximum x position (in reverse, so the next item to remove can be popped)
    src_by_max_x: Vec<(Bounds<TItem::Point>, &'a TItem)>,

    /// Target items that have npt yet been swept away, ordered by maximum x position (in reverse, so the next item to remove can be popped)
    tgt_by_max_x: Vec<(Bounds<TItem::Point>, &'a TItem)>
}

impl<'a, TItem, SrcIterator, TgtIterator> Iterator for SweepAgainstIterator<'a, TItem, SrcIterator, TgtIterator>
where
SrcIterator:    'a+Iterator<Item=&'a TItem>,
TgtIterator:    'a+Iterator<Item=&'a TItem>,
TItem:          'a+HasBoundingBox,
TItem::Point:   Coordinate2D {
    type Item = (&'a TItem, &'a TItem);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
