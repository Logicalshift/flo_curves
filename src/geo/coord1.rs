use super::coordinate::*;

impl Coordinate for f64 {
    fn from_components(components: &[f64]) -> f64 {
        components[0]
    }

    #[inline] fn origin() -> f64 { 0.0 }
    #[inline] fn len() -> usize { 1 }
    #[inline] fn get(&self, _index: usize) -> f64 { *self }

    #[inline]
    fn from_biggest_components(p1: f64, p2: f64) -> f64 {
        if p1 > p2 {
            p1
        } else {
            p2
        }
    }

    #[inline]
    fn from_smallest_components(p1: f64, p2: f64) -> f64 {
        if p1 < p2 {
            p1
        } else {
            p2
        }
    }

    #[inline]
    fn distance_to(&self, target: &f64) -> f64 {
        f64::abs(self-target)
    }

    fn dot(&self, target: &f64) -> f64 {
        self * target
    }
}
