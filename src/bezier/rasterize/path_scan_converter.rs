use super::scan_converter::*;
use super::curve_scan_converter::*;
use crate::bezier::*;
use crate::bezier::path::*;

use std::cmp::{Ordering};
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
                    Some((scanline, Some(iterator)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Order by scanline so we sweep the path from top to bottom
        all_curves.sort_by(|(scanline_a, _), (scanline_b, _)| scanline_a.cmp(scanline_b));

        let mut scanline_edges      = vec![];

        // Create a function to return the next curve
        let next_fn = move || {
            loop {
                // Loop until there are some edges to iterate
                if !scanline_edges.is_empty() {
                    break;
                }

                // Finished entirely once the all_curves list is empty
                if all_curves.is_empty() {
                    return None;
                }

                // First remaining scanline defines the 'current' scanline
                let current_scanline    = all_curves.get(0).map(|(scanline, _)| *scanline).unwrap_or(0);

                // Read everything in the current scanline
                let mut finished_curve  = false;

                for (scanline, scanline_iter) in all_curves.iter_mut() {
                    // Scanlines are stored in order with the earliest first, so stop once we find an iterator
                    if *scanline != current_scanline {
                        break;
                    }

                    if let Some(iter) = scanline_iter {
                        // Iterator is at the start of the current scanline: read from it to populate the scanline
                        loop {
                            match iter.next() {
                                Some(ScanEdgeFragment::StartScanline(new_scanline)) => {
                                    // End of scanline
                                    *scanline = new_scanline;
                                    break;
                                }

                                Some(ScanEdgeFragment::Edge(edge_x, fragment)) => {
                                    // TODO: update fragment with curve, path idx
                                    scanline_edges.push((edge_x, fragment))
                                }

                                None    => {
                                    // This curve is finished
                                    finished_curve = true;
                                    *scanline_iter = None;
                                    break;
                                }
                            }
                        }
                    } else {
                        // Shouldn't happen; one of the curves is finished
                        finished_curve = true;
                    }
                }

                // Remove finished curves from the list
                if finished_curve {
                    // TODO: only consider the curves that were on the current scanline (more efficient when there are very many curves)
                    all_curves.retain(|(_, iter)| iter.is_some());
                }

                // Order the edges in reverse order so we can just pop them to iterate
                scanline_edges.sort_by(|(edge_a, _), (edge_b, _)| {
                    edge_b.partial_cmp(edge_a).unwrap_or(Ordering::Equal)
                });
            }

            // Iterate through the scanlines
            scanline_edges.pop().map(|(edge_x, fragment)| ScanEdgeFragment::Edge(edge_x, fragment))
        };

        /*
        BezierPathScanConverterIterator {
            next_fn: Box::new(next_fn)
        }
        */
        todo!()
    }
}
