use super::coordinate::*;

use std::ops::*;

/// Represents a 2D point
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Coord2(pub f64, pub f64);

impl Coord2 {
    ///
    /// Creates a Coord2 from any other implementation of Coordinate2D
    ///
    #[inline]
    pub fn from_coordinate(coord: impl Coordinate2D) -> Coord2 {
        Coord2(coord.x(), coord.y())
    }
}

impl Coordinate2D for Coord2 {
    ///
    /// X component of this coordinate
    /// 
    #[inline]
    fn x(&self) -> f64 {
        self.0
    }

    ///
    /// Y component of this coordinate
    /// 
    #[inline]
    fn y(&self) -> f64 {
        self.1
    }
}

impl Add<Coord2> for Coord2 {
    type Output=Coord2;

    #[inline]
    fn add(self, rhs: Coord2) -> Coord2 {
        Coord2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Coord2> for Coord2 {
    type Output=Coord2;

    #[inline]
    fn sub(self, rhs: Coord2) -> Coord2 {
        Coord2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f64> for Coord2 {
    type Output=Coord2;

    #[inline]
    fn mul(self, rhs: f64) -> Coord2 {
        Coord2(self.0 * rhs, self.1 * rhs)
    }
}

impl From<(f64, f64)> for Coord2 {
    fn from((x, y): (f64, f64)) -> Coord2 {
        Coord2(x, y)
    }
}

impl Into<(f64, f64)> for Coord2 {
    fn into(self) -> (f64, f64) {
        (self.0, self.1)
    }
}

impl From<(f32, f32)> for Coord2 {
    fn from((x, y): (f32, f32)) -> Coord2 {
        Coord2(x as _, y as _)
    }
}

impl Into<(f32, f32)> for Coord2 {
    fn into(self) -> (f32, f32) {
        (self.0 as _, self.1 as _)
    }
}

impl Coordinate for Coord2 {
    #[inline]
    fn from_components(components: &[f64]) -> Coord2 {
        Coord2(components[0], components[1])
    }

    #[inline]
    fn origin() -> Coord2 {
        Coord2(0.0, 0.0)
    }

    #[inline]
    fn len() -> usize { 2 }

    #[inline]
    fn get(&self, index: usize) -> f64 { 
        match index {
            0 => self.0,
            1 => self.1,
            _ => panic!("Coord2 only has two components")
        }
    }

    fn from_biggest_components(p1: Coord2, p2: Coord2) -> Coord2 {
        Coord2(f64::from_biggest_components(p1.0, p2.0), f64::from_biggest_components(p1.1, p2.1))
    }

    fn from_smallest_components(p1: Coord2, p2: Coord2) -> Coord2 {
        Coord2(f64::from_smallest_components(p1.0, p2.0), f64::from_smallest_components(p1.1, p2.1))
    }

    #[inline]
    fn distance_to(&self, target: &Coord2) -> f64 {
        let dist_x = target.0-self.0;
        let dist_y = target.1-self.1;

        f64::sqrt(dist_x*dist_x + dist_y*dist_y)
    }

    #[inline]
    fn dot(&self, target: &Self) -> f64 {
        self.0*target.0 + self.1*target.1
    }
}
