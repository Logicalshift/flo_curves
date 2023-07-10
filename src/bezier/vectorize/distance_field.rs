use super::sampled_contour::*;

use smallvec::*;

use std::ops::{Range};

///
/// A distance field represents a sampling of how far certain discrete points are from an edge in an image.
/// This is a signed distance field, where negative distances are used to indicate samples that are inside a shape.
///
/// This can be used to more precisely position points than is possible using a `SampledContour` alone.
///
/// Distances are typically in pixels, however some implementations (eg U8SampledDistanceField) may use arbitrary units.
/// (The units typically don't matter when searching for the edge as '0' is a common point)
///
pub trait SampledSignedDistanceField {
    /// A type that can represent the edge contour for this distance field (see `ContourFromDistanceField` for a basic implementation)
    type Contour: SampledContour;

    ///
    /// The size of this distance field
    ///
    fn field_size(&self) -> ContourSize;

    ///
    /// Returns the distance to the nearest edge of the specified point (a negative value if the point is inside the shape)
    ///
    fn distance_at_point(&self, pos: ContourPosition) -> f64;

    ///
    /// Returns an edge contour for this distance field
    ///
    fn as_contour<'a>(&'a self) -> &'a Self::Contour;
}

impl<'a, T> SampledSignedDistanceField for &'a T
where
    T: SampledSignedDistanceField,
{
    type Contour = T::Contour;

    #[inline] fn field_size(&self) -> ContourSize { (*self).field_size() }
    #[inline] fn distance_at_point(&self, pos: ContourPosition) -> f64 { (*self).distance_at_point(pos) }
    #[inline] fn as_contour<'b>(&'b self) -> &'b Self::Contour { (*self).as_contour() }
}

///
/// Converts a signed distance field into a sampled contour
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ContourFromDistanceField<'a, TDistanceField>(pub &'a TDistanceField)
where
    TDistanceField: SampledSignedDistanceField;

impl<'a, TDistanceField> ContourFromDistanceField<'a, TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    #[inline]
    fn point_is_inside(&self, pos: ContourPosition) -> bool {
        self.0.distance_at_point(pos) <= 0.0
    }
}

impl<'a, TDistanceField> SampledContour for ContourFromDistanceField<'a, TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    #[inline]
    fn contour_size(&self) -> ContourSize {
        self.0.field_size()
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

///
/// Represents a signed distance field sampled with f32 values (>0 for values outside of the shape)
///
/// The vec here is represents the whole distance field from the top-left coordinate: it should be of size
/// ContourSize.0 * ContourSize.1
///
#[derive(Clone, PartialEq, Debug)]
pub struct F32SampledDistanceField(pub ContourSize, pub Vec<f32>);

///
/// Represents a signed distance field sampled with f64 values (>0 for values outside of the shape)
///
/// The vec here is represents the whole distance field from the top-left coordinate: it should be of size
/// ContourSize.0 * ContourSize.1
///
#[derive(Clone, PartialEq, Debug)]
pub struct F64SampledDistanceField(pub ContourSize, pub Vec<f64>);

///
/// Represents a signed distance field sampled with u8 values (>127 for values outside of the shape)
///
/// The vec here is represents the whole distance field from the top-left coordinate: it should be of size
/// ContourSize.0 * ContourSize.1
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct U8SampledDistanceField(pub ContourSize, pub Vec<u8>);

impl SampledSignedDistanceField for F32SampledDistanceField {
    type Contour = Self;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.0
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        let width   = self.0.0;
        let height  = self.0.1;

        if pos.0 < width && pos.1 < height {
            let pos = pos.0 + (pos.1 * width);
            self.1[pos] as _
        } else {
            f64::MAX
        }
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        self
    }
}

impl SampledContour for F32SampledDistanceField {
    #[inline] fn contour_size(&self) -> ContourSize { self.field_size() }
    #[inline] fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> { ContourFromDistanceField(self).intercepts_on_line(y )}
}

impl SampledSignedDistanceField for F64SampledDistanceField {
    type Contour = Self;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.0
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        let width   = self.0.0;
        let height  = self.0.1;

        if pos.0 < width && pos.1 < height {
            let pos = pos.0 + (pos.1 * width);
            self.1[pos]
        } else {
            f64::MAX
        }
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        self
    }
}

impl SampledContour for F64SampledDistanceField {
    #[inline] fn contour_size(&self) -> ContourSize { self.field_size() }
    #[inline] fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> { ContourFromDistanceField(self).intercepts_on_line(y )}
}

impl SampledSignedDistanceField for U8SampledDistanceField {
    type Contour = Self;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.0
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        let width   = self.0.0;
        let pos     = pos.0 + (pos.1 * width);

        (self.1[pos] as f64) - 127.0
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        self
    }
}

impl SampledContour for U8SampledDistanceField {
    #[inline] fn contour_size(&self) -> ContourSize { self.field_size() }
    #[inline] fn intercepts_on_line(&self, y: f64) -> SmallVec<[Range<f64>; 4]> { ContourFromDistanceField(self).intercepts_on_line(y )}
}

