use crate::geo::*;

use smallvec::*;

use std::ops::{Range};

///
/// The size of a bitmap contour (in the form width, height)
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ContourSize(pub usize, pub usize);

///
/// An x,y coordinate within a contour bitmap
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ContourPosition(pub usize, pub usize);

///
/// Represents a 'cell' in a contour bitmap, a 2x2 square of samples
///
/// The value stored in this cell is a value from 0-15, where each bit represents one of the four corners of the cell:
///
///  * Bit 0 = top-left
///  * Bit 1 = top-right
///  * Bit 2 = bottom-left
///  * Bit 3 = bottom-right
///
/// A y value of 0 is considered to be the top of the bitmap
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ContourCell(pub (crate) u8);

///
/// Represents the midpoint of an edge in a contour bitmap.
///
/// Edges are represented as a number where the lowest bit indicates if it's a horizontal or vertical edge, then
/// counting from the top left, 'edge 0' is between samples (0,0) and (0,1), 'edge 1' between (0,1) and (0,2) and
/// so on.
///
/// For vertical edges, the coordinates are counted horizontally still, so 'edge 0' is (0,0) and (1, 0), edge 1 is
/// between (1,0) and (1,1) and so on.
///
/// Note that there is one less horizontal edge than there are samples across the contour, so there's one 'unused'
/// edge per row (similarly, the last set of horizontal edges is not followed by a set of vertical edges)
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ContourEdge(pub (crate) usize);

///
/// Represents a contour stored as samples at integer coordinates, where each sample can either be within the shape (1) or outside of the shape (0)
///
/// Implement this trait on a reference to a storage type rather than the type itself
///
pub trait SampledContour : Copy {
    /// Iterator that visits all of the cells in this contour
    type EdgeCellIterator: Iterator<Item=(ContourPosition, ContourCell)>;

    ///
    /// The size of this contour
    ///
    fn contour_size(self) -> ContourSize;

    ///
    /// Returns true if the specified point is inside the contour, or false if it's outside
    ///
    /// A y-value of 0 is considered to be the 'top' of the bitmap
    ///
    fn point_is_inside(self, pos: ContourPosition) -> bool;

    ///
    /// Returns an iterator that visits all of the cells that are on an edge (has at least one set and one unset bit in the ContourCell)
    /// starting from the top-left corner of the contour
    ///
    /// The position returned here is the position of the bottom-right corner of the cell containing the edge.
    ///
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator;

    ///
    /// Given a y coordinate returns ranges indicating the filled pixels on that line
    ///
    /// The ranges must be provided in ascending order, and must also not overlap.
    ///
    fn intercepts_on_line(self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        // Default implementation is a simple scan of the line: mathematically defined contours might use a ray-casting algorithm here instead
        let width   = self.contour_size().width();
        let y       = y.floor() as usize;

        let mut ranges = smallvec![];
        let mut inside = None;

        for x in 0..width {
            // Transitioning from 'outside' to 'inside' sets a start position, and doing the opposite generates a range
            match (inside, self.point_is_inside(ContourPosition(x, y))) {
                (None, true)            => { inside = Some(x); },
                (Some(start_x), false)  => {
                    inside = None;
                    ranges.push((start_x as f64)..(x as f64));
                }
                _ => { }
            }
        }

        if let Some(start_x) = inside {
            ranges.push((start_x as f64)..(width as f64));
        }

        ranges
    }
}

impl ContourCell {
    ///
    /// Returns a cell made up of 4 corner values (top-left, top-right, bottom-left and bottom-right)
    ///
    #[inline]
    pub const fn from_corners(tl: bool, tr: bool, bl: bool, br: bool) -> ContourCell {
        let tl = if tl { 1 } else { 0 };
        let tr = if tr { 2 } else { 0 };
        let bl = if bl { 4 } else { 0 };
        let br = if br { 8 } else { 0 };

        ContourCell(tl | tr | bl | br)
    }

    ///
    /// True if this represents a cell on the edge of the shape
    ///
    #[inline]
    pub const fn is_on_edge(&self) -> bool {
        self.0 != 0 && self.0 != 15
    }

    ///
    /// Merge this cell with another cell to create a cell with all the corners selected
    ///
    #[inline]
    pub const fn merge(self, cell: ContourCell) -> ContourCell {
        ContourCell(self.0 | cell.0)
    }

    ///
    /// Returns this cell shifted one pixel to the left
    ///
    #[inline]
    pub const fn shift_left(self) -> ContourCell {
        ContourCell((self.0 >> 1) & !2)
    }

    ///
    /// Returns true if the cell is empty
    ///
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    ///
    /// Returns true if the cell is full
    ///
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.0 == 15
    }
}

