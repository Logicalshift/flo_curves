use std::vec;

#[inline]
fn squared(val: f64) -> f64 { val * val }

///
/// Iterator that generates 1 dimensional distance field from a series of parabolas
///
pub struct MarchingParabolasIterator<TXposIterator> {
    /// An iterator of x positions in this item
    x_positions:            TXposIterator,

    /// The intercept that we're currently processing
    current_intercept:      ParabolaIntercept,

    /// The intercept that we'll process next (or None if the current intercept is the only one left)
    next_intercept:         Option<ParabolaIntercept>,

    /// Intercepts following the next intercept
    following_intercepts:   vec::IntoIter<ParabolaIntercept>,
}

///
/// Square of the distance to a point
///
pub struct DistanceSquared(pub f64);

///
/// Description of a parabola for the marching parabolas algorithm
///
/// These parabolas are all of the form `y = (x-p)^2 + q`, indicating the squared distance from a point. A parabola
/// at x=0 
///
#[derive(Copy, Clone, Debug)]
pub struct Parabola {
    /// The x coordinate of the lowest point of this parabola
    pub xpos: f64,

    /// The lowest value for this parabola
    pub ypos: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct ParabolaIntercept {
    /// Where this parabola intercepts the following parabola
    pub intercept_xpos: f64,

    /// The parabola which has the intercept
    pub parabola: Parabola
}

impl<TXposIterator> MarchingParabolasIterator<TXposIterator> 
where
    TXposIterator: Iterator<Item = f64>
{
    ///
    /// Creates a new marching parabolas iterator from a list of parabolas. This will calculate distance squared values at each x position
    /// from the `ordered_x_positions` list, as the minimum value from the parabolas supplied in the `ordered_parabolas` list.
    ///
    /// The input parabolas should be in x position order, and the x positions that we should calculate distance values for should
    /// also be in 
    ///
    pub fn new<TIntoXposIterator>(ordered_parabolas: impl IntoIterator<Item=impl Into<Parabola>>, ordered_x_postions: TIntoXposIterator) -> Self
    where
        TIntoXposIterator: IntoIterator<IntoIter=TXposIterator>
    {
        // Collect the parabolas into a single vec
        let mut ordered_parabolas = ordered_parabolas.into_iter().map(|p| p.into());

        // Where each parabola intercepts with the following parabola
        let mut parabola_intercepts = vec![];

        parabola_intercepts.push(ParabolaIntercept { intercept_xpos: -f64::INFINITY, parabola: ordered_parabolas.next().unwrap() });

        // Figure out how the parabolas occlude each other to create the hull
        for current_parabola in ordered_parabolas {
            let mut last_curve      = parabola_intercepts.last().unwrap();
            let mut intercept       = current_parabola.intercepts(&last_curve.parabola);

            while intercept <= parabola_intercepts.last().unwrap().intercept_xpos {
                parabola_intercepts.pop();

                last_curve  = parabola_intercepts.last().unwrap();
                intercept   = current_parabola.intercepts(&last_curve.parabola);
            }

            parabola_intercepts.push(ParabolaIntercept { intercept_xpos: intercept, parabola: current_parabola });
        }

        // We'll interate through the intercepts in a left-to-right fashion
        let mut following_intercepts    = parabola_intercepts.into_iter();
        let current_intercept           = following_intercepts.next().unwrap();
        let next_intercept              = following_intercepts.next();
        let x_positions                 = ordered_x_postions.into_iter();

        MarchingParabolasIterator {
            following_intercepts, current_intercept, next_intercept, x_positions,
        }
    }
}

impl<TXposIterator> Iterator for MarchingParabolasIterator<TXposIterator>
where
    TXposIterator: Iterator<Item = f64>
{
    type Item = DistanceSquared;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl Parabola {
    ///
    /// Finds the x-position where this parabola intercepts another parabola
    ///
    /// 'other' must not have the same x position as this parabola or this will find a division by 0 instead of an intercept
    ///
    #[inline]
    pub fn intercepts(&self, other: &Parabola) -> f64 {
        let denom = 2.0*other.xpos - 2.0*self.xpos;

        (other.ypos - self.ypos - squared(self.xpos) + squared(other.xpos))/denom
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn all_remaining_intercepts<T>(iterator: MarchingParabolasIterator<T>) -> Vec<ParabolaIntercept> {
        let mut result = vec![];

        result.push(iterator.current_intercept);
        if let Some(next_intercept) = &iterator.next_intercept {
            result.push(*next_intercept);
        }
        result.extend(iterator.following_intercepts);

        result
    }

    #[test]
    fn parabola_intercepts_1() {
        let iterator = MarchingParabolasIterator::new(vec![
            Parabola { xpos: 4.0, ypos: 0.0 },
            Parabola { xpos: 6.0, ypos: 0.0 },
        ], vec![0.0]);
        let parabola_intercepts = all_remaining_intercepts(iterator); 

        assert!(parabola_intercepts.len() == 2, "{:?}", parabola_intercepts);
        assert!(parabola_intercepts[0].parabola.xpos == 4.0, "{:?}", parabola_intercepts);
        assert!(parabola_intercepts[1].parabola.xpos == 6.0, "{:?}", parabola_intercepts);
    }

    #[test]
    fn parabola_intercepts_2() {
        let iterator = MarchingParabolasIterator::new(vec![
            Parabola { xpos: 4.0, ypos: 0.0 },
            Parabola { xpos: 5.0, ypos: 6.0 },
            Parabola { xpos: 6.0, ypos: 0.0 },
        ], vec![0.0]);
        let parabola_intercepts = all_remaining_intercepts(iterator); 

        assert!(parabola_intercepts.len() == 2, "{:?}", parabola_intercepts);
        assert!(parabola_intercepts[0].parabola.xpos == 4.0, "{:?}", parabola_intercepts);
        assert!(parabola_intercepts[1].parabola.xpos == 6.0, "{:?}", parabola_intercepts);
    }

    #[test]
    fn parabola_intercepts_3() {
        let iterator = MarchingParabolasIterator::new(vec![
            Parabola { xpos: 4.0, ypos: 6.0 },
            Parabola { xpos: 5.0, ypos: 0.0 },
            Parabola { xpos: 6.0, ypos: 6.0 },
        ], vec![0.0]);
        let parabola_intercepts = all_remaining_intercepts(iterator); 

        assert!(parabola_intercepts.len() == 3, "{:?}", parabola_intercepts);
        assert!(parabola_intercepts[0].parabola.xpos == 4.0, "{:?}", parabola_intercepts);
        assert!(parabola_intercepts[1].parabola.xpos == 5.0, "{:?}", parabola_intercepts);
        assert!(parabola_intercepts[2].parabola.xpos == 6.0, "{:?}", parabola_intercepts);
    }
}
