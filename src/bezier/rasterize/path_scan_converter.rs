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

///
/// Iterator for the bezier path scan converter
///
pub struct BezierPathScanConverterIterator {
    next_fn: Box<dyn FnMut() -> Option<ScanEdgeFragment>>
}

impl Iterator for BezierPathScanConverterIterator {
    type Item = ScanEdgeFragment;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.next_fn)()
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
    fn scan_convert(self, paths: &'a Vec<TPath>) -> Self::ScanIterator {
        // Collect all the curves from the paths
        let all_curves = paths.iter()
            .map(|path| {
                path.to_curves::<Curve<TPath::Point>>()
            })
            .collect::<Vec<_>>();

        let mut all_curves = all_curves
            .iter()
            .flat_map(|path_curves| {
                // Scan convert every curve
                path_curves.iter()
                    .map(|curve| self.curve_converter.scan_convert(curve))
            })
            .flat_map(|mut iterator| {
                // First instruction in every iterator should be a scanline
                let first_scanline = iterator.next()?;
                if let ScanEdgeFragment::StartScanline(scanline) = first_scanline {
                    // Store the 'current' scanline and the iterator
                    Some((scanline, iterator))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Order by scanline so we sweep the path from top to bottom
        all_curves.sort_by(|(scanline_a, _), (scanline_b, _)| scanline_a.cmp(scanline_b));

        let mut current_scanline = all_curves.get(0).map(|(scanline, _)| *scanline).unwrap_or(0);

        // Create a function to return the next curve
        let next_fn = move || {

        };

        todo!()
    }
}
