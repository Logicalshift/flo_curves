use super::sampled_contour::*;

use smallvec::*;

use std::mem;

///
/// Iterator that takes a set of contour edges ordered by y position and groups them by their y position
///
/// The iterators returned from `SampledContour::edge_cell_iterator()` should be in a suitable ordering to be
/// transformed by this iterator.
///
pub struct EdgesByScanlineIterator<TIterator>
where
    TIterator: Iterator<Item=(ContourPosition, ContourCell)>,
{
    // The contour iterator
    iterator: TIterator,

    /// The current scanline identifier (None if all of the edges have been generated)
    current_scanline: Option<usize>,

    /// The cells in the current scanline
    scanline_cells: SmallVec<[(usize, ContourCell); 4]>,
}

impl<TIterator> From<TIterator> for EdgesByScanlineIterator<TIterator>
where
    TIterator: Iterator<Item=(ContourPosition, ContourCell)>,
{
    fn from(iterator: TIterator) -> EdgesByScanlineIterator<TIterator> {
        let mut iterator = iterator;

        if let Some((first_pos, first_cell)) = iterator.next() {
            EdgesByScanlineIterator {
                iterator:           iterator,
                current_scanline:   Some(first_pos.1),
                scanline_cells:     smallvec![(first_pos.0, first_cell)],
            }
        } else {
            EdgesByScanlineIterator {
                iterator:           iterator,
                current_scanline:   None,
                scanline_cells:     smallvec![],
            }
        }
    }
}

impl<TIterator> Iterator for EdgesByScanlineIterator<TIterator>
where
    TIterator: Iterator<Item=(ContourPosition, ContourCell)>,
{
    type Item = (usize, SmallVec<[(usize, ContourCell); 4]>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_scanline) = self.current_scanline {
            // Fetch the cells in the current scanline
            let mut cells = smallvec![];
            mem::swap(&mut self.scanline_cells, &mut cells);

            // Read cells until we reach the end of the current scanline
            loop {
                if let Some((next_pos, next_cell)) = self.iterator.next() {
                    // Store the current value in the next scanline and return once we run out of cells on the old scanline
                    if next_pos.1 != current_scanline {
                        self.current_scanline = Some(next_pos.1);
                        self.scanline_cells.push((next_pos.0, next_cell));

                        return Some((current_scanline, cells));
                    } else {
                        // Add to the current scanline and continue reading
                        cells.push((next_pos.0, next_cell));
                    }
                } else {
                    // Won't return any more entries
                    self.current_scanline = None;

                    // Return the last scanline (there should always be at least one cell per scaline)
                    return Some((current_scanline, cells));
                }
            }
        } else {
            // No more items to return
            None
        }
    }
}