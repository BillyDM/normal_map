use crate::util::*;

pub struct LinearBaseF32 {
    min: f32,
    max: f32,
    range: f32,
    range_inv: f32,
}

impl LinearBaseF32 {
    pub fn new(min: f32, max: f32) -> Self {
        let range = max - min;
        let range_inv = if range == 0.0 { 0.0 } else { 1.0 / range };

        Self {
            min,
            max,
            range,
            range_inv,
        }
    }

    #[inline]
    pub fn normalize(&self, value: f32) -> f32 {
        (value - self.min) * self.range_inv
    }

    #[inline]
    pub fn normalize_db(&self, value: f32) -> f32 {
        (coeff_to_db_f32(value) - self.min) * self.range_inv
    }

    #[inline]
    pub fn denormalize(&self, normalized: f32) -> f32 {
        (normalized * self.range) + self.min
    }

    #[inline]
    pub fn denormalize_db(&self, normalized: f32) -> f32 {
        db_to_coeff_f32((normalized * self.range) + self.min)
    }

    #[inline]
    pub fn min(&self) -> f32 {
        self.min
    }

    #[inline]
    pub fn max(&self) -> f32 {
        self.max
    }
}


pub struct LinearBaseF64 {
    min: f64,
    max: f64,
    range: f64,
    range_inv: f64,
}

impl LinearBaseF64 {
    pub fn new(min: f64, max: f64) -> Self {
        let range = max - min;
        let range_inv = if range == 0.0 { 0.0 } else { 1.0 / range };

        Self {
            min,
            max,
            range,
            range_inv,
        }
    }

    #[inline]
    pub fn normalize(&self, value: f64) -> f64 {
        (value - self.min) * self.range_inv
    }

    #[inline]
    pub fn normalize_db(&self, value: f64) -> f64 {
        (coeff_to_db_f64(value) - self.min) * self.range_inv
    }

    #[inline]
    pub fn denormalize(&self, normalized: f64) -> f64 {
        (normalized * self.range) + self.min
    }

    #[inline]
    pub fn denormalize_db(&self, normalized: f64) -> f64 {
        db_to_coeff_f64((normalized * self.range) + self.min)
    }

    #[inline]
    pub fn min(&self) -> f64 {
        self.min
    }

    #[inline]
    pub fn max(&self) -> f64 {
        self.max
    }
}
