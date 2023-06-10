use super::coordinate::*;

use std::ops::*;

/// Represents a 2D point
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Coord3(pub f64, pub f64, pub f64);

impl Coordinate3D for Coord3 {
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

    ///
    /// Z component of this coordinate
    /// 
    #[inline]
    fn z(&self) -> f64 {
        self.2
    }
}

impl Add<Coord3> for Coord3 {
    type Output=Coord3;

    #[inline]
    fn add(self, rhs: Coord3) -> Coord3 {
        Coord3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Coord3> for Coord3 {
    type Output=Coord3;

    #[inline]
    fn sub(self, rhs: Coord3) -> Coord3 {
        Coord3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Coord3 {
    type Output=Coord3;

    #[inline]
    fn mul(self, rhs: f64) -> Coord3 {
        Coord3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl From<(f64, f64, f64)> for Coord3 {
    fn from((x, y, z): (f64, f64, f64)) -> Coord3 {
        Coord3(x, y, z)
    }
}

impl Into<(f64, f64, f64)> for Coord3 {
    fn into(self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }
}

impl From<(f32, f32, f32)> for Coord3 {
    fn from((x, y, z): (f32, f32, f32)) -> Coord3 {
        Coord3(x as _, y as _, z as _)
    }
}

impl Into<(f32, f32, f32)> for Coord3 {
    fn into(self) -> (f32, f32, f32) {
        (self.0 as _, self.1 as _, self.2 as _)
    }
}

impl Coordinate for Coord3 {
    #[inline]
    fn from_components(components: &[f64]) -> Coord3 {
        Coord3(components[0], components[1], components[2])
    }

    #[inline]
    fn origin() -> Coord3 {
        Coord3(0.0, 0.0, 0.0)
    }

    #[inline]
    fn len() -> usize { 3 }

    #[inline]
    fn get(&self, index: usize) -> f64 { 
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("Coord3 only has three components")
        }
    }

    fn from_biggest_components(p1: Coord3, p2: Coord3) -> Coord3 {
        Coord3(f64::from_biggest_components(p1.0, p2.0), f64::from_biggest_components(p1.1, p2.1), f64::from_biggest_components(p1.2, p2.2))
    }

    fn from_smallest_components(p1: Coord3, p2: Coord3) -> Coord3 {
        Coord3(f64::from_smallest_components(p1.0, p2.0), f64::from_smallest_components(p1.1, p2.1), f64::from_smallest_components(p1.2, p2.2))
    }

    #[inline]
    fn distance_to(&self, target: &Coord3) -> f64 {
        let dist_x = target.0-self.0;
        let dist_y = target.1-self.1;
        let dist_z = target.2-self.2;

        f64::sqrt(dist_x*dist_x + dist_y*dist_y + dist_z*dist_z)
    }

    #[inline]
    fn dot(&self, target: &Self) -> f64 {
        self.0*target.0 + self.1*target.1 + self.2*target.2
    }
}
