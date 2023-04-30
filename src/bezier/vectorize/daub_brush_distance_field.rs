use super::distance_field::*;
use super::sampled_contour::*;

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
    daubs: Vec<(TDaub, ContourPosition)>
}

///
/// Represents an iterator over the edges in a 
///
struct EdgeIterator<TIterator> {
    daub_idx:       usize,
    iterator:       Option<TIterator>,
    daub_position:  ContourPosition,
    size:           ContourSize,
    lookahead:      (ContourPosition, ContourCell),
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
    edge_iterators: Vec<EdgeIterator<<<TDaub as SampledSignedDistanceField>::Contour as SampledContour>::EdgeCellIterator>>,

    /// The edge iterators that are on a future scanline (these are generally iterators that have moved down a scanline)
    future_scanline_iterators: Vec<EdgeIterator<<<TDaub as SampledSignedDistanceField>::Contour as SampledContour>::EdgeCellIterator>>,

    /// The scanline that we're collecting edges for
    current_scanline: usize,
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

                let ContourSize(daub_w, daub_h) = next_daub.size();
                let daub_w                      = x + daub_w;
                let daub_h                      = y + daub_h;

                ContourSize(usize::max(w, daub_w), usize::max(h, daub_h))
            });

        // Sort the daubs by y position
        daubs.sort_by_key(|(_, ContourPosition(_, y))| *y);

        DaubBrushDistanceField {
            size, daubs
        }
    }
}

impl<'a, TDaub> SampledContour for &'a DaubBrushDistanceField<TDaub> 
where
    TDaub: SampledSignedDistanceField,
{
    type EdgeCellIterator = DaubBrushContourIterator<'a, TDaub>;

    #[inline]
    fn size(self) -> ContourSize {
        self.size
    }

    #[inline]
    fn point_is_inside(self, pos: ContourPosition) -> bool {
        self.distance_at_point(pos) <= 0.0
    }

    fn edge_cell_iterator(self) -> Self::EdgeCellIterator {
        // 

        // Create the iterator
        DaubBrushContourIterator {
            distance_field:             self,
            next_daub_idx:              0,
            edge_iterators:             vec![],
            future_scanline_iterators:  vec![],
            current_scanline:           0,
        }
    }
}

impl<'a, TDaub> SampledSignedDistanceField for &'a DaubBrushDistanceField<TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    type Contour = &'a DaubBrushDistanceField<TDaub>;

    #[inline]
    fn size(self) -> ContourSize {
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
            let ContourSize(w, h) = daub.size();
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

impl<'a, TDaub> Iterator for DaubBrushContourIterator<'a, TDaub>
where
    TDaub: SampledSignedDistanceField,
{
    type Item = (ContourPosition, ContourCell);

    fn next(&mut self) -> Option<(ContourPosition, ContourCell)> {
        // Where two shapes overlap each other, it's possible either for there to be an edge from either shape, a combined edge made of both shapes,
        // an edge one pixel to the left or right, or no edge at all. In the case the edge looks like it might have moved, it's possible that the new
        // edge may have been hit by another pixel already.

        // Might be possible to test all the possibilities by building up an exhaustive set of possible edge tiles and what they can be combined with

        todo!()
    }
}

