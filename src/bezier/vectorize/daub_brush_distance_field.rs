///
/// Describes a shape as a distance field made up by 'daubing' discrete brush shapes over a canvas
///
/// This is quite similar to how a painting application generates brush strokes, except that this can be combined with
/// `trace_contours_from_distance_field()` in order to produce a vector representation of the brush stroke instead of
/// working only with bitmap images.
///
/// Note that for just creating a thick line, `offset_lms_sampling()` is much faster but it can only offset along a
/// fixed distance along the normal of the curve, so it doesn't produce good results if the offset is changing across
/// the span of the curve or if the curve is not particularly smooth. `offset_scaling()` is also available as an even
/// faster alternative for the simple cases, but is even more limited in terms of what it can produce.
///
/// This provides the most general purpose approach to generating vectors from brush strokes or other patterns.
///
pub struct DaubBrushDistanceField {

}