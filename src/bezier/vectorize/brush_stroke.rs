use super::distance_field::*;
use super::sampled_contour::*;
use crate::bezier::*;
use crate::geo::*;

///
/// Trait implemented by brushes that can generate discrete 'daubs' using a distance field
///
/// This is used to generate brush strokes made up by layering images of the 'brush head' on top of each other, which can be converted
/// to vectors by using the `DaubBrushDistanceField`. Things that implement this inter
///
pub trait BrushDistanceField : SampledSignedDistanceField {
    ///
    /// Creates a daub with a size at a position. Returns a distance field representing which parts of the daub are filled
    /// and unfilled, and a position that indicates the offset to place the as part of the brush stroke.
    ///
    /// The centered position will be chosen so that `centered_at.x()-radius` and `centered_at.y()-radius` is greater than 1.
    ///
    fn create_daub(centered_at: impl Coordinate + Coordinate2D, radius: f64) -> Option<(Self, ContourPosition)>;
}

///
/// Creates the daubs making up a brush stroke from a bezier curve
///
/// The iterator can be passed into `DaubBrushDistanceField` to generate a distance field for the brush stroke. The generated path will
/// be at an offset, so a vector to subtract from the coorindates of the distance field is also returned.
///
/// The curve passed in to this function should have 3 dimensions: the third dimension is the radius of the brush stroke at each point.
///
/// The step value is the distance between each daub (smaller distances generate more points but are more accurate) and the max error is 
/// the amount of 'jitter' that is allowed in the spacing of the daubs. Values of `0.5` and `0.25` should produce good results.
///
pub fn brush_stroke_daubs<'a, TDistanceField, TCurve>(curve: &'a TCurve, step: f64, max_error: f64) -> (impl 'a + Iterator<Item=(TDistanceField, ContourPosition)>, Coord2)
where
    TCurve:         BezierCurve,
    TCurve::Point:  Coordinate + Coordinate3D,
    TDistanceField: 'a + BrushDistanceField,
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
    let offset          = bounds.min();
    let offset          = Coord2(offset.x() + radius_max + 1.0, offset.y() + radius_max + 1.0);

    // Create the daubs by walking the 2D curve
    let iterator = walk_curve_evenly(&curve2d, step, max_error)
        .flat_map(move |curve_section| {
            let (t_min, t_max)  = curve_section.original_curve_t_values();
            let t_mid           = (t_min+t_max)/2.0;

            let pos     = curve2d.point_at_pos(t_mid);
            let pos     = pos + offset;
            let radius  = radius.point_at_pos(t_mid);

            TDistanceField::create_daub(pos, radius)
        });

    // TODO: figure out how to make curve2d owned by the iterator
    let iterator = iterator.collect::<Vec<_>>().into_iter();

    (iterator, offset)
}
