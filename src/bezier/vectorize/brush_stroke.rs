use super::distance_field::*;
use super::daub_brush_distance_field::*;
use super::marching_squares::*;
use super::sampled_contour::*;
use crate::bezier::*;
use crate::bezier::path::*;
use crate::geo::*;

///
/// Trait implemented by brushes that can generate discrete 'daubs' using a distance field
///
/// This is used to generate brush strokes made up by layering images of the 'brush head' on top of each other, which can be converted
/// to vectors by using the `DaubBrushDistanceField` type.
///
pub trait DaubBrush {
    type DaubDistanceField;

    ///
    /// Creates a daub with a size at a position. Returns a distance field representing which parts of the daub are filled
    /// and unfilled, and a position that indicates the offset to place the as part of the brush stroke.
    ///
    /// The centered position will be chosen so that `centered_at.x()-radius` and `centered_at.y()-radius` is greater than 1.
    ///
    fn create_daub(&self, centered_at: impl Coordinate + Coordinate2D, radius: f64) -> Option<(Self::DaubDistanceField, ContourPosition)>;
}

///
/// Creates the daubs making up a brush stroke from a bezier curve
///
/// The iterator can be passed into `DaubBrushDistanceField` to generate a distance field for the brush stroke. The generated path will
/// be at an offset, so a vector to subtract from the coordinates of the distance field is also returned.
///
/// The curve passed in to this function should have 3 dimensions: the third dimension is the radius of the brush stroke at each point.
///
/// The step value is the distance between each daub (smaller distances generate more points but are more accurate) and the max error is 
/// the amount of 'jitter' that is allowed in the spacing of the daubs. Values of `0.5` and `0.25` should produce good results.
///
pub fn brush_stroke_daubs_from_curve<'a, TBrush, TCurve>(distance_field: &'a TBrush, curve: &'a TCurve, step: f64, max_error: f64) -> (impl 'a + Iterator<Item=(TBrush::DaubDistanceField, ContourPosition)>, Coord2)
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate3D,
    TBrush:         'a + DaubBrush,
{
    // Read the points from the curve
    let start_point = curve.start_point();
    let (cp1, cp2)  = curve.control_points();
    let end_point   = curve.end_point();

    // Create a 2D curve (this is the one we walk along: we don't care about moving equidistantly along the z axis)
    let curve2d     = Curve::from_points(Coord2(start_point.x(), start_point.y()), (Coord2(cp1.x(), cp1.y()), Coord2(cp2.x(), cp2.y())), Coord2(end_point.x(), end_point.y()));

    // We'll use the z positions for the radius
    let radius      = Curve::from_points(start_point.z(), (cp1.z(), cp2.z()), end_point.z());

    // The bounding box is used to create the offset
    let bounds          = curve2d.bounding_box::<Bounds<_>>();
    let radius_bounds   = radius.bounding_box::<Bounds<_>>();
    let radius_max      = radius_bounds.max().max(0.0);
    let radius_max      = radius_max.ceil() + 1.0;
    let offset          = bounds.min();
    let offset          = Coord2(offset.x() - radius_max - 1.0, offset.y() - radius_max - 1.0);

    // Subtract the offset from the curve
    let curve2d         = Curve::from_points(Coord2(start_point.x(), start_point.y())-offset, (Coord2(cp1.x(), cp1.y())-offset, Coord2(cp2.x(), cp2.y())-offset), Coord2(end_point.x(), end_point.y())-offset);

    // Create the daubs by walking the 2D curve
    let iterator = walk_curve_evenly_map(curve2d, step, max_error,
        move |curve_section| {
            let t_mid = curve_section.t_for_t(0.5);

            let pos     = curve2d.point_at_pos(t_mid);
            let radius  = radius.point_at_pos(t_mid);

            (pos, radius)
        }).flat_map(move |(pos, radius)| {
            distance_field.create_daub(pos, radius)
        });

    (iterator, offset)
}

