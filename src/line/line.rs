use super::coefficients::*;
use super::super::geo::*;

///
/// Represents a straight line
/// 
pub trait Line : Geo {
    ///
    /// Creates a new line from points
    /// 
    fn from_points(p1: Self::Point, p2: Self::Point) -> Self;

    ///
    /// Returns the two points that mark the start and end of this line
    /// 
    fn points(&self) -> (Self::Point, Self::Point);

    ///
    /// Given a value 't' from 0 to 1, returns the point at that position along the line
    /// 
    fn point_at_pos(&self, t: f64) -> Self::Point {
        let (p1, p2)    = self.points();
        let delta       = p2-p1;

        p1 + delta*t
    }

    ///
    /// Given a point (assumed to be on the line), returns the 't' value on the line
    /// 
    /// If the point is not on the line, this will return a t value where at least one of the components of the point matches with
    /// the point on the line.
    ///
    fn pos_for_point(&self, point: &Self::Point) -> f64 {
        let (p1, p2)    = self.points();
        let delta_line  = p2-p1;
        let delta_point = *point-p1;

        for component_idx in 0..Self::Point::len() {
            let line_component  = delta_line.get(component_idx);
            let point_component = delta_point.get(component_idx);

            if line_component.abs() > 0.000001 && point_component.abs() > 0.000001 {
                return point_component/line_component;
            }
        }

        // Result is 0 if there are no components that map to the line somehow
        0.0
    }
}

///
/// Trait implemented by a 2D line
/// 
pub trait Line2D {
    type Point: Coordinate+Coordinate2D;

    ///
    /// Returns the coefficients (a, b, c) for this line, such that ax+by+c = 0 for
    /// any point on the line and also such that a^2 + b^2 = 1
    /// 
    fn coefficients(&self) -> LineCoefficients;

    ///
    /// Returns the distance from a point to the closest point along this line
    ///
    /// Note that this will project the line to infinity so this can return a distance to a point outside of the start or end point
    /// of the line. To determine if this has occurred, the `pos_for_point()` call can be used to determine the `t` value for the
    /// closest point: it will return a value in the range `0.0..1.0` if the closest point is within the line.
    /// 
    fn distance_to(&self, p: &Self::Point) -> f64;

    ///
    /// Returns a value indicating which side of the line the specified point is on (+1, 0 or -1)
    ///
    fn which_side(&self, p: &Self::Point) -> i8;

    ///
    /// Returns the nearest point on the line to the specified point
    ///
    fn nearest_point(&self, point: &Self::Point) -> Self::Point;

    ///
    /// Returns the t values of the closest point on the line to the specified point
    ///
    fn nearest_pos(&self, point: &Self::Point) -> f64;
}

impl<Point> Geo for (Point, Point) 
where
    Point: Coordinate + Clone,
{
    type Point = Point;
}

///
/// Simplest line is just a tuple of two points
/// 
impl<Point> Line for (Point, Point) 
where
    Point: Coordinate+ Clone,
{
    ///
    /// Creates a new line from points
    ///
    #[inline]
    fn from_points(p1: Self::Point, p2: Self::Point) -> Self {
        (p1, p2)
    }

    ///
    /// Returns the two points that mark the start and end of this line
    /// 
    #[inline]
    fn points(&self) -> (Self::Point, Self::Point) {
        *self
    }
}

impl<L> Line2D for L 
where
    L:          Line,
    L::Point:   Coordinate2D + Coordinate + Clone,
{
    type Point = L::Point;

    ///
    /// Returns the coefficients (a, b, c) for this line, such that ax+by+c = 0 for
    /// any point on the line and also such that a^2 + b^2 = 1
    /// 
    #[inline]
    fn coefficients(&self) -> LineCoefficients {
        line_coefficients_2d(self)
    }

    ///
    /// Returns the distance from a point to this line
    /// 
    #[inline]
    fn distance_to(&self, p: &Self::Point) -> f64 {
        self.coefficients().distance_to(p)
    }

    ///
    /// Returns a value indicating which side of the line the specified point is on (+1, 0 or -1)
    ///
    #[inline]
    fn which_side(&self, p: &Self::Point) -> i8 {
        let (start, end) = self.points();

        let side = ((p.x()-start.x())*(end.y()-start.y()) - (p.y()-start.y())*(end.x()-start.x())).signum();

        if side < 0.0 {
            -1
        } else if side > 0.0 {
            1
        } else {
            0
        }
    }

    ///
    /// Returns the nearest point on the line to the specified point
    ///
    #[inline]
    fn nearest_point(&self, point: &Self::Point) -> Self::Point {
        self.coefficients().nearest_point(point)
    }

    ///
    /// Returns the t values of the closest point on the line to the specified point
    ///
    #[inline]
    fn nearest_pos(&self, point: &Self::Point) -> f64 {
        self.pos_for_point(&self.nearest_point(point))
    }
}
