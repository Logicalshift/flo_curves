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
    fn size(self) -> ContourSize;

    ///
    /// Returns true if the specified point is inside the contour, or false if it's outside
    ///
    /// A y-value of 0 is considered to be the 'top' of the bitmap
    ///
    fn point_is_inside(self, pos: ContourPosition) -> bool;

    ///
    /// Returns an iterator that visits all of the cells that are on an edge (has at least one set and one unset bit in the ContourCell)
    ///
    /// The position returned here is the position of the bottom-right corner of the cell.
    ///
    fn edge_cell_iterator(self) -> Self::EdgeCellIterator;
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
}

impl ContourEdge {
    #[inline] pub (crate) const fn top()      -> ContourEdge { ContourEdge(1) }
    #[inline] pub (crate) const fn left()     -> ContourEdge { ContourEdge(0) }
    #[inline] pub (crate) const fn bottom()   -> ContourEdge { ContourEdge(3) }   // Assuming a 1x1 sample size
    #[inline] pub (crate) const fn right()    -> ContourEdge { ContourEdge(2) }   // Assuming a 1x1 sample size
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
        let ContourSize(size_x, size_y) = contour.size();

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
    fn size(self) -> ContourSize {
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
    fn size(self) -> ContourSize {
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