impl ContourEdge {
    #[inline] pub (crate) const fn top()      -> ContourEdge { ContourEdge(1) }
    #[inline] pub (crate) const fn left()     -> ContourEdge { ContourEdge(0) }
    #[inline] pub (crate) const fn bottom()   -> ContourEdge { ContourEdge(3) }   // Assuming a 1x1 sample size
    #[inline] pub (crate) const fn right()    -> ContourEdge { ContourEdge(2) }   // Assuming a 1x1 sample size

    #[inline] pub (crate) const fn at_coordinates(self, size: ContourSize, pos: ContourPosition) -> ContourEdge {
        // Offset is calculated from the size and the position
        let edge_width  = size.0 + 1;
        let offset      = edge_width * pos.1 + pos.0;

        // This can either be the left or the right cell depending on the upper bit
        let offset = match self.0 {
            0 => offset,            // left
            1 => offset,            // top
            2 => offset + 1,        // right
            3 => offset + edge_width,   // bottom
            _ => unreachable!()
        };

        // This can be the horizontal or vertical edge depending on the lower bit
        let offset = (offset<<1) | (self.0&1);

        ContourEdge(offset)
    }

    ///
    /// Returns the coordinates of the samples in the original `SampledContour` for this edge
    ///
    #[inline]
    pub fn to_contour_coords(self, size: ContourSize) -> (ContourPosition, ContourPosition) {
        let edge_width  = size.0 + 1;
        let x           = (self.0 >> 1) % edge_width;
        let y           = (self.0 >> 1) / edge_width;

        if (self.0&1) == 1 {
            // Horizontal edge
            (ContourPosition(x, y), ContourPosition(x+1, y))
        } else {
            // Vertical edge
            (ContourPosition(x, y), ContourPosition(x, y+1))
        }
    }

    ///
    /// Returns the coordinate of the center point of this edge
    ///
    #[inline]
    pub fn to_coords<TCoord>(self, size: ContourSize) -> TCoord
    where
        TCoord: Coordinate + Coordinate2D,
    {
        let edge_width  = size.0 + 1;
        let x           = (self.0 >> 1) % edge_width;
        let y           = (self.0 >> 1) / edge_width;

        if (self.0&1) == 1 {
            // Horizontal edge
            TCoord::from_components(&[x as f64 + 0.5, y as f64])
        } else {
            // Vertical edge
            TCoord::from_components(&[x as f64, y as f64 + 0.5])
        }
    }
}

impl ContourPosition {
    #[inline]
    pub fn x(&self) -> usize { self.0 }

    #[inline]
    pub fn y(&self) -> usize { self.1 }
}

impl ContourSize {
    #[inline]
    pub fn width(&self) -> usize { self.0 }

    #[inline]
    pub fn height(&self) -> usize { self.1 }
}

///
/// Iterator that returns the edge cells in a bitmap contour by calling `point_is_inside` for the cells
///
pub struct SimpleEdgeCellIterator<TContour>
where
    TContour: SampledContour,
{
    last_is_inside: (bool, bool),
    contour_size:   (usize, usize),
    pos:            (usize, usize),
    contour:        TContour
}

impl<TContour> SimpleEdgeCellIterator<TContour> 
where
    TContour: SampledContour,
{
    ///
    /// Cretes a new iterator that will return the edge cells in the specified contour
    ///
    #[inline]
    pub fn from_contour(contour: TContour) -> Self {
        let ContourSize(size_x, size_y) = contour.contour_size();

        SimpleEdgeCellIterator {
            contour_size:   (size_x, size_y),
            last_is_inside: (false, false),
            pos:            (0, 0),
            contour:        contour,
        }
    }
}

impl<TContour> Iterator for SimpleEdgeCellIterator<TContour> 
where
    TContour: SampledContour,
{
    type Item = (ContourPosition, ContourCell);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Loop until we find a cell that's on the edge
        loop {
            let (size_x, size_y)    = self.contour_size;
            let (pos_x, pos_y)      = self.pos;

            // Finished once we go beyond the end of the contour
            if pos_y > size_y {
                return None;
            }

            // We store the 'top-left, bottom-left' values in our state and the pos indicates the 'bottom-right' value
            let (tl, bl) = self.last_is_inside;

            // The 'top-right' and 'bottom-right' values need to be fetched from the contour
            let (tr, br) = if pos_x >= size_x {
                (false, false)
            } else {
                let br = if pos_y >= size_y {
                    false
                } else {
                    self.contour.point_is_inside(ContourPosition(pos_x, pos_y))
                };

                let tr = if pos_y == 0 {
                    false
                } else {
                    self.contour.point_is_inside(ContourPosition(pos_x, pos_y-1))
                };

                (tr, br)
            };

            // The cell for this position consists of all 4 values
            let cell = ContourCell::from_corners(tl, tr, bl, br);

            // Move to the next position
            self.pos.0          += 1;
            self.last_is_inside = (tr, br);

            if self.pos.0 > size_x {
                self.pos.0 = 0;
                self.pos.1 += 1;
                self.last_is_inside = (false, false);
            }

            // Return a value if this cell is on the edge of the contour
            if cell.is_on_edge() {
                return Some((ContourPosition(pos_x, pos_y), cell));
            }
        }
    }
}

