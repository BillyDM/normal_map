//! A struct that maps a range of values to the normalized range `[0.0, 1.0]` using various
//! gradients, useful for DSP applications.
//!
//! (prerelease)


use std::fmt::Debug;

mod linear_base;
mod discrete;
mod log2;
mod linear;
mod power;
mod util;

pub use discrete::{DiscreteMapF32, DiscreteMapF64};
pub use log2::{Log2MapF32, Log2MapF64};
pub use linear::{LinearMapF32, LinearMapF64};
pub use power::{PowerMapF32, PowerMapF64};
pub use util::*;

/// The type of mapping to use
pub enum MapperF32 {
    /// Linear mapping
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// will be linearly mapped, not the raw amplitude.
    Lin(LinearMapF32),
    /// Exponential mapping where the normalized value is raised to the
    /// supplied exponent.
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// will be linearly mapped, not the raw amplitude.
    Pow(PowerMapF32),
    /// Logarithmic mapping using `log2`
    Log2(Log2MapF32),
    /// Discrete `isize` integer mapping
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    Discrete(DiscreteMapF32),
}

/// The type of mapping to use
pub enum MapperF64 {
    /// Linear mapping
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// will be linearly mapped, not the raw amplitude.
    Lin(LinearMapF64),
    /// Exponential mapping where the normalized value is raised to the
    /// supplied exponent.
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// will be linearly mapped, not the raw amplitude.
    Pow(PowerMapF64),
    /// Logarithmic mapping using `log2`
    Log2(Log2MapF64),
    /// Discrete `isize` integer mapping
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    Discrete(DiscreteMapF64),
}

#[derive(Debug)]
/// The unit to use
pub enum Unit {
    /// Generic units
    Generic,
    /// Decibel units
    Decibels,
}

/// A mapper than maps a range of values to and from the normalized
/// `f32` range `[0.0, 1.0]`.
pub struct NormalMapF32 {
    /// The current mapper in use
    pub mapper: MapperF32,
}

impl NormalMapF32 {
    /// Create a new `NormalMap` with linear mapping.
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// are what will be linearly mapped, not the raw amplitude.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range
    /// * max - the maximum of the range
    /// * unit - the type of unit
    pub fn linear(min: f32, max: f32, unit: Unit) -> Self {
        Self {
            mapper: MapperF32::Lin(LinearMapF32::new(min, max, unit)),
        }
    }

    /// Create a new `NormalMap` with an exponential mapping where the
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
    pub fn power(min: f32, max: f32, exponent: f32, unit: Unit) -> Self {
        Self {
            mapper: MapperF32::Pow(PowerMapF32::new(min, max, exponent, unit)),
        }
    }

    /// Create a new `NormalMap` with a logarithmic mapping using `log2`.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range, must be > 0.0
    /// * max - the maximum of the range, must be > 0.0
    ///
    /// # Panics
    ///
    /// * Panics when either `min` or `max` <= 0.0.
    pub fn log2(min: f32, max: f32) -> Self {
        Self {
            mapper: MapperF32::Log2(Log2MapF32::new(min, max)),
        }
    }

    /// Create a new `NormalMap` with a discrete `isize` integer range.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range
    /// * max - the maximum of the range
    pub fn discrete<T>(min: T, max: T) -> Self
    where
        T: From<isize> + Into<isize> + Copy + Clone
    {
        Self {
            mapper: MapperF32::Discrete(DiscreteMapF32::new(min, max)),
        }
    }

    /// Map an `f32` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f32) -> f32 {
        match &self.mapper {
            MapperF32::Lin(mapper) => mapper.normalize(value),
            MapperF32::Pow(mapper) => mapper.normalize(value),
            MapperF32::Log2(mapper) => mapper.normalize(value),
            MapperF32::Discrete(mapper) => mapper.normalize_f32(value),
        }
    }

    /// Map an array of `f32` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f32], out_normalized: &mut [f32]) {
        match &self.mapper {
            MapperF32::Lin(mapper) => mapper.normalize_array(in_values, out_normalized),
            MapperF32::Pow(mapper) => mapper.normalize_array(in_values, out_normalized),
            MapperF32::Log2(mapper) => mapper.normalize_array(in_values, out_normalized),
            MapperF32::Discrete(mapper) => mapper.normalize_array_f32(in_values, out_normalized),
        }
    }

    /// Un-map a normalized value to the corresponding `f32` value.
    pub fn denormalize(&self, normalized: f32) -> f32 {
        match &self.mapper {
            MapperF32::Lin(mapper) => mapper.denormalize(normalized),
            MapperF32::Pow(mapper) => mapper.denormalize(normalized),
            MapperF32::Log2(mapper) => mapper.denormalize(normalized),
            MapperF32::Discrete(mapper) => mapper.denormalize_f32(normalized),
        }
    }

    /// Un-map an array of normalized values to the corresponding `f32` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f32], out_values: &mut [f32]) {
        match &self.mapper {
            MapperF32::Lin(mapper) => mapper.denormalize_array(in_normalized, out_values),
            MapperF32::Pow(mapper) => mapper.denormalize_array(in_normalized, out_values),
            MapperF32::Log2(mapper) => mapper.denormalize_array(in_normalized, out_values),
            MapperF32::Discrete(mapper) => mapper.denormalize_array_f32(in_normalized, out_values),
        }
    }
}


