use crate::geo::coordinate::*;

///
/// Extra functions provided for coordinate types
///
pub trait CoordinateExt {
    ///
    /// Creates a unit vector along the x axis
    ///
    fn unit_vector() -> Self;
}

///
/// Extra functions introduced for 2D coordinate types
///
pub trait Coordinate2DExt {
    ///
    /// Creates a unit vector at an angle in radians measured from the x-axis
    ///
    fn unit_vector_at_angle(radians: impl Into<f64>) -> Self;
}

impl<T> CoordinateExt for T
where
    T: Coordinate,
{
    fn unit_vector() -> Self {
        let mut components = vec![0.0; Self::len()];
        components[0] = 1.0;

        Self::from_components(&components)
    }
}

impl<T> Coordinate2DExt for T
where
    T: Coordinate + Coordinate2D,
{
    fn unit_vector_at_angle(radians: impl Into<f64>) -> Self {
        let radians = radians.into();

        Self::from_components(&[radians.cos(), radians.sin()])
    }
}
