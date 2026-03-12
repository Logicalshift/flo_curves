use super::geo::*;
use super::has_bounds::*;
use super::coordinate::*;

///
/// Trait implemented by things representing axis-aligned bounding boxes
/// 
pub trait BoundingBox : Geo+Sized {
    ///
    /// Returns a bounding box with the specified minimum and maximum coordinates
    /// 
    fn from_min_max(min: Self::Point, max: Self::Point) -> Self;

    ///
    /// Returns a bounding box containing the specified points
    /// 
    fn bounds_for_points<PointIter: IntoIterator<Item=Self::Point>>(points: PointIter) -> Self {
        let mut points = points.into_iter();

        // Initialise the bounding box with the first point
        let first_point = points.next();
        if let Some(first_point) = first_point {
            // min, max is just the first point initially
            let (mut min, mut max) = (first_point, first_point);

            // Update with the remainder of the points
            for point in points {
                min = Self::Point::from_smallest_components(min, point);
                max = Self::Point::from_biggest_components(max, point);
            }

            Self::from_min_max(min, max)
        } else {
            // If there are no points, then the result is the empty bounding box
            Self::empty()
        }
    }

    ///
    /// Returns the minimum point of this bounding box
    ///
    fn min(&self) -> Self::Point;

    ///
    /// Returns the maximum point of this bounding box
    /// 
    fn max(&self) -> Self::Point;

    ///
    /// Returns an empty bounding box
    /// 
    fn empty() -> Self {
        Self::from_min_max(Self::Point::origin(), Self::Point::origin())
    }

    ///
    /// True if this bounding box is empty
    /// 
    #[inline]
    fn is_empty(&self) -> bool {
        self.min() == self.max()
    }

    ///
    /// Creates the union of this and another bounding box
    /// 
    fn union_bounds(self, target: Self) -> Self {
        if self.is_empty() {
            target
        } else if target.is_empty() {
            self
        } else {
            Self::from_min_max(Self::Point::from_smallest_components(self.min(), target.min()), Self::Point::from_biggest_components(self.max(), target.max()))
        }
    }

    ///
    /// Returns true if this bounding box overlaps another
    /// 
    fn overlaps(&self, target: &Self) -> bool {
        let (min1, max1) = (self.min(), self.max());
        let (min2, max2) = (target.min(), target.max());

        for p_index in 0..Self::Point::len() {
            if min1.get(p_index) > max2.get(p_index) { return false; }
            if min2.get(p_index) > max1.get(p_index) { return false; }
        }

        true
    }
}

///
/// Type representing a bounding box
/// 
/// (Unlike a normal point tuple this always represents its bounds in minimum/maximum order)
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds<Point: Coordinate>(pub Point, pub Point);

impl<Point: Coordinate> BoundingBox for (Point, Point) {
    #[inline]
    fn from_min_max(min: Self::Point, max: Self::Point) -> Self {
        (min, max)
    }

    #[inline]
    fn min(&self) -> Self::Point {
        Point::from_smallest_components(self.0, self.1)
    }

    #[inline]
    fn max(&self) -> Self::Point {
        Point::from_biggest_components(self.0, self.1)
    }
}

impl<Point: Coordinate> HasBoundingBox for Bounds<Point> {
    fn get_bounding_box<Bounds: BoundingBox<Point=Self::Point>>(&self) -> Bounds {
        Bounds::from_min_max(self.min(), self.max())
    }
}

impl<Point: Coordinate> Geo for Bounds<Point> {
    type Point=Point;
}

impl<Point: Coordinate> BoundingBox for Bounds<Point> {
    #[inline]
    fn from_min_max(min: Self::Point, max: Self::Point) -> Self {
        Bounds(min, max)
    }

    #[inline]
    fn min(&self) -> Self::Point {
        self.0
    }

    #[inline]
    fn max(&self) -> Self::Point {
        self.1
    }
}

///
/// Transforms a bounding box to create a new axis-aligned bounding box
///
pub fn transform_bounding_box_axis_aligned<TBoundingBox>(source: &TBoundingBox, transform_point: &impl Fn(TBoundingBox::Point) -> TBoundingBox::Point) -> TBoundingBox
where
    TBoundingBox:           BoundingBox,
    TBoundingBox::Point:    Coordinate2D,
{
    // Compute the corners of the bounding box
    let min = source.min();
    let max = source.max();

    let corners = (
        min,
        TBoundingBox::Point::from_components(&[min.x(), max.y()]),
        max,
        TBoundingBox::Point::from_components(&[max.x(), min.y()]),
    );

    // Transform them
    let corners = [
        transform_point(corners.0).coords(),
        transform_point(corners.1).coords(),
        transform_point(corners.2).coords(),
        transform_point(corners.3).coords(),
    ];

    // Use the minimum/maximum values as the corners of the new bounding box
    let min_x = corners[0].0.min(corners[1].0).min(corners[2].0).min(corners[3].0);
    let min_y = corners[0].1.min(corners[1].1).min(corners[2].1).min(corners[3].1);
    let max_x = corners[0].0.max(corners[1].0).max(corners[2].0).max(corners[3].0);
    let max_y = corners[0].1.max(corners[1].1).max(corners[2].1).max(corners[3].1);

    TBoundingBox::from_min_max(TBoundingBox::Point::from_components(&[min_x, min_y]), TBoundingBox::Point::from_components(&[max_x, max_y]))
}
