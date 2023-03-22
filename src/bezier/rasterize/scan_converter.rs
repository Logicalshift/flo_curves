use crate::bezier::path::*;

///
/// A normal for a scan edge fragment
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScanNormal(pub f64, pub f64);

///
/// X position of a scan fragment
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScanX(pub f64);

///
/// A scan fragment indicates which part of a set of curves that a scan edge comes from
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScanFragment {
    /// The index into the list of paths that this fragment is from
    pub path_idx:   u32,

    /// The curve index within the path that this fragment is from
    pub curve_idx:  u32,

    /// The 't' value along the curve where the intersection occurred
    pub t:          f64,
}

///
/// An edge fragment is an instruction produced by a scan converter. Edges are samples located at the center of a scan line:
/// they are calculated to high precision horizontally and are supplied with a normal vector.
///
pub enum ScanEdgeFragment {
    /// Indicates that the scan converter is about to produce the edge fragments for the specified scanline
    StartScanline(i64),

    /// Edge fragment on the current scanline (these are returned in ascending order)
    Edge(ScanX, ScanNormal, ScanFragment),
}

///
/// Trait implemented by things that can convert bezier paths to scanline positions
///
pub trait ScanConverter<'a> : Copy {
    /// The iterator type that returns scan fragments from this path
    type ScanIterator: 'a + Iterator<Item=ScanEdgeFragment>;

    ///
    /// Takes a bezier path and scan converts it. Edges are returned from the top left (y index 0) and 
    ///
    fn scan_convert<TPath: 'a + BezierPath>(self, paths: impl IntoIterator<Item=&'a TPath>) -> Self::ScanIterator;
}
