//! A struct that maps a range of values to the normalized range `[0.0, 1.0]` using various
//! gradients, useful for DSP applications.
//!
//! (prerelease)

#[cfg(test)]
mod tests;

use std::fmt::Debug;

mod discrete;
mod linear;
mod linear_base;
mod log2;
mod power;

pub use discrete::{DiscreteMapF32, DiscreteMapF64};
pub use linear::{LinearMapF32, LinearMapF64};
pub use log2::{Log2MapF32, Log2MapF64};
pub use power::{PowerMapF32, PowerMapF64};

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
        T: From<isize> + Into<isize> + Copy + Clone,
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
        T: From<isize> + Into<isize> + Copy + Clone,
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
