use super::scan_converter::*;
use crate::bezier::*;
use crate::geo::*;

use std::ops::{Range};
use std::marker::{PhantomData};

///
/// Bezier curve scan converter that works by root solving
///
/// This isn't the fastest algorithm but it's quite simple and reliably correct so it works as a baseline algorithm.
///
pub struct RootSolvingScanConverter<TCurve> 
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate2D,
{
    /// The y-range for this scan converter
    y_range:    Range<i64>,
    curve:      PhantomData<TCurve>
}

impl<TCurve> RootSolvingScanConverter<TCurve>
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate2D,
{
    ///
    /// Creates a bezier curve scan converter. Scanlines will be returned within the y_range
    ///
    fn new(y_range: Range<i64>) -> RootSolvingScanConverter<TCurve> {
        RootSolvingScanConverter {
            y_range:    y_range,
            curve:      PhantomData,
        }
    }
}

///
/// Iterator that solves the roots of a bezier curve on each scanline in a range
///
pub struct RootSolvingScanIterator<'a, TCurve>
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate2D,
{
    curve: &'a TCurve,
}

impl<'a, TCurve> Iterator for RootSolvingScanIterator<'a, TCurve>
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate2D,
{
    type Item = ScanEdgeFragment;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }    
}

impl<'a, TCurve> ScanConverter<'a, TCurve> for &'a RootSolvingScanConverter<TCurve>
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate2D,
{
    /// The iterator type that returns scan fragments from this path
    type ScanIterator = RootSolvingScanIterator<'a, TCurve>;

    ///
    /// Takes a bezier path and scan converts it. Edges are returned from the top left (y index 0) and 
    ///
    #[inline]
    fn scan_convert(self, path: &'a TCurve) -> Self::ScanIterator {
        RootSolvingScanIterator {
            curve: path
        }
    }
}
