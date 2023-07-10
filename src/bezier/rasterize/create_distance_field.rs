use crate::bezier::vectorize::*;

///
/// Creates a distance field from a function providing a distance at a point
///
/// This is the slowest way to create a distance field in most instances
///
pub fn create_distance_field(signed_distance_at_point: impl Fn(f64, f64) -> f64, size: ContourSize) -> F64SampledDistanceField {
    let width   = size.width();
    let height  = size.height();

    let samples = (0..width*height)
        .map(|pixel| {
            let x           = pixel % width;
            let y           = pixel / width;

            signed_distance_at_point(x as _, y as _)
        })
        .collect();

    F64SampledDistanceField(size, samples)
}
