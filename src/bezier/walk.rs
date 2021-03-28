use super::curve::*;
use super::section::*;

///
/// Walks a bezier curve by dividing it into a number of sections
///
/// These sections are uneven in length: they all advance equally by 't' value but the points will
/// be spaced according to the shape of the curve (will have an uneven distance between them) 
///
#[inline]
pub fn walk_curve_uneven<'a, Curve: BezierCurve>(curve: &'a Curve, num_subdivisions: usize) -> impl 'a+Iterator<Item=CurveSection<'a, Curve>> {
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
