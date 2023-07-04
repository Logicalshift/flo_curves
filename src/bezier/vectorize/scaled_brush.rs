use super::brush_stroke::*;
use super::distance_field::*;
use super::sampled_contour::*;
use super::scaled_distance_field::*;

///
/// Brush that returns a scaled version of a distance field for each daub
///
pub struct ScaledBrush<TDistanceField> 
where
    TDistanceField: SampledSignedDistanceField,
{
    distance_field: TDistanceField,
    center_x: f64,
    center_y: f64,
    radius_scale: f64,
}

impl<TDistanceField> DaubBrush for ScaledBrush<TDistanceField> 
where
    TDistanceField: SampledSignedDistanceField,
{
    type DaubDistanceField = ScaledDistanceField<TDistanceField>;

    #[inline]
    fn create_daub(&self, pos: impl crate::Coordinate + crate::Coordinate2D, radius: f64) -> Option<(Self::DaubDistanceField, ContourPosition)> {
        if radius > 0.0 {
            let x = pos.x() - self.center_x - 1.0;
            let y = pos.y() - self.center_y - 1.0;

            if x < 0.0 || y < 0.0 { return None; }

            // TODO: the scaled distance field needs to support the offset too
            let offset_x = x - x.floor();
            let offset_y = y - y.floor();

            let circle      = ScaledDistanceField::from_distance_field(self.distance_field, radius * self.radius_scale, (offset_x, offset_y));
            let position    = ContourPosition(x.floor() as usize, y.floor() as usize);

            Some((circle, position))
        } else {
            None
        }
    }
}
