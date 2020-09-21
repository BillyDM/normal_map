use std::fmt::Debug;

use super::linear_base;

/// Discrete `isize` integer mapping
///
/// A supplied enum may be used as well as long
/// as it implements `From<isize> + Into<isize> + Copy + Clone`.
#[derive(Debug)]
pub struct DiscreteMap {
    min: f32,
    max: f32,
    lin_base: linear_base::Generic,
}

impl DiscreteMap {
    /// Create a new `DiscreteMap` with a discrete `isize` integer range.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range
    /// * max - the maximum of the range
    pub fn new<T>(min: T, max: T) -> Self
    where
        T: Into<isize> + Copy + Clone,
    {
        let min: isize = min.into();
        let max: isize = max.into();

        let min = min as f32;
        let max = max as f32;

        let lin_base = linear_base::Generic::new(min, max);

        Self { min, max, lin_base }
    }

    /// Map a discrete `isize` value to the normalized range `[0.0, 1.0]`.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    pub fn normalize<T>(&self, value: T) -> f32
    where
        T: Into<isize> + Copy + Clone,
    {
        self.normalize_generic(value)
    }

    #[inline(always)]
    fn normalize_generic<T>(&self, value: T) -> f32
    where
        T: Into<isize> + Copy + Clone,
    {
        let value: isize = value.into();
        let value = value as f32;

        if value <= self.min {
            return 0.0;
        };
        if value >= self.max {
            return 1.0;
        };

        self.lin_base.normalize(value)
    }

    /// Map an `f32` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize_float(&self, value: f32) -> f32 {
        self.normalize_generic_float(value)
    }

    #[inline(always)]
    fn normalize_generic_float(&self, value: f32) -> f32 {
        if value <= self.min {
            return 0.0;
        };
        if value >= self.max {
            return 1.0;
        };

        self.lin_base.normalize(value.round())
    }

    /// Map an array of discrete `isize` values to the normalized range `[0.0, 1.0]`.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array<T>(&self, in_values: &[T], out_normalized: &mut [f32])
    where
        T: Into<isize> + Copy + Clone,
    {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        for i in 0..min_len {
            output[i] = self.normalize_generic(input[i]);
        }
    }

    /// Map an array of `f32` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array_float(&self, in_values: &[f32], out_normalized: &mut [f32]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        for i in 0..min_len {
            output[i] = self.normalize_generic_float(input[i]);
        }
    }

    /// Un-map a normalized value to the corresponding discrete `isize` value.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    pub fn denormalize<T>(&self, normalized: f32) -> T
    where
        T: From<isize> + Copy + Clone,
    {
        self.denormalize_generic(normalized)
    }

    #[inline(always)]
    fn denormalize_generic<T>(&self, normalized: f32) -> T
    where
        T: From<isize> + Copy + Clone,
    {
        if normalized <= 0.0 {
            return (self.min as isize).into();
        }
        if normalized >= 1.0 {
            return (self.max as isize).into();
        }

        (self.lin_base.denormalize(normalized).round() as isize).into()
    }

    /// Un-map a normalized value to the corresponding `f32` value.
    pub fn denormalize_float(&self, normalized: f32) -> f32 {
        self.denormalize_generic_float(normalized)
    }

    #[inline(always)]
    fn denormalize_generic_float(&self, normalized: f32) -> f32 {
        if normalized <= 0.0 {
            return self.min;
        }
        if normalized >= 1.0 {
            return self.max;
        }

        self.lin_base.denormalize(normalized).round()
    }

    /// Un-map an array of normalized values to the corresponding discrete `isize` value.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array<T>(&self, in_normalized: &[f32], out_values: &mut [T])
    where
        T: From<isize> + Copy + Clone,
    {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        for i in 0..min_len {
            output[i] = self.denormalize_generic(input[i]);
        }
    }

    /// Un-map an array of normalized values to the corresponding `f32` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array_float(&self, in_normalized: &[f32], out_values: &mut [f32]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        for i in 0..min_len {
            output[i] = self.denormalize_generic_float(input[i]);
        }
    }
}
