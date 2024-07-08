use super::marching_parabolas::*;
use super::path_contour::*;
use crate::bezier::path::*;
use crate::bezier::vectorize::*;
use crate::geo::*;

use smallvec::*;
use itertools::*;

use std::ops::{Range};

#[inline] fn squared(val: f64) -> f64 { val * val }

///
/// Distance field that's computed by following the marching parabolas algorithm
///
pub struct MarchingParabolaDistanceField {
    /// Width of the distance field in pixels
    width: usize,

    /// Height of the distance field in pixels
    height: usize,

    /// width * height pixels indicating the squared distance
    squared_distance_field: Vec<f64>,
}

impl MarchingParabolaDistanceField {
    ///
    /// Computes a distance field using the marching parabolas algorithm given functions that calculate the X and Y intercepts against a
    /// shape.
    ///
    /// This takes the width and height of the distance fields and two functions. `intercepts_for_x(xpos)` takes an x position and returns
    /// where the shape intersects that column in ascending order, and `intercepts_for_y(ypos)` does the same except for a row along a y position.
    ///
    /// Error can be up to 1 pixel for this function but is usually much less as we can compute the parabolas to a higher precision than
    /// one pixel.
    ///
    pub fn from_intercepts<TXIterator, TYIterator>(width: usize, height: usize, intercepts_for_x: impl Fn(f64) -> TXIterator, intercepts_for_y: impl Fn(f64) -> TYIterator) -> Self 
    where
        TXIterator: IntoIterator<Item=Range<f64>>,
        TYIterator: IntoIterator<Item=Range<f64>>,
    {
        // Create a 1D distance field for the columns
        // TODO: generate the signed distance field, not just the exterior field 
        let mut x_distance_field = vec![f64::INFINITY; width * height];

        // Iterate over the columns to build the initial distance field
        for x in 0..width {
            let xpos            = x as f64;
            let mut intercepts  = intercepts_for_x(xpos).into_iter();

            if let Some(initial_intercept) = intercepts.next() {
                let mut current_intercept   = initial_intercept;
                let mut following_intercept = intercepts.next();

                // Fill in a column via these intercepts
                for (y, val) in x_distance_field.iter_mut().skip(x).step_by(width).enumerate() {
                    let ypos = y as f64;

                    loop {
                        if ypos >= current_intercept.start {
                            if ypos > current_intercept.end {
                                // Outside of the distance field
                                let distance_1 = squared(ypos - current_intercept.end);

                                if let Some(next_intercept) = &following_intercept {
                                    let distance_2 = squared(ypos - next_intercept.start);

                                    if distance_2 <= distance_1 {
                                        // Move to the next intercept (next range is closer)
                                        current_intercept   = next_intercept.clone();
                                        following_intercept = intercepts.next();

                                        // In case the next parabola is also closer, keep trying
                                        continue;
                                    } else {
                                        // Use the current distance
                                        *val = distance_1;
                                        break;
                                    }
                                } else {
                                    // The end is closer than the start of the next intercept
                                    *val = distance_1;
                                    break;
                                }
                            } else {
                                // Inside the distance field, these create negative distances (we assume the intercepts are non-overlapping so this is much simpler)
                                let distance_1 = squared(ypos - current_intercept.start);
                                let distance_2 = squared(ypos - current_intercept.end);

                                *val = -(distance_1.min(distance_2));
                                break;
                            }
                        } else {
                            // Outside of the distance field, but closer to the start of the current intercept than the next one
                            let offset = squared(ypos - current_intercept.start);
                            *val = offset;
                            break;
                        }
                    }
                }
            }
        }

        // Use the marching parabolas algorithm to fill in the remaining distance field
        let mut squared_distance_field = Vec::with_capacity(width * height);

        for y in 0..height {
            // Get the row that we sampled before
            let input_row    = &x_distance_field[y*width..(y*width+width)];
            let y_intercepts = intercepts_for_y(y as _).into_iter();

            // Use the marching parabolas algorithm to generate the 2D distance field
            let marching_parabolas = MarchingParabolasIterator::new(
                input_row.iter().enumerate()
                    .flat_map(|(x_pos, distance)| {     // <-- Distances from the first pass
                        if distance.is_infinite() {
                            None
                        } else {
                            Some(Parabola {
                                xpos: x_pos as f64,
                                ypos: distance.abs()
                            })
                        }
                    })
                    .merge_by(                          // <-- Intercepts computed along a column
                        y_intercepts.flat_map(|intercept| [
                            Parabola {
                                xpos: intercept.start,
                                ypos: 0.0,
                            },
                            Parabola {
                                xpos: intercept.end,
                                ypos: 0.0,
                            }
                        ]),
                        |a, b| a.xpos < b.xpos),
                    (0..width).map(|x| x as f64))
                .map(|DistanceSquared(distance)| distance)
                .zip(input_row)                         // <-- Take 'inside/outside' from the sign of the original input row
                .map(|(val, original)| if original < &0.0 { -val } else { val });

            squared_distance_field.extend(marching_parabolas);
        }

        // Return this as the result
        MarchingParabolaDistanceField {
            width, height, squared_distance_field
        }
    }

