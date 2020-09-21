use std::fmt::Debug;

use super::Unit;

#[derive(Debug)]
pub enum Base {
    Generic(Generic),
    DB(DB),
    DBClamped(DBClamped),
}

impl Base {
    pub fn new(min: f32, max: f32, unit: Unit) -> Self {
        match unit {
            Unit::Generic => Base::Generic(Generic::new(min, max)),
            Unit::Decibels { neg_infinity_clamp } => {
                if let Some(clamp_db) = neg_infinity_clamp {
                    Base::DBClamped(DBClamped::new(min, max, clamp_db))
                } else {
                    Base::DB(DB::new(min, max))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Generic {
    min: f32,
    range: f32,
    range_inv: f32,
}

impl Generic {
    pub fn new(min: f32, max: f32) -> Self {
        let range = max - min;
        let range_inv = if range == 0.0 { 0.0 } else { 1.0 / range };

        Self {
            min,
            range,
            range_inv,
        }
    }

    #[inline(always)]
    pub fn normalize(&self, value: f32) -> f32 {
        (value - self.min) * self.range_inv
    }

    #[inline(always)]
    pub fn denormalize(&self, normalized: f32) -> f32 {
        (normalized * self.range) + self.min
    }
}

#[derive(Debug)]
pub struct DB {
    min: f32,
    range: f32,
    range_inv: f32,
}

impl DB {
    pub fn new(min: f32, max: f32) -> Self {
        let range = max - min;
        let range_inv = if range == 0.0 { 0.0 } else { 1.0 / range };

        Self {
            min,
            range,
            range_inv,
        }
    }

    #[inline(always)]
    pub fn normalize(&self, value: f32) -> f32 {
        (coeff_to_db(value) - self.min) * self.range_inv
    }

    #[inline(always)]
    pub fn denormalize(&self, normalized: f32) -> f32 {
        db_to_coeff((normalized * self.range) + self.min)
    }
}

#[derive(Debug)]
pub struct DBClamped {
    min: f32,
    range: f32,
    range_inv: f32,
    clamp_db: f32,
    clamp_coeff: f32,
}

impl DBClamped {
    pub fn new(min: f32, max: f32, neg_infinity_clamp_db: f32) -> Self {
        let range = max - min;
        let range_inv = if range == 0.0 { 0.0 } else { 1.0 / range };

        Self {
            min,
            range,
            range_inv,
            clamp_db: neg_infinity_clamp_db,
            clamp_coeff: db_to_coeff(neg_infinity_clamp_db),
        }
    }

    #[inline(always)]
    pub fn normalize(&self, value: f32) -> f32 {
        let db = if value <= self.clamp_coeff {
            self.clamp_db
        } else {
            coeff_to_db(value)
        };

        (db - self.min) * self.range_inv
    }

    #[inline(always)]
    pub fn denormalize(&self, normalized: f32) -> f32 {
        let value = (normalized * self.range) + self.min;

        if value <= self.clamp_db {
            0.0
        } else {
            db_to_coeff(value)
        }
    }
}

#[inline(always)]
fn db_to_coeff(db: f32) -> f32 {
    10.0f32.powf(0.05 * db)
}

#[inline(always)]
fn coeff_to_db(coeff: f32) -> f32 {
    20.0 * coeff.log(10.0)
}
