use super::distance_field::*;
use super::sampled_contour::*;

use itertools::*;

use std::sync::*;

///
/// Cached data for a mip-map level in a distance field
///
pub struct DistanceFieldMipLevel {
    width:      usize,
    height:     usize,
    distances:  Vec<f64>
}

impl DistanceFieldMipLevel {
    ///
    /// Creates a mip level from a set of rows known to be even
    ///
    #[inline]
    fn from_rows(width: usize, height: usize, rows: impl IntoIterator<Item=impl IntoIterator<Item=f64>>) -> Self {
        // Read as 2x2 blocks by combining the rows then the columns into tuples
        let distances = rows.into_iter()
            .tuples()
            .flat_map(|(row1, row2)| row1.into_iter()
                .tuples()
                .zip(row2.into_iter().tuples())
                .map(|((a, b), (c, d))| (a+b+c+d)/4.0));

        Self {
            width:      width/2,
            height:     height/2,
            distances:  distances.collect()
        }
    }

    ///
    /// Creates the next mip level down from this one
    ///
    fn next_level(&self) -> Self {
        Self::from_rows(self.width, self.height,
            self.distances
                .chunks(self.width)
                .map(|row| row.iter().copied()))
    }

    ///
    /// Reads the distance at a point in this mip-map field
    ///
    #[inline]
    pub fn distance_at_point(&self, ContourPosition(x, y): ContourPosition) -> f64 {
        let x = if x >= self.width  { self.width - 1 }  else { x };
        let y = if y >= self.height { self.height - 1 } else { y };

        if let Some(distance) = self.distances.get(x + y*self.width) {
            *distance
        } else {
            1e20
        }
    }

    #[inline]
    pub fn field_size(&self) -> ContourSize {
        ContourSize(self.width, self.height)
    }
}

///
/// The mip-map distance field can be used to calculate the distances in scaled-down distance fields
///
/// Mainly useful in combination with ScaledDistanceField. Sharing the MipMap can be used to avoid recalculating
/// mip levels unnecessarily.
///
pub struct MipMapDistanceField<TDistanceField> {
    /// The top level of this distance field
    top_level: TDistanceField,

    /// The mip-map levels, when calculated (each is half the size of the previous one)
    mip_levels: Mutex<Vec<Arc<DistanceFieldMipLevel>>>,
}

impl<TDistanceField> From<TDistanceField> for MipMapDistanceField<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    #[inline]
    fn from(distance_field: TDistanceField) -> Self {
        Self::new(distance_field)
    }
}

impl<TDistanceField> MipMapDistanceField<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    ///
    /// Creates a new mip-mapped distance field. Mip levels are calculated on demand, so this call
    /// won't do any calculations
    ///
    pub fn new(distance_field: TDistanceField) -> Self {
        MipMapDistanceField {
            top_level:  distance_field,
            mip_levels: Mutex::new(vec![]),
        }
    }

    ///
    /// Retrieves the top-level distance field that this mipmap field is for
    ///
    #[inline]
    pub fn top_level_distance_field(&self) -> &TDistanceField {
        &self.top_level
    }

    ///
    /// Converts this mipmap back to its top-level distance field
    ///
    #[inline]
    pub fn to_top_level_distance_field(self) -> TDistanceField {
        self.top_level
    }

    ///
    /// Creates the top-level mip-map level
    ///
    fn create_top_mip_level(&self) -> Arc<DistanceFieldMipLevel> {
        // Create the initial mip-map field by iterating over the entire
        let ContourSize(width, height) = self.top_level.field_size();

        let top_mip_level = DistanceFieldMipLevel::from_rows(width, height,
            (0..height)
            .map(|ypos| (0..width)
                .map(move |xpos| self.top_level.distance_at_point(ContourPosition(xpos, ypos)))));

        Arc::new(top_mip_level)
    }

    ///
    /// Returns the 'nth' mip level, where level 0 is a distance field of half the size of the top-level distance field
    ///
    pub fn mip_level(&self, level: usize) -> Arc<DistanceFieldMipLevel> {
        let mut mip_levels = self.mip_levels.lock().unwrap();

        // Create the 0th mip level if it's not already in the list
        if mip_levels.len() == 0 {
            mip_levels.push(self.create_top_mip_level());
        }

        // Create the following mip levels, until 'level' exists
        while mip_levels.len() <= level {
            let next_level = mip_levels.last().unwrap().next_level();
            mip_levels.push(Arc::new(next_level));
        }

        Arc::clone(&mip_levels[level])
    }
}

impl<TDistanceField> SampledSignedDistanceField for MipMapDistanceField<TDistanceField>
where
    TDistanceField: SampledSignedDistanceField,
{
    type Contour = TDistanceField::Contour;

    #[inline]
    fn field_size(&self) -> ContourSize {
        self.top_level.field_size()
    }

    #[inline]
    fn distance_at_point(&self, pos: ContourPosition) -> f64 {
        self.top_level.distance_at_point(pos)
    }

    #[inline]
    fn as_contour<'a>(&'a self) -> &'a Self::Contour {
        self.top_level.as_contour()
    }
}
