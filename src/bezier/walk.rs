use super::basis::*;
use super::curve::*;
use super::section::*;
use super::derivative::*;

use crate::geo::*;

///
/// Walks a bezier curve by dividing it into a number of sections
///
/// These sections are uneven in length: they all advance equally by 't' value but the points will
/// be spaced according to the shape of the curve (will have an uneven distance between them) 
///
#[inline]
pub fn walk_curve_unevenly<'a, Curve: BezierCurve>(curve: &'a Curve, num_subdivisions: usize) -> impl 'a+Iterator<Item=CurveSection<'a, Curve>> {
    if num_subdivisions > 0 {
        UnevenWalkIterator {
            curve:              curve,
            step:               (1.0)/(num_subdivisions as f64),
            num_subdivisions:   num_subdivisions,
            last_subdivision:   0
        }
    } else {
        UnevenWalkIterator {
            curve:              curve,
            step:               0.0,
            num_subdivisions:   0,
            last_subdivision:   0
        }
    }
}

///
/// Walks a bezier curve by moving forward a set amount at each point. Each point may be up to `max_error` away from `distance.
///
pub fn walk_curve_evenly<'a, Curve: BezierCurve>(curve: &'a Curve, distance: f64, max_error: f64) -> EvenWalkIterator<'a, Curve> {
    const INITIAL_INCREMENT: f64 = 0.1;

    // Too small or negative values might produce bad effects due to floating point inprecision
    let max_error       = if max_error < 1e-10  { 1e-10 } else { max_error };
    let distance        = if distance < 1e-10   { 1e-10 } else { distance };

    // Compute the derivative of the curve
    let (cp1, cp2)      = curve.control_points();
    let (wn1, wn2, wn3) = derivative4(curve.start_point(), cp1, cp2, curve.end_point());

    // We can calculate the initial speed from close to the first point of the curve
    let initial_speed   = de_casteljau3(0.001, wn1, wn2, wn3).magnitude();

    let initial_increment = if initial_speed.abs() < 0.00000001 {
        INITIAL_INCREMENT
    } else {
        distance / initial_speed
    };

    EvenWalkIterator {
        curve:          curve,
        derivative:     (wn1, wn2, wn3),
        last_t:         0.0, 
        last_point:     curve.start_point(),
        last_increment: initial_increment,
        distance:       distance,
        max_error:      max_error
    }
}

///
/// Iterator implemenation that performs an uneven walk along a curve
///
struct UnevenWalkIterator<'a, Curve: BezierCurve> {
    /// The curve that this is iterating over
    curve:              &'a Curve,

    /// The distance between t-values
    step:               f64,

    /// The total number of subdivisions to return
    num_subdivisions:   usize,

    /// The number of the most recently returned subdivision
    last_subdivision:   usize
}

impl<'a, Curve: BezierCurve> Iterator for UnevenWalkIterator<'a, Curve> {
    type Item = CurveSection<'a, Curve>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.last_subdivision >= self.num_subdivisions {
            // No more sections
            None
        } else {
            // Update the position and work out the range of t-values to return
            let t_min = self.step * (self.last_subdivision as f64);
            self.last_subdivision += 1;
            let t_max = self.step * (self.last_subdivision as f64);

            // Generate a section for this range of values
            Some(self.curve.section(t_min, t_max))
        }
    }
}

///
/// Iterator implementation that performs an even walk along a curve
///
pub struct EvenWalkIterator<'a, Curve: BezierCurve> {
    /// The curve that is being walked
    curve:          &'a Curve,

    /// The wn1, wn2, wn3 of the derivative of the curve
    derivative:     (Curve::Point, Curve::Point, Curve::Point),

    /// The last 't' value where a coordinate was generated
    last_t:         f64,

    /// The point generated at the last 't' value
    last_point:     Curve::Point,

    /// The last increment
    last_increment: f64,

    /// The target distance between points (as the chord length)
    distance:       f64,

    /// The maximum error in distance for the points that are generated by this iterator
    max_error:      f64
}

///
/// Iterator that modifies the behaviour of EvenWalkIterator so that it varies the distance between
/// each step
///
struct VaryingWalkIterator<'a, Curve: BezierCurve, DistanceIter: 'a+Iterator<Item=f64>> {
    /// The even walk iterator
    even_iterator: EvenWalkIterator<'a, Curve>,

    /// Iterator that returns the distance for each step (or None if the distance is fixed for the remaining distance)
    distance_iterator: Option<DistanceIter>
}

