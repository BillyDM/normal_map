use crate::linear_base::{LinearBaseF32, LinearBaseF64};

/// Discrete `isize` integer mapping
///
/// A supplied enum may be used as well as long
/// as it implements `From<isize> + Into<isize> + Copy + Clone`.
pub struct DiscreteMapF32 {
    lin_base: LinearBaseF32,
}

impl DiscreteMapF32 {
    /// Create a new `NormalMap` with a discrete `isize` integer range.
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

        let lin_base = LinearBaseF32::new(min as f32, max as f32);

        Self { lin_base }
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

        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        self.lin_base.normalize(value)
    }

    /// Map an `f32` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize_f32(&self, value: f32) -> f32 {
        self.normalize_generic_f32(value)
    }

    #[inline(always)]
    fn normalize_generic_f32(&self, value: f32) -> f32 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
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
    pub fn normalize_array_f32(&self, in_values: &[f32], out_normalized: &mut [f32]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        for i in 0..min_len {
            output[i] = self.normalize_generic_f32(input[i]);
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
        if normalized == 0.0 {
            return (self.lin_base.min() as isize).into();
        }
        if normalized == 1.0 {
            return (self.lin_base.max() as isize).into();
        }

        (self.lin_base.denormalize(normalized).round() as isize).into()
    }

    /// Un-map a normalized value to the corresponding `f32` value.
    pub fn denormalize_f32(&self, normalized: f32) -> f32 {
        self.denormalize_generic_f32(normalized)
    }

    #[inline(always)]
    fn denormalize_generic_f32(&self, normalized: f32) -> f32 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
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
    pub fn denormalize_array_f32(&self, in_normalized: &[f32], out_values: &mut [f32]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        for i in 0..min_len {
            output[i] = self.denormalize_generic_f32(input[i]);
        }
    }
}

/// Discrete `isize` integer mapping
///
/// A supplied enum may be used as well as long
/// as it implements `From<isize> + Into<isize> + Copy + Clone`.
pub struct DiscreteMapF64 {
    lin_base: LinearBaseF64,
}

impl DiscreteMapF64 {
    /// Create a new `NormalMap` with a discrete `isize` integer range.
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

        let lin_base = LinearBaseF64::new(min as f64, max as f64);

        Self { lin_base }
    }

    /// Map a discrete `isize` value to the normalized range `[0.0, 1.0]`.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    pub fn normalize<T>(&self, value: T) -> f64
    where
        T: Into<isize> + Copy + Clone,
    {
        self.normalize_generic(value)
    }

    #[inline(always)]
    fn normalize_generic<T>(&self, value: T) -> f64
    where
        T: Into<isize> + Copy + Clone,
    {
        let value: isize = value.into();
        let value = value as f64;

        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
            return 1.0;
        };

        self.lin_base.normalize(value)
    }

    /// Map an `f64` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize_f64(&self, value: f64) -> f64 {
        self.normalize_generic_f64(value)
    }

    #[inline(always)]
    fn normalize_generic_f64(&self, value: f64) -> f64 {
        if value <= self.lin_base.min() {
            return 0.0;
        };
        if value >= self.lin_base.max() {
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
    pub fn normalize_array<T>(&self, in_values: &[T], out_normalized: &mut [f64])
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

    /// Map an array of `f64` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array_f64(&self, in_values: &[f64], out_normalized: &mut [f64]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        for i in 0..min_len {
            output[i] = self.normalize_generic_f64(input[i]);
        }
    }

    /// Un-map a normalized value to the corresponding discrete `isize` value.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    pub fn denormalize<T>(&self, normalized: f64) -> T
    where
        T: From<isize> + Copy + Clone,
    {
        self.denormalize_generic(normalized)
    }

    #[inline(always)]
    fn denormalize_generic<T>(&self, normalized: f64) -> T
    where
        T: From<isize> + Copy + Clone,
    {
        if normalized == 0.0 {
            return (self.lin_base.min() as isize).into();
        }
        if normalized == 1.0 {
            return (self.lin_base.max() as isize).into();
        }

        (self.lin_base.denormalize(normalized).round() as isize).into()
    }

    /// Un-map a normalized value to the corresponding `f64` value.
    pub fn denormalize_f64(&self, normalized: f64) -> f64 {
        self.denormalize_generic_f64(normalized)
    }

    #[inline(always)]
    fn denormalize_generic_f64(&self, normalized: f64) -> f64 {
        if normalized == 0.0 {
            return self.lin_base.min();
        }
        if normalized == 1.0 {
            return self.lin_base.max();
        }

        self.lin_base.denormalize(normalized).round()
    }

    /// Un-map an array of normalized values to the corresponding discrete `isize` value.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array<T>(&self, in_normalized: &[f64], out_values: &mut [T])
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

    /// Un-map an array of normalized values to the corresponding `f64` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array_f64(&self, in_normalized: &[f64], out_values: &mut [f64]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        for i in 0..min_len {
            output[i] = self.denormalize_generic_f64(input[i]);
        }
    }
}