/// A mapper than maps a range of values to and from the normalized
/// `f64` range `[0.0, 1.0]`.
pub struct NormalMapF64 {
    /// The current mapper in use
    pub mapper: MapperF64,
}

impl NormalMapF64 {
    /// Create a new `NormalMap` with linear mapping.
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// are what will be linearly mapped, not the raw amplitude.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range
    /// * max - the maximum of the range
    /// * unit - the type of unit
    pub fn linear(min: f64, max: f64, unit: Unit) -> Self {
        Self {
            mapper: MapperF64::Lin(LinearMapF64::new(min, max, unit)),
        }
    }

    /// Create a new `NormalMap` with an exponential mapping where the
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
    pub fn power(min: f64, max: f64, exponent: f64, unit: Unit) -> Self {
        Self {
            mapper: MapperF64::Pow(PowerMapF64::new(min, max, exponent, unit)),
        }
    }

    /// Create a new `NormalMap` with a logarithmic mapping using `log2`.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range, must be > 0.0
    /// * max - the maximum of the range, must be > 0.0
    ///
    /// # Panics
    ///
    /// * Panics when either `min` or `max` <= 0.0.
    pub fn log2(min: f64, max: f64) -> Self {
        Self {
            mapper: MapperF64::Log2(Log2MapF64::new(min, max)),
        }
    }

    /// Create a new `NormalMap` with a discrete `isize` integer range.
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    ///
    /// # Arguments
    ///
    /// * min - the minimum of the range
    /// * max - the maximum of the range
    pub fn discrete<T>(min: T, max: T) -> Self
    where
        T: From<isize> + Into<isize> + Copy + Clone
    {
        Self {
            mapper: MapperF64::Discrete(DiscreteMapF64::new(min, max)),
        }
    }