    ///
    /// Creates a distance field from a path (within a particular region)
    ///
    pub fn from_path_region(x_origin: f64, y_origin: f64, width: usize, height: usize, path: Vec<impl 'static + BezierPath<Point=impl Coordinate+Coordinate2D>>) -> Self {
        let contour = PathContour::from_path(path, ContourSize(width, height));

        Self::from_intercepts(width, height, 
            |x| contour.intercepts_on_column(x).into_iter().map(|y| (y.start-y_origin)..(y.end-y_origin)), 
            |y| contour.intercepts_on_line(y).into_iter().map(|x| (x.start-x_origin)..(x.end-x_origin)))
    }

    ///
    /// Creates a distance field from a path
    ///
    /// This will calculate the bounds of the path. The two f64 values are the x and y coordinates of the origin of the resulting distance field
    ///
    pub fn from_path(path: Vec<impl 'static + BezierPath<Point=impl Coordinate+Coordinate2D>>) -> (Self, f64, f64) {
        // Compute the bounding box of the path
        let bounds = path.iter()
            .map(|p| p.bounding_box::<Bounds<_>>())
            .reduce(|bounds1, bounds2| bounds1.union_bounds(bounds2))
            .unwrap_or(Bounds::empty());

        // Decide on the range to calculate a distance field for
        let origin_x = bounds.min().x() - 4.0;
        let origin_y = bounds.min().y() - 4.0;
        let width    = (bounds.max().x() - origin_x).ceil() + 4.0;
        let height   = (bounds.max().y() - origin_y).ceil() + 4.0;

        (Self::from_path_region(origin_x, origin_y, width as _, height as _, path), origin_x, origin_y)
    }

    ///
    /// Computes a distance field using the marching parabolas algorithm given an iterator that indicates whether or not a pixel is inside
    /// or outside of the shape.
    ///
    /// Calculated distances may be up to 1 pixel out compared to the source shape, but this can be used as a way to vectorize and scale
    /// silhouettes.
    ///
    pub fn from_bitfield() -> Self {
        todo!()
    }

    ///
    /// True if the specified point is inside in the contour
    ///
    #[inline]
    pub fn point_is_inside(&self, ContourPosition(x, y): ContourPosition) -> bool {
        self.squared_distance_field.get(x + y * self.width)
            .map(|distance| *distance <= 0.0)
            .unwrap_or(false)
    }
}

impl SampledSignedDistanceField for MarchingParabolaDistanceField {
    type Contour = Self;

    #[inline]
    fn field_size(&self) -> ContourSize {
        ContourSize(self.width, self.height)
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        let distance_squared = self.squared_distance_field[pos.0 + pos.1 * self.width];

        if distance_squared >= 0.0 {
            distance_squared.sqrt()
        } else {
            -(-distance_squared).sqrt()
        }
    }

    fn as_contour<'b>(&'b self) -> &'b Self::Contour {
        self
    }
}

impl SampledContour for MarchingParabolaDistanceField {
    #[inline]
    fn contour_size(&self) -> ContourSize {
        ContourSize(self.width, self.height)
    }

    fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> {
        let width   = self.contour_size().width();
        let y       = y.floor() as usize;

        let mut ranges = smallvec![];
        let mut inside = None;

        for x in 0..width {
            // Transitioning from 'outside' to 'inside' sets a start position, and doing the opposite generates a range
            match (inside, self.point_is_inside(ContourPosition(x, y))) {
                (None, true)            => { inside = Some(x); },
                (Some(start_x), false)  => {
                    inside = None;
                    ranges.push((start_x as f64)..(x as f64));
                }
                _ => { }
            }
        }

        if let Some(start_x) = inside {
            ranges.push((start_x as f64)..(width as f64));
        }

        ranges
    }
}
