///
/// Options that affect the fill algorithm
/// 
/// The default options are created using `FillOptions::default()`. These can be used to tweak
/// settings like this step size.
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FillSettings {
    /// The distance between one ray and the next
    pub (crate) step: f64,

    /// The maximum error to allow when performing curve fitting
    pub (crate) fit_error: f64,

    /// For concave fills, the minimum gap size that a fill can escape through
    pub (crate) min_gap: Option<f64>
}

impl FillSettings {
    ///
    /// Creates a new fill options from this one by setting the step
    /// 
    /// The step size defines how accurately the flood-filled region reflects the area defined by the
    /// ray-casting function. Higher steps will result in a faster but less accurate result.
    ///
    pub fn with_step(self, new_step: f64) -> FillSettings {
        let mut new_options = self;
        new_options.step = new_step;
        new_options
    }

    ///
    /// Creates a new fill options from this one by setting the curve fitting error
    /// 
    /// The curve fitting error indicates how precisely the generated curve fits against the points
    /// returned by the ray casting algorithm. Increasing this value reduces the precision of the
    /// fit, which may produce a simpler (and smoother) resulting path but which will not necessarily
    /// fit the points as well.
    ///
    pub fn with_fit_error(self, new_fit_error: f64) -> FillSettings {
        let mut new_options = self;
        new_options.fit_error = new_fit_error;
        new_options
    }

    ///
    /// Sets the minimum gap size that a fill can 'escape' through when moving between regions
    ///
    /// This makes it possible to fill regions that are not perfectly enclosed
    ///
    pub fn with_min_gap(self, new_min_gap: Option<f64>) -> FillSettings {
        let mut new_options = self;
        new_options.min_gap = new_min_gap;
        new_options
    }
}

impl Default for FillSettings {
    ///
    /// Creates the default set of fill options
    ///
    fn default() -> FillSettings {
        FillSettings {
            step:       2.0,
            fit_error:  0.5,
            min_gap:    Some(5.0)
        }
    }    
}