    /// Map an `f64` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f64) -> f64 {
        match &self.mapper {
            MapperF64::Lin(mapper) => mapper.normalize(value),
            MapperF64::Pow(mapper) => mapper.normalize(value),
            MapperF64::Log2(mapper) => mapper.normalize(value),
            MapperF64::Discrete(mapper) => mapper.normalize_f64(value),
        }
    }

    /// Map an array of `f64` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f64], out_normalized: &mut [f64]) {
        match &self.mapper {
            MapperF64::Lin(mapper) => mapper.normalize_array(in_values, out_normalized),
            MapperF64::Pow(mapper) => mapper.normalize_array(in_values, out_normalized),
            MapperF64::Log2(mapper) => mapper.normalize_array(in_values, out_normalized),
            MapperF64::Discrete(mapper) => mapper.normalize_array_f64(in_values, out_normalized),
        }
    }

    /// Un-map a normalized value to the corresponding `f64` value.
    pub fn denormalize(&self, normalized: f64) -> f64 {
        match &self.mapper {
            MapperF64::Lin(mapper) => mapper.denormalize(normalized),
            MapperF64::Pow(mapper) => mapper.denormalize(normalized),
            MapperF64::Log2(mapper) => mapper.denormalize(normalized),
            MapperF64::Discrete(mapper) => mapper.denormalize_f64(normalized),
        }
    }

    /// Un-map an array of normalized values to the corresponding `f64` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f64], out_values: &mut [f64]) {
        match &self.mapper {
            MapperF64::Lin(mapper) => mapper.denormalize_array(in_normalized, out_values),
            MapperF64::Pow(mapper) => mapper.denormalize_array(in_normalized, out_values),
            MapperF64::Log2(mapper) => mapper.denormalize_array(in_normalized, out_values),
            MapperF64::Discrete(mapper) => mapper.denormalize_array_f64(in_normalized, out_values),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_map_f32() {
        let normal_map = NormalMapF32::linear(-50.0, 50.0, Unit::Generic);

        assert_approximate_f32(0.0, normal_map.normalize(-50.0));
        assert_approximate_f32(0.0, normal_map.normalize(-52.0));
        assert_approximate_f32(1.0, normal_map.normalize(50.0));
        assert_approximate_f32(1.0, normal_map.normalize(52.0));

        assert_approximate_f32(-50.0, normal_map.denormalize(0.0));
        assert_approximate_f32(50.0, normal_map.denormalize(1.0));


        assert_approximate_f32(0.5, normal_map.normalize(0.0));
        assert_approximate_f32(0.25, normal_map.normalize(-25.0));
        assert_approximate_f32(0.75, normal_map.normalize(25.0));

        assert_approximate_f32(0.0, normal_map.denormalize(0.5));
        assert_approximate_f32(-25.0, normal_map.denormalize(0.25));
        assert_approximate_f32(25.0, normal_map.denormalize(0.75));
    }

    #[test]
    fn linear_map_f64() {
        let normal_map = NormalMapF64::linear(-50.0, 50.0, Unit::Generic);

        assert_approximate_f64(0.0, normal_map.normalize(-50.0));
        assert_approximate_f64(0.0, normal_map.normalize(-52.0));
        assert_approximate_f64(1.0, normal_map.normalize(50.0));
        assert_approximate_f64(1.0, normal_map.normalize(52.0));

        assert_approximate_f64(-50.0, normal_map.denormalize(0.0));
        assert_approximate_f64(50.0, normal_map.denormalize(1.0));


        assert_approximate_f64(0.5, normal_map.normalize(0.0));
        assert_approximate_f64(0.25, normal_map.normalize(-25.0));
        assert_approximate_f64(0.75, normal_map.normalize(25.0));

        assert_approximate_f64(0.0, normal_map.denormalize(0.5));
        assert_approximate_f64(-25.0, normal_map.denormalize(0.25));
        assert_approximate_f64(25.0, normal_map.denormalize(0.75));
    }

    #[test]
    fn power_map_f32() {
        let normal_map = NormalMapF32::power(-50.0, 50.0, 0.5, Unit::Generic);

        assert_approximate_f32(0.0, normal_map.normalize(-50.0));
        assert_approximate_f32(0.0, normal_map.normalize(-52.0));
        assert_approximate_f32(1.0, normal_map.normalize(50.0));
        assert_approximate_f32(1.0, normal_map.normalize(52.0));

        assert_approximate_f32(-50.0, normal_map.denormalize(0.0));
        assert_approximate_f32(50.0, normal_map.denormalize(1.0));


        assert_approximate_f32(0.25, normal_map.normalize(0.0));
        assert_approximate_f32(0.0625, normal_map.normalize(-25.0));
        assert_approximate_f32(0.5625, normal_map.normalize(25.0));

        assert_approximate_f32(0.0, normal_map.denormalize(0.25));
        assert_approximate_f32(-25.0, normal_map.denormalize(0.0625));
        assert_approximate_f32(25.0, normal_map.denormalize(0.5625));
    }

    #[test]
    fn power_map_f64() {
        let normal_map = NormalMapF64::power(-50.0, 50.0, 0.5, Unit::Generic);

        assert_approximate_f64(0.0, normal_map.normalize(-50.0));
        assert_approximate_f64(0.0, normal_map.normalize(-52.0));
        assert_approximate_f64(1.0, normal_map.normalize(50.0));
        assert_approximate_f64(1.0, normal_map.normalize(52.0));

        assert_approximate_f64(-50.0, normal_map.denormalize(0.0));
        assert_approximate_f64(50.0, normal_map.denormalize(1.0));


        assert_approximate_f64(0.25, normal_map.normalize(0.0));
        assert_approximate_f64(0.0625, normal_map.normalize(-25.0));
        assert_approximate_f64(0.5625, normal_map.normalize(25.0));

        assert_approximate_f64(0.0, normal_map.denormalize(0.25));
        assert_approximate_f64(-25.0, normal_map.denormalize(0.0625));
        assert_approximate_f64(25.0, normal_map.denormalize(0.5625));
    }

    #[test]
    fn log_map_f32() {
        let normal_map = NormalMapF32::log2(20.0, 20480.0);

        assert_approximate_f32(0.0, normal_map.normalize(20.0));
        assert_approximate_f32(0.0, normal_map.normalize(18.0));
        assert_approximate_f32(1.0, normal_map.normalize(20480.0));
        assert_approximate_f32(1.0, normal_map.normalize(20500.0));

        assert_approximate_f32(20.0, normal_map.denormalize(0.0));
        assert_approximate_f32(20480.0, normal_map.denormalize(1.0));


        assert_approximate_f32(0.1, normal_map.normalize(40.0));
        assert_approximate_f32(0.5643856, normal_map.normalize(1000.0));
        assert_approximate_f32(0.89657843, normal_map.normalize(10000.0));

        assert_approximate_f32(40.0, normal_map.denormalize(0.1));
        assert_approximate_f32(640.0, normal_map.denormalize(0.5));
        assert_approximate_f32(3620.3865, normal_map.denormalize(0.75));
    }

    #[test]
    fn log_map_f64() {
        let normal_map = NormalMapF64::log2(20.0, 20480.0);

        assert_approximate_f64(0.0, normal_map.normalize(20.0));
        assert_approximate_f64(0.0, normal_map.normalize(18.0));
        assert_approximate_f64(1.0, normal_map.normalize(20480.0));
        assert_approximate_f64(1.0, normal_map.normalize(20500.0));

        assert_approximate_f64(20.0, normal_map.denormalize(0.0));
        assert_approximate_f64(20480.0, normal_map.denormalize(1.0));


        assert_approximate_f64(0.1, normal_map.normalize(40.0));
        assert_approximate_f64(0.5643856189774724, normal_map.normalize(1000.0));
        assert_approximate_f64(0.8965784284662086, normal_map.normalize(10000.0));

        assert_approximate_f64(40.0, normal_map.denormalize(0.1));
        assert_approximate_f64(640.0, normal_map.denormalize(0.5));
        assert_approximate_f64(3620.3867196751216, normal_map.denormalize(0.75));
    }

    #[test]
    fn discrete_map_f32() {
        let normal_map = NormalMapF32::discrete::<isize>(-5, 5);

        assert_approximate_f32(0.0, normal_map.normalize(-5.0));
        assert_approximate_f32(0.0, normal_map.normalize(-6.0));
        assert_approximate_f32(1.0, normal_map.normalize(5.0));
        assert_approximate_f32(1.0, normal_map.normalize(6.0));

        assert_approximate_f32(-5.0, normal_map.denormalize(0.0));
        assert_approximate_f32(5.0, normal_map.denormalize(1.0));


        assert_approximate_f32(0.0, normal_map.normalize(-4.9));
        assert_approximate_f32(1.0, normal_map.normalize(4.9));
        assert_approximate_f32(0.2, normal_map.normalize(-3.0));
        assert_approximate_f32(0.8, normal_map.normalize(3.0));

        assert_approximate_f32(-3.0, normal_map.denormalize(0.2));
        assert_approximate_f32(3.0, normal_map.denormalize(0.8));
    }

    #[test]
    fn discrete_map_f64() {
        let normal_map = NormalMapF64::discrete::<isize>(-5, 5);

        assert_approximate_f64(0.0, normal_map.normalize(-5.0));
        assert_approximate_f64(0.0, normal_map.normalize(-6.0));
        assert_approximate_f64(1.0, normal_map.normalize(5.0));
        assert_approximate_f64(1.0, normal_map.normalize(6.0));

        assert_approximate_f64(-5.0, normal_map.denormalize(0.0));
        assert_approximate_f64(5.0, normal_map.denormalize(1.0));


        assert_approximate_f64(0.0, normal_map.normalize(-4.9));
        assert_approximate_f64(1.0, normal_map.normalize(4.9));
        assert_approximate_f64(0.2, normal_map.normalize(-3.0));
        assert_approximate_f64(0.8, normal_map.normalize(3.0));

        assert_approximate_f64(-3.0, normal_map.denormalize(0.2));
        assert_approximate_f64(3.0, normal_map.denormalize(0.8));
    }

    fn assert_approximate_f32(a: f32, b: f32) {
        assert!((a - b).abs() <= 0.0001, "Values are not approximate: a = {}, b = {}", a, b);
    }

    fn assert_approximate_f64(a: f64, b: f64) {
        assert!((a - b).abs() < 0.000000000001, "Values are not approximate: a = {}, b = {}", a, b);
    }
}