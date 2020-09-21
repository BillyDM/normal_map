use crate::linear_base::{LinearBaseF32, LinearBaseF64};
use crate::Unit;

/// Linear mapping.
///
/// Please note if you use `Unit::Decibels`, then the decibels
/// will be linearly mapped, not the raw amplitude.
pub struct LinearMapF32 {
    lin_base: LinearBaseF32,
    unit: Unit,
}

impl LinearMapF32 {
    /// Create a new `LinearMapF32` for linear mapping.
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// are what will be linearly mapped, not the raw amplitude.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range
    /// * max - the maximum of the range
    /// * unit - the type of unit
    pub fn new(min: f32, max: f32, unit: Unit) -> Self {
        Self {
            lin_base: LinearBaseF32::new(min, max),
            unit,
        }
    }

    /// Map an `f32` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f32) -> f32 {
        match self.unit {
            Unit::Decibels => self.normalize_db(value),
            _ => self.normalize_generic(value),
        }
    }

    #[inline(always)]
    fn normalize_db(&self, value: f32) -> f32 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        self.lin_base.normalize_db(value)
    }

    #[inline(always)]
    fn normalize_generic(&self, value: f32) -> f32 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        self.lin_base.normalize(value)
    }

    /// Map an array of `f32` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f32], out_normalized: &mut [f32]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        match self.unit {
            Unit::Decibels => {
                for i in 0..min_len {
                    output[i] = self.normalize_db(input[i])
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = self.normalize_generic(input[i])
                }
            }
        }
    }

    /// Un-map a normalized value to the corresponding `f32` value.
    pub fn denormalize(&self, normalized: f32) -> f32 {
        match self.unit {
            Unit::Decibels => self.denormalize_db(normalized),
            _ => self.denormalize_generic(normalized),
        }
    }

    #[inline(always)]
    fn denormalize_db(&self, normalized: f32) -> f32 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
        }

        self.lin_base.denormalize_db(normalized)
    }

    #[inline(always)]
    fn denormalize_generic(&self, normalized: f32) -> f32 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
        }

        self.lin_base.denormalize(normalized)
    }

    /// Un-map an array of normalized values to the corresponding `f32` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f32], out_values: &mut [f32]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        match self.unit {
            Unit::Decibels => {
                for i in 0..min_len {
                    output[i] = self.denormalize_db(input[i])
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = self.denormalize_generic(input[i])
                }
            }
        }
    }
}

/// Linear mapping.
///
/// Please note if you use `Unit::Decibels`, then the decibels
/// will be linearly mapped, not the raw amplitude.
pub struct LinearMapF64 {
    lin_base: LinearBaseF64,
    unit: Unit,
}

impl LinearMapF64 {
    /// Create a new `LinearMapF64` for linear mapping.
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// are what will be linearly mapped, not the raw amplitude.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range
    /// * max - the maximum of the range
    /// * unit - the type of unit
    pub fn new(min: f64, max: f64, unit: Unit) -> Self {
        Self {
            lin_base: LinearBaseF64::new(min, max),
            unit,
        }
    }

    /// Map an `f64` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f64) -> f64 {
        match self.unit {
            Unit::Decibels => self.normalize_db(value),
            _ => self.normalize_generic(value),
        }
    }

    #[inline(always)]
    fn normalize_db(&self, value: f64) -> f64 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        self.lin_base.normalize_db(value)
    }

    #[inline(always)]
    fn normalize_generic(&self, value: f64) -> f64 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        self.lin_base.normalize(value)
    }

    /// Map an array of `f64` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f64], out_normalized: &mut [f64]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        match self.unit {
            Unit::Decibels => {
                for i in 0..min_len {
                    output[i] = self.normalize_db(input[i])
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = self.normalize_generic(input[i])
                }
            }
        }
    }

    /// Un-map a normalized value to the corresponding `f64` value.
    pub fn denormalize(&self, normalized: f64) -> f64 {
        match self.unit {
            Unit::Decibels => self.denormalize_db(normalized),
            _ => self.denormalize_generic(normalized),
        }
    }

    #[inline(always)]
    fn denormalize_db(&self, normalized: f64) -> f64 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
        }

        self.lin_base.denormalize_db(normalized)
    }

    #[inline(always)]
    fn denormalize_generic(&self, normalized: f64) -> f64 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
        }

        self.lin_base.denormalize(normalized)
    }

    /// Un-map an array of normalized values to the corresponding `f64` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f64], out_values: &mut [f64]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        match self.unit {
            Unit::Decibels => {
                for i in 0..min_len {
                    output[i] = self.denormalize_db(input[i])
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = self.denormalize_generic(input[i])
                }
            }
        }
    }
}
