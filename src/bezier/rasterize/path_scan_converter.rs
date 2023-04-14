use super::scan_converter::*;
use super::curve_scan_converter::*;
use crate::bezier::*;
use crate::bezier::path::*;

use ouroboros::self_referencing;

use std::cmp::{Ordering};
use std::ops::{Range};
use std::marker::{PhantomData};

pub struct BezierPathScanConverter<TPath, TCurveScanConverter>
where
    TPath:          BezierPath,
    TPath::Point:   Coordinate + Coordinate2D,
{
    path:               PhantomData<TPath>,
    curve_converter:    TCurveScanConverter,
}

impl<TPath, TCurveScanConverter> BezierPathScanConverter<TPath, TCurveScanConverter>
where
    TPath:          BezierPath,
    TPath::Point:   Coordinate + Coordinate2D,
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
#[self_referencing]
pub struct BezierPathScanConverterIterator<TPath, TCurveScanConverter>
where
    TPath:                              'static + BezierPath,
    TPath::Point:                       'static + Coordinate + Coordinate2D,
    TCurveScanConverter:                'static,
    for<'b> &'b TCurveScanConverter:    ScanConverter<'b, Curve<TPath::Point>>,
{
    /// The scan converter for this iterator (we have to clone this instead of borrowing it due to lifetime issues with ouroboros and a crash in the borrow checker if we use a reference here)
    scan_converter: TCurveScanConverter,

    /// All the curves from all of the paths
    all_curves: Vec<(usize, usize, Curve<TPath::Point>)>,

    /// The iterators for each curve
    #[borrows(all_curves, scan_converter)]
    #[not_covariant]
    scanline_iterators: Vec<(usize, usize, i64, Option<<&'this TCurveScanConverter as ScanConverter<'this, Curve<TPath::Point>>>::ScanIterator>)>,

    /// The scan edges for the current scanline, in reverse order
    scanline_edges: Vec<(ScanX, ScanFragment)>,
}

impl<TPath, TCurveScanConverter> Iterator for BezierPathScanConverterIterator<TPath, TCurveScanConverter>
where
    TPath:                              'static + BezierPath,
    TPath::Point:                       'static + Coordinate + Coordinate2D,
    TCurveScanConverter:                'static,
    for<'b> &'b TCurveScanConverter:    ScanConverter<'b, Curve<TPath::Point>>,
{
    type Item = ScanEdgeFragment;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.with_mut(|fields| {
            loop {
                // Loop until there are some edges to iterate
                if !fields.scanline_edges.is_empty() {
                    break;
                }

                // Finished entirely once the scanline_iterators list is empty
                if fields.scanline_iterators.is_empty() {
                    return None;
                }

                // First remaining scanline defines the 'current' scanline
                let current_scanline    = fields.scanline_iterators.get(0).map(|(_, _, scanline, _)| *scanline).unwrap_or(0);

                // Read everything in the current scanline
                let mut finished_curve  = false;

                for (path_idx, curve_idx, scanline, scanline_iter) in fields.scanline_iterators.iter_mut() {
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
                                    if fragment.t < 1.0 {
                                        // Hits that exactly match an endpoint will also match the start point of the following curve, so we exclude those
                                        fields.scanline_edges.push((edge_x, fragment))
                                    }
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
                    fields.scanline_iterators.retain(|(_, _, _, iter)| iter.is_some());
                }

                // Order the edges in reverse order so we can just pop them to iterate
                fields.scanline_edges.sort_by(|(edge_a, _), (edge_b, _)| {
                    edge_b.partial_cmp(edge_a).unwrap_or(Ordering::Equal)
                });

                if !fields.scanline_edges.is_empty() {
                    // Indicate that we're starting this scanline
                    return Some(ScanEdgeFragment::StartScanline(current_scanline));
                }
            }

            // Iterate through the recently generated scanlines (should be at least one if we reach here)
            fields.scanline_edges.pop().map(|(edge_x, fragment)| ScanEdgeFragment::Edge(edge_x, fragment))
        })
    }
}

impl<'a, TPath, TCurveScanConverter> ScanConverter<'a, Vec<TPath>> for &'a BezierPathScanConverter<TPath, TCurveScanConverter>
where
    TPath:                              'static + BezierPath,
    TPath::Point:                       'static + Coordinate + Coordinate2D,
    TCurveScanConverter:                'static + Clone,
    for<'b> &'b TCurveScanConverter:    ScanConverter<'b, Curve<TPath::Point>>,
{
    /// The iterator type that returns scan fragments from this path
    type ScanIterator = BezierPathScanConverterIterator<TPath, TCurveScanConverter>;

    ///
    /// Takes a bezier path and scan converts it. Edges are returned from the top left (y index 0) and 
    ///
    fn scan_convert(self, paths: &'a Vec<TPath>) -> Self::ScanIterator {
        // Collect all the curves from the paths
        let all_curves = paths.iter().enumerate()
            .flat_map(|(path_idx, path)| {
                path.to_curves::<Curve<TPath::Point>>()
                    .into_iter()
                    .enumerate()
                    .map(move |(curve_idx, curve)| (path_idx, curve_idx, curve))
            })
            .collect::<Vec<_>>();

        // Create the iterator for all of the scanlines
        let path_scanline_iterator  = BezierPathScanConverterIteratorBuilder {
            scan_converter:     self.curve_converter.clone(),
            all_curves:         all_curves,
            scanline_edges:     vec![],

            scanline_iterators_builder: move |all_curves, scan_converter| {
                // Create iterators for the curves
                let mut scanline_iterators = all_curves
                    .iter()
                    .map(move |(path_idx, curve_idx, curve)| {
                        (*path_idx, *curve_idx, (*scan_converter).scan_convert(curve))
                    })
                    .flat_map(move |(path_idx, curve_idx, mut iterator)| {
                        // First instruction in every iterator should be a scanline
                        let first_scanline = iterator.next()?;
                        if let ScanEdgeFragment::StartScanline(scanline) = first_scanline {
                            // Store the 'current' scanline and the iterator
                            Some((path_idx, curve_idx, scanline, Some(iterator)))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                // Order by scanline so we sweep the path from top to bottom
                scanline_iterators.sort_by(|(_, _, scanline_a, _), (_, _, scanline_b, _)| scanline_a.cmp(scanline_b));

                scanline_iterators
            }
        }.build();

        path_scanline_iterator
    }
}