///
/// Represents a contour sampled using boolean values that indicate whether or not each sample is in or out
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolSampledContour(pub ContourSize, pub Vec<bool>);

///
/// Represents a contour sampled using u8 values that are 0 for items outside the contour and 1 otherwise
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct U8SampledContour(pub ContourSize, pub Vec<u8>);

impl<'a> SampledContour for &'a BoolSampledContour {
    /// Iterator that visits all of the cells in this contour
    type EdgeCellIterator = SimpleEdgeCellIterator<Self>;

    ///
    /// The size of this contour
    ///
    #[inline]
    fn contour_size(self) -> ContourSize {
        self.0
    }

    ///
    /// Returns true if the specified point is inside the contour, or false if it's outside
    ///
    /// A y-value of 0 is considered to be the 'top' of the bitmap
    ///
    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        // Position as an offset into the vector array, without bounds checking
        let idx = pos.0 + (pos.1) * self.0.0;

        self.1[idx]
    }

    ///
    /// Returns an iterator that visits all of the cells that are on an edge (has at least one set and one unset bit in the ContourCell)
    ///
    /// The position returned here is the position of the bottom-right corner of the cell.
    ///
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        SimpleEdgeCellIterator::from_contour(self)
    }
}

impl<'a> SampledContour for &'a U8SampledContour {
    /// Iterator that visits all of the cells in this contour
    type EdgeCellIterator = SimpleEdgeCellIterator<Self>;

    ///
    /// The size of this contour
    ///
    #[inline]
    fn contour_size(self) -> ContourSize {
        self.0
    }

    ///
    /// Returns true if the specified point is inside the contour, or false if it's outside
    ///
    /// A y-value of 0 is considered to be the 'top' of the bitmap
    ///
    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        // Position as an offset into the vector array, without bounds checking
        let idx = pos.0 + (pos.1) * self.0.0;

        self.1[idx] != 0
    }

    ///
    /// Returns an iterator that visits all of the cells that are on an edge (has at least one set and one unset bit in the ContourCell)
    ///
    /// The position returned here is the position of the bottom-right corner of the cell.
    ///
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        SimpleEdgeCellIterator::from_contour(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn left_at_coordinate() {
        let size            = ContourSize(80, 80);
        let left            = ContourEdge::left();
        let at_coord        = left.at_coordinates(size, ContourPosition(7, 8));
        let (start, end)    = at_coord.to_contour_coords(size);

        assert!(start == ContourPosition(7, 8), "Start doesn't match {:?} {:?}", start, end);
        assert!(end == ContourPosition(7, 9), "End doesn't match {:?} {:?}", start, end);
    }

    #[test]
    fn right_at_coordinate() {
        let size            = ContourSize(80, 80);
        let right           = ContourEdge::right();
        let at_coord        = right.at_coordinates(size, ContourPosition(7, 8));
        let (start, end)    = at_coord.to_contour_coords(size);

        assert!(start == ContourPosition(8, 8), "Start doesn't match {:?} {:?}", start, end);
        assert!(end == ContourPosition(8, 9), "End doesn't match {:?} {:?}", start, end);
    }

    #[test]
    fn top_at_coordinate() {
        let size            = ContourSize(80, 80);
        let top             = ContourEdge::top();
        let at_coord        = top.at_coordinates(size, ContourPosition(7, 8));
        let (start, end)    = at_coord.to_contour_coords(size);

        assert!(start == ContourPosition(7, 8), "Start doesn't match {:?} {:?}", start, end);
        assert!(end == ContourPosition(8, 8), "End doesn't match {:?} {:?}", start, end);
    }

    #[test]
    fn bottom_at_coordinate() {
        let size            = ContourSize(80, 80);
        let bottom          = ContourEdge::bottom();
        let at_coord        = bottom.at_coordinates(size, ContourPosition(7, 8));
        let (start, end)    = at_coord.to_contour_coords(size);

        assert!(start == ContourPosition(7, 9), "Start doesn't match {:?} {:?}", start, end);
        assert!(end == ContourPosition(8, 9), "End doesn't match {:?} {:?}", start, end);
    }
}
