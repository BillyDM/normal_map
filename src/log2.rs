/// Logarithmic mapping using `log2`
pub struct Log2MapF32 {
    min: f32,
    max: f32,
    min_log2: f32,
    range_log2: f32,
    range_log2_inv: f32,
}

impl Log2MapF32 {
    /// Create a new `LogMap` for logarithmic mapping using `log2`.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range, must be > 0.0
    /// * max - the maximum of the range, must be > 0.0
    ///
    /// # Panics
    ///
    /// * Panics when either `min` or `max` <= 0.0.
    pub fn new(min: f32, max: f32) -> Self {
        assert!(min > 0.0);
        assert!(max > 0.0);

        let min_log2 = min.log2();
        let range_log2 = max.log2() - min_log2;

        let range_log2_inv = if range_log2 == 0.0 {
            0.0
        } else {
            1.0 / range_log2
        };

        Self {
            min,
            max,
            min_log2,
            range_log2,
            range_log2_inv,
        }
    }

    /// Map an `f32` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f32) -> f32 {
        self.normalize_generic(value)
    }

    #[inline(always)]
    fn normalize_generic(&self, value: f32) -> f32 {
        if value <= self.min {
            return 0.0;
        };
        if value >= self.max {
            return 1.0;
        };

        (value.log2() - self.min_log2) * self.range_log2_inv
    }

    /// Map an array of `f32` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f32], out_normalized: &mut [f32]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        for i in 0..min_len {
            output[i] = self.normalize_generic(input[i]);
        }
    }

    /// Un-map a normalized value to the corresponding `f32` value.
    pub fn denormalize(&self, normalized: f32) -> f32 {
        self.denormalize_generic(normalized)
    }

    #[inline(always)]
    fn denormalize_generic(&self, normalized: f32) -> f32 {
        if normalized == 0.0 {
            return self.min;
        }
        if normalized == 1.0 {
            return self.max;
        }

        2.0f32.powf((normalized * self.range_log2) + self.min_log2)
    }

    /// Un-map an array of normalized values to the corresponding `f32` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f32], out_values: &mut [f32]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        for i in 0..min_len {
            output[i] = self.denormalize_generic(input[i]);
        }
    }
}

/// Logarithmic mapping using `log2`
pub struct Log2MapF64 {
    min: f64,
    max: f64,
    min_log2: f64,
    range_log2: f64,
    range_log2_inv: f64,
}

impl Log2MapF64 {
    /// Create a new `LogMap` for logarithmic mapping using `log2`.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range, must be > 0.0
    /// * max - the maximum of the range, must be > 0.0
    ///
    /// # Panics
    ///
    /// * Panics when either `min` or `max` <= 0.0.
    pub fn new(min: f64, max: f64) -> Self {
        assert!(min > 0.0);
        assert!(max > 0.0);

        let min_log2 = min.log2();
        let range_log2 = max.log2() - min_log2;

        let range_log2_inv = if range_log2 == 0.0 {
            0.0
        } else {
            1.0 / range_log2
        };

        Self {
            min,
            max,
            min_log2,
            range_log2,
            range_log2_inv,
        }
    }

    /// Map an `f64` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f64) -> f64 {
        self.normalize_generic(value)
    }

    #[inline(always)]
    fn normalize_generic(&self, value: f64) -> f64 {
        if value <= self.min {
            return 0.0;
        };
        if value >= self.max {
            return 1.0;
        };

        (value.log2() - self.min_log2) * self.range_log2_inv
    }

    /// Map an array of `f64` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f64], out_normalized: &mut [f64]) {
        let min_len = std::cmp::min(in_values.len(), out_normalized.len());
        let input = &in_values[..min_len];
        let output = &mut out_normalized[..min_len];

        for i in 0..min_len {
            output[i] = self.normalize_generic(input[i]);
        }
    }

    /// Un-map a normalized value to the corresponding `f64` value.
    pub fn denormalize(&self, normalized: f64) -> f64 {
        self.denormalize_generic(normalized)
    }

    #[inline(always)]
    fn denormalize_generic(&self, normalized: f64) -> f64 {
        if normalized == 0.0 {
            return self.min;
        }
        if normalized == 1.0 {
            return self.max;
        }

        2.0f64.powf((normalized * self.range_log2) + self.min_log2)
    }

    /// Un-map an array of normalized values to the corresponding `f64` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f64], out_values: &mut [f64]) {
        let min_len = std::cmp::min(in_normalized.len(), out_values.len());
        let input = &in_normalized[..min_len];
        let output = &mut out_values[..min_len];

        for i in 0..min_len {
            output[i] = self.denormalize_generic(input[i]);
        }
    }
}
