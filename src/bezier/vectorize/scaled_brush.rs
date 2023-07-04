use super::brush_stroke::*;
use super::distance_field::*;
use super::sampled_contour::*;
use super::scaled_distance_field::*;

///
/// Brush that returns a scaled version of a distance field for each daub
///
pub struct ScaledBrush<TDistanceField> {
    /// The base distance field for the brush
    distance_field: TDistanceField,

    /// The x offset to the center of the distance field (point we scale around)
    center_x: f64,

    /// The y offset to the center of the distance field (point we scale around)
    center_y: f64,

    /// The scale factor to apply to the radius in the daubs
    radius_scale: f64,
}

impl<TDistanceField> ScaledBrush<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    ///
    /// Creates a new scaled brush that will produce scaled versions of the supplied distance field
    ///
    pub fn from_distance_field(distance_field: TDistanceField) -> Self {
        // Scale around the center of the distance field
        let size                        = distance_field.field_size();
        let ContourSize(width, height)  = size;

        let center_x    = (width as f64) / 2.0;
        let center_y    = (height as f64) / 2.0;

        // Scale to the largest of the width/height
        let radius          = width.max(height);
        let radius_scale    = 1.0 / (radius as f64);

        ScaledBrush {
            distance_field, center_x, center_y, radius_scale
        }
    }
}

impl<'a, TDistanceField> DaubBrush for &'a ScaledBrush<TDistanceField> 
where
    TDistanceField: SampledSignedDistanceField,
{
    type DaubDistanceField = ScaledDistanceField<&'a TDistanceField>;

    #[inline]
    fn create_daub(&self, pos: impl crate::Coordinate + crate::Coordinate2D, radius: f64) -> Option<(Self::DaubDistanceField, ContourPosition)> {
        if radius > 0.0 {
            let x = pos.x() - self.center_x - 1.0;
            let y = pos.y() - self.center_y - 1.0;

            if x < 0.0 || y < 0.0 { return None; }

            let offset_x = x - x.floor();
            let offset_y = y - y.floor();

            let distance_field  = ScaledDistanceField::from_distance_field(&self.distance_field, radius * self.radius_scale, (offset_x, offset_y));
            let position        = ContourPosition(x.floor() as usize, y.floor() as usize);

            Some((distance_field, position))
        } else {
            None
        }
    }
}
