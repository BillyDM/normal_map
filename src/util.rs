#[inline]
pub fn db_to_coeff_f32(db: f32) -> f32 {
    if db < -90.0 {
        0.0
    } else {
        10.0f32.powf(0.05 * db)
    }
}

#[inline]
pub fn coeff_to_db_f32(coeff: f32) -> f32 {
    if coeff <= 0.00003162277 {
        -90.0
    } else {
        20.0 * coeff.log(10.0)
    }
}

#[inline]
pub fn db_to_coeff_f64(db: f64) -> f64 {
    if db < -90.0 {
        0.0
    } else {
        10.0f64.powf(0.05 * db)
    }
}

#[inline]
pub fn coeff_to_db_f64(coeff: f64) -> f64 {
    if coeff <= 0.00003162277 {
        -90.0
    } else {
        20.0 * coeff.log(10.0)
    }
}
