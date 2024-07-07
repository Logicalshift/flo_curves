#[inline]
fn squared(val: f64) -> f64 { val * val }

///
/// Iterator that generates 1 dimensional distance field from a series of parabolas
///
pub struct MarchingParabolasIterator {
    parabola_intercepts: Vec<ParabolaIntercept>,
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

impl MarchingParabolasIterator {
    ///
    /// Creates a new marching parabolas iterator from a list of parabolas
    ///
    /// The input parabolas should be in x position order
    ///
    #[inline]
    pub fn new(ordered_parabolas: impl IntoIterator<Item=impl Into<Parabola>>) -> Self {
        // Collect the parabolas into a single vec
        let ordered_parabolas = ordered_parabolas.into_iter().map(|p| p.into()).collect::<Vec<_>>();

        // Where each parabola intercepts with the following parabola
        let mut parabola_intercepts = Vec::with_capacity(ordered_parabolas.len());

        parabola_intercepts.push(ParabolaIntercept { intercept_xpos: -f64::INFINITY, parabola: ordered_parabolas[0] });

        // Figure out how the parabolas occlude each other to create the hull
        for idx in 1..ordered_parabolas.len() {
            let current_parabola    = &ordered_parabolas[idx];
            let mut last_curve      = parabola_intercepts.last().unwrap();
            let mut intercept       = current_parabola.intercepts(&last_curve.parabola);

            while intercept <= parabola_intercepts.last().unwrap().intercept_xpos {
                parabola_intercepts.pop();

                last_curve  = parabola_intercepts.last().unwrap();
                intercept   = current_parabola.intercepts(&last_curve.parabola);
            }

            parabola_intercepts.push(ParabolaIntercept { intercept_xpos: intercept, parabola: *current_parabola });
        }

        MarchingParabolasIterator {
            parabola_intercepts
        }
    }
}

impl Iterator for MarchingParabolasIterator {
    type Item = DistanceSquared;

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

    #[test]
    fn parabola_intercepts_1() {
        let iterator = MarchingParabolasIterator::new(vec![
            Parabola { xpos: 4.0, ypos: 0.0 },
            Parabola { xpos: 6.0, ypos: 0.0 },
        ]);

        assert!(iterator.parabola_intercepts.len() == 2, "{:?}", iterator.parabola_intercepts);
        assert!(iterator.parabola_intercepts[0].parabola.xpos == 4.0, "{:?}", iterator.parabola_intercepts);
        assert!(iterator.parabola_intercepts[1].parabola.xpos == 6.0, "{:?}", iterator.parabola_intercepts);
    }

    #[test]
    fn parabola_intercepts_2() {
        let iterator = MarchingParabolasIterator::new(vec![
            Parabola { xpos: 4.0, ypos: 0.0 },
            Parabola { xpos: 5.0, ypos: 6.0 },
            Parabola { xpos: 6.0, ypos: 0.0 },
        ]);

        assert!(iterator.parabola_intercepts.len() == 2, "{:?}", iterator.parabola_intercepts);
        assert!(iterator.parabola_intercepts[0].parabola.xpos == 4.0, "{:?}", iterator.parabola_intercepts);
        assert!(iterator.parabola_intercepts[1].parabola.xpos == 6.0, "{:?}", iterator.parabola_intercepts);
    }

    #[test]
    fn parabola_intercepts_3() {
        let iterator = MarchingParabolasIterator::new(vec![
            Parabola { xpos: 4.0, ypos: 6.0 },
            Parabola { xpos: 5.0, ypos: 0.0 },
            Parabola { xpos: 6.0, ypos: 6.0 },
        ]);

        assert!(iterator.parabola_intercepts.len() == 3, "{:?}", iterator.parabola_intercepts);
        assert!(iterator.parabola_intercepts[0].parabola.xpos == 4.0, "{:?}", iterator.parabola_intercepts);
        assert!(iterator.parabola_intercepts[1].parabola.xpos == 5.0, "{:?}", iterator.parabola_intercepts);
        assert!(iterator.parabola_intercepts[2].parabola.xpos == 6.0, "{:?}", iterator.parabola_intercepts);
    }
}
