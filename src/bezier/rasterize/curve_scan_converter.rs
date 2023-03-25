use super::scan_converter::*;
use crate::bezier::*;
use crate::geo::*;

use roots::{find_roots_quadratic, find_roots_cubic, Roots};

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
    pub fn new(y_range: Range<i64>) -> RootSolvingScanConverter<TCurve> {
        RootSolvingScanConverter {
            y_range:    y_range,
            curve:      PhantomData,
        }
    }
}

///
/// Iterator that solves the roots of a bezier curve on each scanline in a range
///
pub struct RootSolvingScanIterator {
    w1x: f64,
    w2x: f64,
    w3x: f64,
    w4x: f64,

    a: f64,
    b: f64,
    c: f64,
    d: f64,

    range: Range<i64>,

    cur_y: i64,
    waiting_roots: Roots<f64>,
}

impl RootSolvingScanIterator {
    ///
    /// Generates a fragment at the current position on the scanline
    ///
    #[inline]
    fn create_fragment(&self, t: f64) -> ScanEdgeFragment {
        let x = de_casteljau4(t, self.w1x, self.w2x, self.w3x, self.w4x);

        ScanEdgeFragment::Edge(ScanX(x), ScanFragment { path_idx: 0, curve_idx: 0, t: t })
    }
}

impl Iterator for RootSolvingScanIterator {
    type Item = ScanEdgeFragment;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Update/return the next value from the existing list of roots
        match self.waiting_roots {
            Roots::No(_)                => { },
            Roots::One([a])             => {
                self.waiting_roots = Roots::No([]);
                return Some(self.create_fragment(a));
            }
            Roots::Two([a, b])          => {
                self.waiting_roots = Roots::One([b]);
                return Some(self.create_fragment(a));
            }
            Roots::Three([a, b, c])     => {
                self.waiting_roots = Roots::Two([b, c]);
                return Some(self.create_fragment(a));
            }
            Roots::Four([a, b, c, d])   => {
                self.waiting_roots = Roots::Three([b, c, d]);
                return Some(self.create_fragment(a));
            }
        }

        loop {
            // Finished once cur_y leaves the end of the range
            if self.cur_y >= self.range.end {
                return None;
            }

            // Start solving for this scanline
            let scanline = self.cur_y;
            self.cur_y += 1;

            // Get the coefficients, modified for the current y position
            let (a, b, c, d) = (self.a, self.b, self.c, self.d - (scanline as f64));

            // Solve the curve at this y position
            let roots = if a.abs() <  0.00000001 { find_roots_quadratic(b, c, d) } else { find_roots_cubic(a, b, c, d) };

            // Start a new scanline if there are any roots here
            if let Roots::No(_) = &roots { 
                continue; 
            } else {
                self.waiting_roots = roots;
                return Some(ScanEdgeFragment::StartScanline(scanline));
            }
        }
    }
}

impl<'a, TCurve> ScanConverter<'a, TCurve> for &'a RootSolvingScanConverter<TCurve>
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate2D,
{
    /// The iterator type that returns scan fragments from this path
    type ScanIterator = RootSolvingScanIterator;

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

        // Clip to the range
        let y_min = i64::max(i64::min(self.y_range.end, y_min), y_min);
        let y_max = i64::min(i64::max(self.y_range.start, y_max), y_max);

        // Calculate the base coefficients for the curve in the y axis
        let d = w1y;
        let c = 3.0*(w2y-w1y);
        let b = 3.0*(w3y-w2y)-c;
        let a = w4y-w1y-c-b;

        // Create the iterator for these lines
        RootSolvingScanIterator {
            w1x, w2x, w3x, w4x,
            a, b, c, d,

            range: y_min..y_max,

            cur_y: y_min,
            waiting_roots: Roots::No([]),
        }
    }
}
