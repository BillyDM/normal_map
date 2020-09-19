use crate::linear_base::{LinearBaseF32, LinearBaseF64};
use crate::Unit;

/// Exponential mapping where the normalized value is raised to the
/// supplied exponent.
///
/// Please note if you use `Unit::Decibels`, then the decibels
/// will be linearly mapped, not the raw amplitude.
pub struct PowerMapF32 {
    lin_base: LinearBaseF32,
    exponent: f32,
    exponent_inv: f32,
    unit: Unit,
}

impl PowerMapF32 {
        /// Create a new `PowerMapF32` for exponential mapping where the
        /// normalized value is raised to the supplied exponent.
        ///
        /// Please note if you use `Unit::Decibels`, then the decibels
        /// are what will be mapped, not the raw amplitude.
        ///
        /// # Arguments
        ///
        /// * min - the minimum of the range
        /// * max - the maximum of the range
        /// * exponent - the exponent to raise the normalized value to
        /// * unit - the type of unit
        ///
        /// # Panics
        ///
        /// * Panics when `exponent = 0.0`.
    pub fn new(min: f32, max: f32, exponent: f32, unit: Unit) -> Self {
        if exponent == 0.0 {
            panic!("Exponent cannot be 0");
        }

        let exponent_inv = 1.0 / exponent;

        let lin_base = LinearBaseF32::new(min, max);

        Self {
            lin_base,
            exponent,
            exponent_inv,
            unit,
        }
    }

    #[inline]
    /// Map an `f32` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f32) -> f32 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        let lin_baseped = match self.unit {
            Unit::Decibels => self.lin_base.normalize_db(value),
            _ => self.lin_base.normalize(value),
        };

        lin_baseped.powf(self.exponent_inv)
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
                    output[i] = if input[i] <= self.lin_base.min() {
                        0.0
                    } else if input[i] >= self.lin_base.max() {
                        1.0
                    } else {
                        let lin_baseped = self.lin_base.normalize_db(input[i]);
                        lin_baseped.powf(self.exponent_inv)
                    };
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = if input[i] <= self.lin_base.min() {
                        0.0
                    } else if input[i] >= self.lin_base.max() {
                        1.0
                    } else {
                        let lin_baseped = self.lin_base.normalize(input[i]);
                        lin_baseped.powf(self.exponent_inv)
                    };
                }
            }
        }
    }

    #[inline]
    /// Un-map a normalized value to the corresponding `f32` value.
    pub fn denormalize(&self, normalized: f32) -> f32 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
        }

        let value = normalized.powf(self.exponent);

        match self.unit {
            Unit::Decibels => self.lin_base.denormalize_db(value),
            _ => self.lin_base.denormalize(value),
        }
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
                    output[i] = if input[i] == 0.0 {
                        self.lin_base.min()
                    } else if input[i] == 1.0 {
                        self.lin_base.max()
                    } else {
                        let value = input[i].powf(self.exponent);
                        self.lin_base.denormalize_db(value)
                    };
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = if input[i] == 0.0 {
                        self.lin_base.min()
                    } else if input[i] == 1.0 {
                        self.lin_base.max()
                    } else {
                        let value = input[i].powf(self.exponent);
                        self.lin_base.denormalize(value)
                    };
                }
            }
        }
    }
}


/// Exponential mapping where the normalized value is raised to the
/// supplied exponent.
///
/// Please note if you use `Unit::Decibels`, then the decibels
/// will be linearly mapped, not the raw amplitude.
pub struct PowerMapF64 {
    lin_base: LinearBaseF64,
    exponent: f64,
    exponent_inv: f64,
    unit: Unit,
}

impl PowerMapF64 {
        /// Create a new `PowerMapF64` for exponential mapping where the
        /// normalized value is raised to the supplied exponent.
        ///
        /// Please note if you use `Unit::Decibels`, then the decibels
        /// are what will be mapped, not the raw amplitude.
        ///
        /// # Arguments
        ///
        /// * min - the minimum of the range
        /// * max - the maximum of the range
        /// * exponent - the exponent to raise the normalized value to
        /// * unit - the type of unit
        ///
        /// # Panics
        ///
        /// * Panics when `exponent = 0.0`.
    pub fn new(min: f64, max: f64, exponent: f64, unit: Unit) -> Self {
        if exponent == 0.0 {
            panic!("Exponent cannot be 0");
        }

        let exponent_inv = 1.0 / exponent;

        let lin_base = LinearBaseF64::new(min, max);

        Self {
            lin_base,
            exponent,
            exponent_inv,
            unit,
        }
    }

    #[inline]
    /// Map an `f64` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f64) -> f64 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        let lin_baseped = match self.unit {
            Unit::Decibels => self.lin_base.normalize_db(value),
            _ => self.lin_base.normalize(value),
        };

        lin_baseped.powf(self.exponent_inv)
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
                    output[i] = if input[i] <= self.lin_base.min() {
                        0.0
                    } else if input[i] >= self.lin_base.max() {
                        1.0
                    } else {
                        let lin_baseped = self.lin_base.normalize_db(input[i]);
                        lin_baseped.powf(self.exponent_inv)
                    };
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = if input[i] <= self.lin_base.min() {
                        0.0
                    } else if input[i] >= self.lin_base.max() {
                        1.0
                    } else {
                        let lin_baseped = self.lin_base.normalize(input[i]);
                        lin_baseped.powf(self.exponent_inv)
                    };
                }
            }
        }
    }

    #[inline]
    /// Un-map a normalized value to the corresponding `f64` value.
    pub fn denormalize(&self, normalized: f64) -> f64 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
        }

        let value = normalized.powf(self.exponent);

        match self.unit {
            Unit::Decibels => self.lin_base.denormalize_db(value),
            _ => self.lin_base.denormalize(value),
        }
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
                    output[i] = if input[i] == 0.0 {
                        self.lin_base.min()
                    } else if input[i] == 1.0 {
                        self.lin_base.max()
                    } else {
                        let value = input[i].powf(self.exponent);
                        self.lin_base.denormalize_db(value)
                    };
                }
            }
            _ => {
                for i in 0..min_len {
                    output[i] = if input[i] == 0.0 {
                        self.lin_base.min()
                    } else if input[i] == 1.0 {
                        self.lin_base.max()
                    } else {
                        let value = input[i].powf(self.exponent);
                        self.lin_base.denormalize(value)
                    };
                }
            }
        }
    }
}
