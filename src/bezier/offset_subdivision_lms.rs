use super::fit::*;
use super::curve::*;
use super::bounds::*;
use super::normal::*;
use crate::geo::*;

///
/// Options for the `offset_lms_subdivisions` function
///
/// In order to find the samples to fit the resulting curve against, we subdivide the curve first at its extremities, and then until each sample
/// represents a line segment of a certain flatness or length. This struct can be used to configure these parameters.
///
#[derive(Copy, Clone)]
pub struct SubdivsionOffsetOptions {
    /// The minimum initial number of subdivisions to use
    initial_subdivisions: usize,

    /// The minimum difference between tangents where no further subdivsions are made  
    min_tangent: f64,

    /// The distance between points to give up on the subdivision routine
    min_distance: f64,

    /// The maximum distance between points before they are subdivided
    max_distance: f64,

    /// The maximum allowed error in the fitted curve
    max_error: f64,
}

impl Default for SubdivsionOffsetOptions {
    fn default() -> Self {
        SubdivsionOffsetOptions {
            initial_subdivisions:   8,
            min_tangent:            0.05,
            min_distance:           0.1,
            max_distance:           10.0,
            max_error:              1.0,
        }
    }
}

impl SubdivsionOffsetOptions {
    ///
    /// Sets the minimum number of subdivisions to use for any curve
    ///
    /// The initial curve is always subdivided into at least this many points before trying to flatten the remaining subdivisions
    ///
    #[inline]
    pub fn with_initial_subdivisions(mut self, initial_subdivisions: usize) -> Self {
        self.initial_subdivisions = initial_subdivisions;

        self
    }

    ///
    /// Sets the minimum tangent difference between any two sections of the samples
    ///
    /// This defines the minimum difference in slope between two sections of the curve
    ///
    #[inline]
    pub fn with_min_tangent(mut self, min_tangent: f64) -> Self {
        self.min_tangent = min_tangent;

        self
    }

    ///
    /// Sets the minimum distance between samples
    ///
    /// This is used as a way to stop the subdivision if the minimum tangent fails to find a point where the curve is flattening out
    ///
    #[inline]
    pub fn with_min_distance(mut self, min_distance: f64) -> Self {
        self.min_distance = min_distance;

        self
    }

    ///
    /// Sets the maximum error for fitting the final curve generated by the subdivision algorithm
    ///
    #[inline]
    pub fn with_max_error(mut self, max_error: f64) -> Self {
        self.max_error = max_error;

        self
    }
}

///
/// Calculates the offset point and unit tangent for a point on the curve
///
#[inline]
fn calc_offset_point<Curve, NormalOffsetFn, TangentOffsetFn>(curve: &Curve, normal_offset_for_t: &NormalOffsetFn, tangent_offset_for_t: &TangentOffsetFn, t: f64) -> (Curve::Point, Curve::Point)
where
    Curve:              BezierCurveFactory+NormalCurve,
    Curve::Point:       Normalize+Coordinate2D,
    NormalOffsetFn:     Fn(f64) -> f64,
    TangentOffsetFn:    Fn(f64) -> f64,
{
    // Compute the curve point and normal/tangent points
    let mut point       = curve.point_at_pos(t);
    let normal_offset   = normal_offset_for_t(t);
    let tangent_offset  = tangent_offset_for_t(t);

    // Add the offset and tangent
    let unit_tangent    = curve.tangent_at_pos(t).to_unit_vector();
    let unit_normal     = Curve::Point::to_normal(&point, &unit_tangent);
    let unit_normal     = Curve::Point::from_components(&unit_normal);

    point = point + (unit_normal * normal_offset) + (unit_tangent * tangent_offset);

    (point, unit_tangent)
}