///
/// Creates the daubs making up a brush stroke from a bezier path
///
/// The iterator can be passed into `DaubBrushDistanceField` to generate a distance field for the brush stroke. The generated path will
/// be at an offset, so a vector to subtract from the coordinates of the distance field is also returned.
///
/// The curve passed in to this function should have 3 dimensions: the third dimension is the radius of the brush stroke at each point.
///
/// The step value is the distance between each daub (smaller distances generate more points but are more accurate) and the max error is 
/// the amount of 'jitter' that is allowed in the spacing of the daubs. Values of `0.5` and `0.25` should produce good results.
///
pub fn brush_stroke_daubs_from_path<'a, TBrush, TPath>(distance_field: &'a TBrush, path: &'a TPath, step: f64, max_error: f64) -> (impl 'a + Iterator<Item=(TBrush::DaubDistanceField, ContourPosition)>, Coord2)
where
    TPath:          BezierPath,
    TPath::Point:   Coordinate + Coordinate3D,
    TBrush:         'a + DaubBrush,
{
    // Break the path up into 2D curves for the main path, and 1D curves for the radius
    let mut curves = vec![];

    for path_curve in path.to_curves::<Curve<_>>() {
        let (start_point, (cp1, cp2), end_point) = path_curve.all_points();

        let curve2d = Curve::from_points(Coord2(start_point.x(), start_point.y()), (Coord2(cp1.x(), cp1.y()), Coord2(cp2.x(), cp2.y())), Coord2(end_point.x(), end_point.y()));
        let radius  = Curve::from_points(start_point.z(), (cp1.z(), cp2.z()), end_point.z());

        curves.push((curve2d, radius));
    }

    // Compute the bounds of the curve as a whole
    let (bounds, radius_bounds) = if curves.len() > 0 {
        // Bounds start with the first item in the curve
        let mut bounds          = curves[0].0.bounding_box::<Bounds<_>>();
        let mut radius_bounds   = curves[0].1.bounding_box::<Bounds<_>>();

        // Merge the other curve items
        for (curve, radius) in curves.iter().skip(1) {
            bounds          = bounds.union_bounds(curve.bounding_box());
            radius_bounds   = radius_bounds.union_bounds(radius.bounding_box());
        }

        (bounds, radius_bounds)
    } else {
        // No curves
        (Bounds::empty(), Bounds::empty())
    };

    // The bounding box is used to create the offset
    let radius_max      = radius_bounds.max().max(0.0);
    let radius_max      = radius_max.ceil() + 1.0;
    let offset          = bounds.min();
    let offset          = Coord2(offset.x() - radius_max - 1.0, offset.y() - radius_max - 1.0);

    // Walk the curves to generate the points for this path
    let iterator = curves.into_iter()
        .enumerate()
        .flat_map(move |(idx, (curve2d, radius))| {
            let (start_point, (cp1, cp2), end_point) = curve2d.all_points();
            let curve2d_offset  = Curve::from_points(start_point-offset, (cp1-offset, cp2-offset), end_point-offset);
            let skip            = if idx == 0 { 0 } else { 1 };

            walk_curve_evenly_map(curve2d_offset, step, max_error, move |curve_section| {
                    let (t_min, t_max)  = curve_section.original_curve_t_values();
                    let t_mid           = (t_min+t_max)/2.0;

                    let pos     = curve2d_offset.point_at_pos(t_mid);
                    let radius  = radius.point_at_pos(t_mid);

                    (pos, radius)
                }).skip(skip)
                .flat_map(move |(pos, radius)| {
                    distance_field.create_daub(pos, radius)
                })
        });

    (iterator, offset)
}

///
/// Converts a 3-dimensional bezier curve into a 2-dimensional path where the 3rd dimension is the radius of the brush
///
/// The brush stroke is made by combining discrete 'daubs' and then tracing the resulting path.
///
/// `TBrush` specifies the type of distance field that make up the 'daubs' of the brush stroke. The simplest possible
/// distance field that can be used here is `CircularDistanceField`.
///
/// `TPath` specifies the type of path structure to produce (such as `SimpleBezierPath`)
///
/// `TBrushCurve` is any bezier curve using a 3-dimensional coordinate.
///
/// The `step` parameter indicates the distance between daubs on the brush. Higher values are faster but less accurate, lower values
/// are slower but produce a better shape `0.5` is a good default value. `max_error` indicates the maximum error to allow when generating
/// the daubs and the final path: `0.25` is a good default value for this parameter. Too low a value for `max_error` may produce artifacts
/// from over-fitting against the shape of the distance field.
///
pub fn brush_stroke_from_curve<TPath, TBrushCurve, TBrush>(distance_field: &TBrush, curve: &TBrushCurve, step: f64, max_error: f64) -> Vec<TPath>
where
    TPath:              BezierPathFactory,
    TPath::Point:       Coordinate + Coordinate2D,
    TBrushCurve:        BezierCurve,
    TBrushCurve::Point: Coordinate + Coordinate3D,
    TBrush:             DaubBrush,
    TBrush::DaubDistanceField: SampledSignedDistanceField,
{
    let (daubs, offset) = brush_stroke_daubs_from_curve(distance_field, curve, step, max_error);
    let distance_field  = DaubBrushDistanceField::from_daubs(daubs);
    let mut paths       = trace_paths_from_distance_field::<TPath>(&distance_field, max_error);

    let offset = TPath::Point::from_components(&[offset.x(), offset.y()]);

    paths.iter_mut().for_each(|path| *path = path.with_offset(offset));

    paths
}

///
/// Converts a 3-dimensional bezier path into a 2-dimensional path where the 3rd dimension is the radius of the brush
///
/// The brush stroke is made by combining discrete 'daubs' and then tracing the resulting path.
///
/// `TBrush` specifies the type of distance field that make up the 'daubs' of the brush stroke. The simplest possible
/// distance field that can be used here is `CircularDistanceField`.
///
/// `TPath` specifies the type of path structure to produce (such as `SimpleBezierPath`)
///
/// `TBrushPath` is any bezier path using a 3-dimensional coordinate.
///
/// The `step` parameter indicates the distance between daubs on the brush. Higher values are faster but less accurate, lower values
/// are slower but produce a better shape `0.5` is a good default value. `max_error` indicates the maximum error to allow when generating
/// the daubs and the final path: `0.25` is a good default value for this parameter. Too low a value for `max_error` may produce artifacts
/// from over-fitting against the shape of the distance field.
///
pub fn brush_stroke_from_path<TPath, TBrushPath, TBrush>(distance_field: &TBrush, path: &TBrushPath, step: f64, max_error: f64) -> Vec<TPath>
where
    TPath:              BezierPathFactory,
    TPath::Point:       Coordinate + Coordinate2D,
    TBrushPath:         BezierPath,
    TBrushPath::Point:  Coordinate + Coordinate3D,
    TBrush:             DaubBrush,
    TBrush::DaubDistanceField: SampledSignedDistanceField,
{
    let (daubs, offset) = brush_stroke_daubs_from_path(distance_field, path, step, max_error);
    let distance_field  = DaubBrushDistanceField::from_daubs(daubs);
    let mut paths       = trace_paths_from_distance_field::<TPath>(&distance_field, max_error);

    let offset = TPath::Point::from_components(&[offset.x(), offset.y()]);

    paths.iter_mut().for_each(|path| *path = path.with_offset(offset));

    paths
}
