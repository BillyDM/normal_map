use std::fmt::Debug;

use super::Unit;

#[derive(Debug)]
pub enum Base {
    Generic(Generic),
    DB(DB),
    DBClamped(DBClamped),
}

impl Base {
    pub fn new(min: f64, max: f64, unit: Unit) -> Self {
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
    min: f64,
    range: f64,
    range_inv: f64,
}

impl Generic {
    pub fn new(min: f64, max: f64) -> Self {
        let range = max - min;
        let range_inv = if range == 0.0 { 0.0 } else { 1.0 / range };

        Self {
            min,
            range,
            range_inv,
        }
    }

    #[inline(always)]
    pub fn normalize(&self, value: f64) -> f64 {
        (value - self.min) * self.range_inv
    }

    #[inline(always)]
    pub fn denormalize(&self, normalized: f64) -> f64 {
        (normalized * self.range) + self.min
    }
}

#[derive(Debug)]
pub struct DB {
    min: f64,
    range: f64,
    range_inv: f64,
}

impl DB {
    pub fn new(min: f64, max: f64) -> Self {
        let range = max - min;
        let range_inv = if range == 0.0 { 0.0 } else { 1.0 / range };

        Self {
            min,
            range,
            range_inv,
        }
    }

    #[inline(always)]
    pub fn normalize(&self, value: f64) -> f64 {
        (coeff_to_db(value) - self.min) * self.range_inv
    }

    #[inline(always)]
    pub fn denormalize(&self, normalized: f64) -> f64 {
        db_to_coeff((normalized * self.range) + self.min)
    }
}

#[derive(Debug)]
pub struct DBClamped {
    min: f64,
    range: f64,
    range_inv: f64,
    clamp_db: f64,
    clamp_coeff: f64,
}

impl DBClamped {
    pub fn new(min: f64, max: f64, neg_infinity_clamp_db: f64) -> Self {
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
    pub fn normalize(&self, value: f64) -> f64 {
        let db = if value <= self.clamp_coeff {
            self.clamp_db
        } else {
            coeff_to_db(value)
        };

        (db - self.min) * self.range_inv
    }

    #[inline(always)]
    pub fn denormalize(&self, normalized: f64) -> f64 {
        let value = (normalized * self.range) + self.min;

        if value <= self.clamp_db {
            0.0
        } else {
            db_to_coeff(value)
        }
    }
}

#[inline(always)]
fn db_to_coeff(db: f64) -> f64 {
    10.0f64.powf(0.05 * db)
}

#[inline(always)]
fn coeff_to_db(coeff: f64) -> f64 {
    20.0 * coeff.log(10.0)
}
