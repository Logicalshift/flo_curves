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
    w1x: f64,
    w2x: f64,
    w3x: f64,
    w4x: f64,

    w1y: f64,
    w2y: f64,
    w3y: f64,
    w4y: f64,

    curve: &'a TCurve,
    range: Range<i64>,

    cur_y: i64,
}

impl<'a, TCurve> Iterator for RootSolvingScanIterator<'a, TCurve>
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate2D,
{
    type Item = ScanEdgeFragment;

    #[inline]
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
    fn scan_convert(self, path: &'a TCurve) -> Self::ScanIterator {
        // Get the curve points
        let start_point = path.start_point();
        let (cp1, cp2)  = path.control_points();
        let end_point   = path.end_point();

        // Solve the curve for the y-coordinate extremes
        let (w1x, w2x, w3x, w4x) = (start_point.x(), cp1.x(), cp2.x(), end_point.x());
        let (w1y, w2y, w3y, w4y) = (start_point.y(), cp1.y(), cp2.y(), end_point.y());

        let a = (-w1y + w2y*3.0 - w3y*3.0 + w4y)*3.0;
        let b = (w1y - w2y*2.0 + w3y)*6.0;
        let c = (w2y - w1y)*3.0;

        let root1 = (-b + f64::sqrt(b*b - a*c*4.0)) / (a*2.0);
        let root2 = (-b - f64::sqrt(b*b - a*c*4.0)) / (a*2.0);

        let mut y_min = f64::min(w1y, w4y);
        let mut y_max = f64::max(w1y, w4y);

        if root1 > 0.0 && root1 < 1.0 {
            let p1 = de_casteljau4(root1, w1y, w2y, w3y, w4y);
            y_min = f64::min(y_min, p1);
            y_max = f64::max(y_max, p1);
        }

        if root2 > 0.0 && root2 < 1.0 {
            let p2 = de_casteljau4(root2, w1y, w2y, w3y, w4y);
            y_min = f64::min(y_min, p2);
            y_max = f64::max(y_max, p2);
        }

        let y_min = y_min.floor() as i64;
        let y_max = y_max.floor() as i64 + 1;

        let y_min = i64::max(i64::min(self.y_range.end, y_min), y_min);
        let y_max = i64::min(i64::max(self.y_range.start, y_max), y_max);

        // Create the iterator for these lines
        RootSolvingScanIterator {
            w1x, w2x, w3x, w4x,
            w1y, w2y, w3y, w4y,

            curve: path,
            range: y_min..y_max,

            cur_y: y_min,
        }
    }
}
