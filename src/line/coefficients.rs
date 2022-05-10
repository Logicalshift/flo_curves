use super::line::*;
use super::super::geo::*;

///
/// The coefficients for a line
///
/// This is the value `LineCoefficients(a, b, c)` such that `a*x + b*y + c = 0`. If a, b, c are set to 0 then this
/// represents a point instead of a line. Typically, line coefficients are normalized such that `a*a + b*b = 1`.
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LineCoefficients(pub f64, pub f64, pub f64);

impl Into<(f64, f64, f64)> for LineCoefficients {
    #[inline]
    fn into(self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }
}

///
/// For a two-dimensional line, computes the coefficients of the line equation ax+by+c=0
/// These coefficients are not normalized, which is slightly more efficient than computing the normalized form. 
/// 
/// This will return (0,0,0) for a line where the start and end point are the same.
/// 
pub fn line_coefficients_2d_unnormalized<L: Line+?Sized>(line: &L) -> LineCoefficients
where
    L::Point: Coordinate+Coordinate2D,
{
    // Compute the offset 
    let (from, to)  = line.points();
    let offset      = to - from;

    // Compute values for a, b, c
    let LineCoefficients(a, b, c) = if offset.x() == 0.0 && offset.y() == 0.0 {
        // This is a point rather than a line
        return LineCoefficients(0.0, 0.0, 0.0);
    } else if offset.x().abs() > offset.y().abs() {
        // Derive a, b, c from y = ax+c
        let a = offset.y() / offset.x();
        let b = -1.0;
        let c = -(a*from.x() + b*from.y());

        if offset.x() > 0.0 {
            LineCoefficients(-a, -b, -c)
        } else {
            LineCoefficients(a, b, c)
        }
    } else {
        // Derive a, b, c from x = by+c
        let a = -1.0;
        let b = offset.x() / offset.y();
        let c = -(a*from.x() + b*from.y());

        if offset.y() > 0.0 {
            LineCoefficients(-a, -b, -c)
        } else {
            LineCoefficients(a, b, c)
        }
    };

    LineCoefficients(a, b, c)
}

///
/// For a two-dimensional line, computes the coefficients of the line equation ax+by+c=0, such that 
/// a^2+b^2 = 1. This normalized form means that `a*x + b*y + c` will return the distance that the
/// point `x`, `y` is from the line.
/// 
/// This will return (0,0,0) for a line where the start and end point are the same.
/// 
pub fn line_coefficients_2d<L: Line+?Sized>(line: &L) -> LineCoefficients
where
    L::Point: Coordinate+Coordinate2D,
{
    let LineCoefficients(a, b, c) = line_coefficients_2d_unnormalized(line);

    // Normalise so that a^2+b^2 = 1
    let factor      = (a*a + b*b).sqrt();
    let (a, b, c)   = (a/factor, b/factor, c/factor);

    LineCoefficients(a, b, c)
}

impl LineCoefficients {
    ///
    /// Returns true if these coefficients are for a point rather than a line
    ///
    #[inline]
    pub fn is_point(&self) -> bool {
        self.0 == 0.0 && self.1 == 0.0 && self.2 == 0.0
    }

    ///
    /// Returns the distance from a point to this line
    /// 
    #[inline]
    pub fn distance_to<Point>(&self, p: &Point) -> f64 
    where
        Point: Coordinate2D,
    {
        let LineCoefficients(a, b, c) = self;

        a*p.x() + b*p.y() + c
    }

    ///
    /// Returns the coefficients for a perpendicular line passing through the specified point
    ///
    #[inline]
    pub fn to_perpendicular_line<Point>(&self, pass_through: &Point) -> LineCoefficients
    where
        Point: Coordinate2D,
    {
        let LineCoefficients(a, b, _c) = self;

        let a2 = *b;
        let b2 = *a;
        let c2 = -a2 * pass_through.x() - b2 * pass_through.y();

        LineCoefficients(a2, b2, c2)
    }

    ///
    /// Returns the nearest point on this line to the specified point
    ///
    #[inline]
    pub fn nearest_point<Point>(&self, p: &Point) -> Point
    where
        Point: Coordinate + Coordinate2D,
    {
        let x                           = p.x();
        let y                           = p.y();
        let LineCoefficients(a, b, c)   = self;

        // Perpendicular line has form bx - ay + c2
        let c2      = -b*x + a*y;

        // Solve where the two lines meet (this is the nearest point)
        let near_x  = (-a*c - b*c2) / (a*a + b*b);
        let near_y  = (a*c2 - b*c) / (a*a + b*b);

        Point::from_components(&[near_x, near_y])
    }

    ///
    /// Given a y coordinate, returns the corresponding x coordinate on the line
    ///
    #[inline]
    pub fn x_for_y(&self, y: f64) -> f64 {
        let LineCoefficients(a, b, c) = self;

        (-b*y - c) / a
    }

    ///
    /// Given an x coordinate, returns the corresponding y coordinate on the line
    ///
    #[inline]
    pub fn y_for_x(&self, x: f64) -> f64 {
        let LineCoefficients(a, b, c) = self;

        (-a*x - c) / b
    }
}
