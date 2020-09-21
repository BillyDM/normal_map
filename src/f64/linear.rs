use super::linear_base;
use super::Unit;

use std::fmt::Debug;

/// Linear mapping.
///
/// Please note if you use `Unit::Decibels`, then the decibels
/// will be linearly mapped, not the raw amplitude.
#[derive(Debug)]
pub struct LinearMap {
    min: f64,
    max: f64,
    lin_base: linear_base::Base,
}

impl LinearMap {
    /// Create a new `LinearMap` for linear mapping.
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
            min,
            max,
            lin_base: linear_base::Base::new(min, max, unit),
        }
    }

    /// Map an `f64` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f64) -> f64 {
        match &self.lin_base {
            linear_base::Base::Generic(base) => self.normalize_generic(value, &base),
            linear_base::Base::DB(base) => self.normalize_db(value, &base),
            linear_base::Base::DBClamped(base) => self.normalize_db_clamped(value, &base),
        }
    }

    #[inline(always)]
    fn normalize_generic(&self, value: f64, lin_base: &linear_base::Generic) -> f64 {
        if value <= self.min {
            return 0.0;
        };
        if value >= self.max {
            return 1.0;
        };

        lin_base.normalize(value)
    }

    #[inline(always)]
    fn normalize_db(&self, value: f64, lin_base: &linear_base::DB) -> f64 {
        if value <= self.min {
            return 0.0;
        };
        if value >= self.max {
            return 1.0;
        };

        lin_base.normalize(value)
    }

    #[inline(always)]
    fn normalize_db_clamped(&self, value: f64, lin_base: &linear_base::DBClamped) -> f64 {
        if value <= self.min {
            return 0.0;
        };
        if value >= self.max {
            return 1.0;
        };

        lin_base.normalize(value)
    }

    /// Map an array of `f64` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f64], out_normalized: &mut [f64]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        match &self.lin_base {
            linear_base::Base::Generic(base) => {
                for i in 0..min_len {
                    output[i] = self.normalize_generic(input[i], &base);
                }
            }
            linear_base::Base::DB(base) => {
                for i in 0..min_len {
                    output[i] = self.normalize_db(input[i], &base);
                }
            }
            linear_base::Base::DBClamped(base) => {
                for i in 0..min_len {
                    output[i] = self.normalize_db_clamped(input[i], &base);
                }
            }
        }
    }

    /// Un-map a normalized value to the corresponding `f64` value.
    pub fn denormalize(&self, normalized: f64) -> f64 {
        match &self.lin_base {
            linear_base::Base::Generic(base) => self.denormalize_generic(normalized, &base),
            linear_base::Base::DB(base) => self.denormalize_db(normalized, &base),
            linear_base::Base::DBClamped(base) => self.denormalize_db_clamped(normalized, &base),
        }
    }

    #[inline(always)]
    fn denormalize_generic(&self, normalized: f64, lin_base: &linear_base::Generic) -> f64 {
        if normalized <= 0.0 {
            return self.min;
        }
        if normalized >= 1.0 {
            return self.max;
        }

        lin_base.denormalize(normalized)
    }

    #[inline(always)]
    fn denormalize_db(&self, normalized: f64, lin_base: &linear_base::DB) -> f64 {
        if normalized <= 0.0 {
            return self.min;
        }
        if normalized >= 1.0 {
            return self.max;
        }

        lin_base.denormalize(normalized)
    }

    #[inline(always)]
    fn denormalize_db_clamped(&self, normalized: f64, lin_base: &linear_base::DBClamped) -> f64 {
        if normalized <= 0.0 {
            return self.min;
        }
        if normalized >= 1.0 {
            return self.max;
        }

        lin_base.denormalize(normalized)
    }

    /// Un-map an array of normalized values to the corresponding `f64` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f64], out_values: &mut [f64]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        match &self.lin_base {
            linear_base::Base::Generic(base) => {
                for i in 0..min_len {
                    output[i] = self.denormalize_generic(input[i], &base);
                }
            }
            linear_base::Base::DB(base) => {
                for i in 0..min_len {
                    output[i] = self.denormalize_db(input[i], &base);
                }
            }
            linear_base::Base::DBClamped(base) => {
                for i in 0..min_len {
                    output[i] = self.denormalize_db_clamped(input[i], &base);
                }
            }
        }
    }
}