impl<'a, Curve: BezierCurve> EvenWalkIterator<'a, Curve> {
    ///
    /// Changes this iterator into one that varies distance with each step
    ///
    /// The supplied iterator will by used to get the distance for each subsequent step of the iterator. Normally, this iterator
    /// would generate a cycle of distances (say, by calling `cycle()`), but if it does end, the last distance will be used
    /// until the iteration over the curve is completed
    ///
    pub fn vary_by<DistanceIter: 'a+Iterator<Item=f64>>(self, distance: DistanceIter) -> impl 'a+Iterator<Item=CurveSection<'a, Curve>> {
        VaryingWalkIterator {
            even_iterator:      self,
            distance_iterator:  Some(distance)
        }
    }
}

impl<'a, Curve: BezierCurve> Iterator for EvenWalkIterator<'a, Curve> {
    type Item = CurveSection<'a, Curve>;

    fn next(&mut self) -> Option<Self::Item> {
        const MAX_ITERATIONS: usize = 32;

        // Gather values
        let curve           = self.curve;
        let (wn1, wn2, wn3) = self.derivative;
        let distance        = self.distance;
        let max_error       = self.max_error;
        let mut t_increment = self.last_increment;
        let last_t          = self.last_t;
        let mut next_t      = last_t + t_increment;
        let last_point      = self.last_point.clone();
        let mut next_point;

        // If the next point appears to be after the end of the curve, and the end of the curve is further away than the closest distance, return None
        if last_t >= 1.0 {
            return None;
        }

        if next_t >= 1.0 {
            if last_point.distance_to(&curve.point_at_pos(1.0)) < distance {
                // End point is closer than the target distance
                let last_section = curve.section(last_t, 1.0);
                self.last_t = 1.0;
                return Some(last_section);
            }
        }

        let mut count = 0;
        loop {
            debug_assert!(!t_increment.is_nan());

            // next_point contains the initial estimate of the position of the point at distance 't' from the current point
            next_point      = curve.point_at_pos(next_t);

            // Compute the distance to the guess and the error
            let next_distance   = last_point.distance_to(&next_point);
            let error           = distance - next_distance;

            // We've found the next point if the error drops low enough
            if error.abs() < max_error {
                break;
            }

            // Use the slope of the curve at this position to work out the next point to try
            let tangent         = de_casteljau3(next_t, wn1, wn2, wn3);
            let speed           = tangent.magnitude();

            if speed.abs() < 0.00000001 {
                // Very rarely, the speed can be 0 (at t=0 or t=1 when the control points overlap, for the easiest example to construct)

                // Use the error to adjust the t position we're testing if it's larger than max_error
                let error_ratio     = distance / next_distance;
                t_increment         = if error_ratio < 0.5 { 
                    t_increment * 0.5
                } else if error_ratio > 1.5 { 
                    t_increment * 1.5
                } else {
                    t_increment * error_ratio
                };
            } else {
                // Use the current speed to work out the adjustment for t_increment
                let error       = next_distance - distance;
                let adjustment  = error / speed;

                if adjustment >= t_increment {
                    t_increment *= 0.3333333;
                } else {
                    t_increment = t_increment - adjustment;
                }
            }

            next_t              = last_t + t_increment;

            // Sharp changes in direction can sometimes cause the distance to fail to converge: we limit the maximum number of iterations to avoid this
            // (It's possible for there to be multiple points 'distance' away or two equidistant points around the target point, 
            // and for this algorithm to fail to converge as a result)
            count               += 1;
            if count >= MAX_ITERATIONS {
                break;
            }
        }

        // next_t -> last_t is the next point
        if next_t > 1.0 {
            let last_section = curve.section(last_t, 1.0);
            self.last_t = 1.0;
            return Some(last_section);
        }

        // Update the coordinates
        self.last_point     = next_point;
        self.last_increment = t_increment;
        self.last_t         = next_t;

        // Return the section that we found
        Some(self.curve.section(last_t, next_t))
    }
}

impl<'a, Curve: BezierCurve, DistanceIter: 'a+Iterator<Item=f64>> Iterator for VaryingWalkIterator<'a, Curve, DistanceIter> {
    type Item = CurveSection<'a, Curve>;

    fn next(&mut self) -> Option<Self::Item> {
        // Vary the distance for the next step according to the distance iterator
        if let Some(distance_iterator) = &mut self.distance_iterator {
            if let Some(distance) = distance_iterator.next() {
                // Update the distance in the 'even' iterator
                let ratio                           = distance / self.even_iterator.distance;
                self.even_iterator.distance         = distance;
                self.even_iterator.last_increment   *= ratio;
            } else {
                // No more distance changes
                self.distance_iterator = None;
            }
        }

        // Continue with the even iterator with the new distance
        self.even_iterator.next()
    }
}