///
/// Produces an offset curve by performing a least-mean-square curve fit against the output of a function
///
/// The samples are obtained by subdividing the curve until the 
///
pub fn offset_lms_subdivisions<Curve, NormalOffsetFn, TangentOffsetFn>(curve: &Curve, normal_offset_for_t: NormalOffsetFn, tangent_offset_for_t: TangentOffsetFn, subdivision_options: SubdivsionOffsetOptions) -> Option<Vec<Curve>>
where
    Curve:              BezierCurveFactory+NormalCurve,
    Curve::Point:       Normalize+Coordinate2D,
    NormalOffsetFn:     Fn(f64) -> f64,
    TangentOffsetFn:    Fn(f64) -> f64,
{
    let (start_point, (cp1, cp2), end_point) = curve.all_points();

    // The initial set of points are the extremeties plus 0.0 and 1.0
    let mut extremities = find_extremities(start_point, cp1, cp2, end_point);
    extremities.retain(|t| (0.0..=1.0).contains(t));
    extremities.sort_unstable_by(|t1, t2| t1.total_cmp(t2));

    if extremities.len() == 0 || extremities[0] != 0.0 {
        extremities.insert(0, 0.0);
    }
    if extremities.last() != Some(&1.0) {
        extremities.push(1.0);
    }

    // Calculate the offsets at these as the initial set of samples
    let mut samples = extremities.into_iter()
        .map(|t| (t, calc_offset_point(curve, &normal_offset_for_t, &tangent_offset_for_t, t)))
        .collect::<Vec<_>>();

    // Subdivide any point that is too far away or has a big difference in angles to
    loop {
        // Process from samples into next_samples
        let mut subdivided      = false;
        let mut next_samples    = vec![];

        // When we subdivide we divide two sections at once
        let mut idx = 0;
        while idx < samples.len()-1 {
            // Read the next two points
            let (t1, (first_point, first_tangent))  = &samples[idx];
            let (t2, (next_point, next_tangent))    = &samples[idx+1];

            // Keep the first point for the next set of samples
            next_samples.push((*t1, (*first_point, *first_tangent)));

            let distance = first_point.distance_to(&next_point);

            if distance > subdivision_options.max_distance {
                // Subdivide between t1, t2 as these points are too far apart
                let t3 = (t1+t2)/2.0;
                next_samples.push((t3, calc_offset_point(curve, &normal_offset_for_t, &tangent_offset_for_t, t3)));

                subdivided = true;
            } else if distance > subdivision_options.min_distance && idx < samples.len()-2 {
                // Subdivide the current and next section if the angle difference is low enough
                let first_angle     = f64::atan2(first_tangent.x(), first_tangent.y());
                let second_angle    = f64::atan2(next_tangent.x(), next_tangent.y());
                let angle_diff      = (first_angle-second_angle).abs();

                if angle_diff > subdivision_options.min_tangent {
                    // Subdivide both sides of the points (we already checked that the index is early enough that we can do this)
                    let (t3, (_, _)) = &samples[idx+2];
                    let t4 = (t1+t2)/2.0;
                    let t5 = (t2+t3)/2.0;

                    // Add the three new points
                    next_samples.push((t4, calc_offset_point(curve, &normal_offset_for_t, &tangent_offset_for_t, t4)));
                    next_samples.push((*t2, (*next_point, *next_tangent)));
                    next_samples.push((t5, calc_offset_point(curve, &normal_offset_for_t, &tangent_offset_for_t, t5)));

                    subdivided = true;

                    // Already added idx+1, so skip it to avoid creating points out of order
                    idx += 1;
                }
            }

            // Move to the next sample
            idx += 1;
        }

        // Push any samples that have not been processed
        while idx < samples.len() {
            next_samples.push(samples[idx]);
            idx +=1;
        }

        // Replace the samples with the new samples
        samples = next_samples;

        // Stop once there are no more subdivisions
        if !subdivided {
            break;
        }
    }

    // Convert the samples to just the points and fit the curve
    let sample_points = samples.into_iter().map(|(_, (point, _))| point).collect::<Vec<_>>();

    let start_tangent   = curve.tangent_at_pos(0.0).to_unit_vector();
    let end_tangent     = curve.tangent_at_pos(1.0).to_unit_vector() * -1.0;
    Some(fit_curve_cubic(&sample_points, &start_tangent, &end_tangent, subdivision_options.max_error))
}
