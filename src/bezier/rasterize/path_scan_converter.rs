use super::scan_converter::*;
use super::curve_scan_converter::*;
use crate::bezier::*;
use crate::bezier::path::*;

use std::ops::{Range};
use std::marker::{PhantomData};

pub struct BezierPathScanConverter<TPath, TCurveScanConverter>
where
    TPath:                              BezierPath,
    TPath::Point:                       Coordinate + Coordinate2D,
    for<'a> &'a TCurveScanConverter:    ScanConverter<'a, Curve<TPath::Point>>,
{
    path:               PhantomData<TPath>,
    curve_converter:    TCurveScanConverter,
}

impl<TPath, TCurveScanConverter> BezierPathScanConverter<TPath, TCurveScanConverter>
where
    TPath:                              BezierPath,
    TPath::Point:                       Coordinate + Coordinate2D,
    for<'a> &'a TCurveScanConverter:    ScanConverter<'a, Curve<TPath::Point>>,
{
    ///
    /// Creates a bezier path scan converter
    ///
    pub fn with_curve_converter(scan_converter: TCurveScanConverter) -> BezierPathScanConverter<TPath, TCurveScanConverter> {
        BezierPathScanConverter {
            path:               PhantomData,
            curve_converter:    scan_converter,
        }
    }
}

impl<TPath> BezierPathScanConverter<TPath, RootSolvingScanConverter<Curve<TPath::Point>>>
where
    TPath:          BezierPath,
    TPath::Point:   Coordinate + Coordinate2D,
{
    ///
    /// Creates a bezier path scan converter using the default bezier curve scan converter
    ///
    pub fn new(y_range: Range<i64>) -> BezierPathScanConverter<TPath, RootSolvingScanConverter<Curve<TPath::Point>>> {
        Self::with_curve_converter(RootSolvingScanConverter::new(y_range))
    }
}

pub struct BezierPathScanConverterIterator {
}

impl Iterator for BezierPathScanConverterIterator {
    type Item = ScanEdgeFragment;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, TPath, TCurveScanConverter> ScanConverter<'a, Vec<TPath>> for &'a BezierPathScanConverter<TPath, TCurveScanConverter>
where
    TPath:                              'a + BezierPath,
    TPath::Point:                       'a + Coordinate + Coordinate2D,
    for<'b> &'b TCurveScanConverter:    ScanConverter<'b, Curve<TPath::Point>>,
{
    /// The iterator type that returns scan fragments from this path
    type ScanIterator = BezierPathScanConverterIterator;

    ///
    /// Takes a bezier path and scan converts it. Edges are returned from the top left (y index 0) and 
    ///
    fn scan_convert(self, path: &'a Vec<TPath>) -> Self::ScanIterator {
        todo!()
    }
}
