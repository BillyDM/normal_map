//! Normal mapping using `f64` as the internal unit.

mod discrete;
mod linear;
mod linear_base;
mod log2;
mod power;

pub use discrete::DiscreteMap;
pub use linear::LinearMap;
pub use log2::Log2Map;
pub use power::PowerMap;

/// The type of mapping to use
#[derive(Debug)]
pub enum Mapper {
    /// Linear mapping
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// will be linearly mapped, not the raw amplitude.
    Lin(LinearMap),
    /// Exponential mapping where the normalized value is raised to the
    /// supplied exponent.
    ///
    /// Please note if you use `Unit::Decibels`, then the decibels
    /// will be linearly mapped, not the raw amplitude.
    Pow(PowerMap),
    /// Logarithmic mapping using `log2`. This is useful for frequency (Hz) values.
    Log2(Log2Map),
    /// Discrete `isize` integer mapping
    ///
    /// A supplied enum may be used as well as long
    /// as it implements `From<isize> + Into<isize> + Copy + Clone`.
    Discrete(DiscreteMap),
}

#[derive(Debug)]
/// The unit to use
pub enum Unit {
    /// Generic units
    Generic,
    /// Decibel units.
    ///
    /// Please note that values in and out of the mapper are raw amplitudes, not decibels.
    ///
    /// * `neg_infinity_clamp`: The point at which any values less than
    /// or equal to this value (e.g. `Some(-90.0)` for -90 dB) are clampled to negative
    /// infinity (silence). Set this to `None` for no clamping.
    Decibels { neg_infinity_clamp: Option<f64> },
}

/// A mapper than maps a range of values to and from the normalized
/// `f64` range `[0.0, 1.0]`.
#[derive(Debug)]
pub struct NormalMap {
    /// The current mapper in use
    pub mapper: Mapper,
}

impl NormalMap {
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
            mapper: Mapper::Lin(LinearMap::new(min, max, unit)),
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
            mapper: Mapper::Pow(PowerMap::new(min, max, exponent, unit)),
        }
    }

    /// Create a new `NormalMap` with a logarithmic mapping using `log2`.
    /// This is useful for frequency (Hz) values.
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
            mapper: Mapper::Log2(Log2Map::new(min, max)),
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
            mapper: Mapper::Discrete(DiscreteMap::new(min, max)),
        }
    }

    /// Map an `f64` value to the normalized range `[0.0, 1.0]`.
    pub fn normalize(&self, value: f64) -> f64 {
        match &self.mapper {
            Mapper::Lin(mapper) => mapper.normalize(value),
            Mapper::Pow(mapper) => mapper.normalize(value),
            Mapper::Log2(mapper) => mapper.normalize(value),
            Mapper::Discrete(mapper) => mapper.normalize_float(value),
        }
    }

    /// Map an array of `f64` values to the normalized range `[0.0, 1.0]`.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn normalize_array(&self, in_values: &[f64], out_normalized: &mut [f64]) {
        match &self.mapper {
            Mapper::Lin(mapper) => mapper.normalize_array(in_values, out_normalized),
            Mapper::Pow(mapper) => mapper.normalize_array(in_values, out_normalized),
            Mapper::Log2(mapper) => mapper.normalize_array(in_values, out_normalized),
            Mapper::Discrete(mapper) => mapper.normalize_array_float(in_values, out_normalized),
        }
    }

    /// Un-map a normalized value to the corresponding `f64` value.
    pub fn denormalize(&self, normalized: f64) -> f64 {
        match &self.mapper {
            Mapper::Lin(mapper) => mapper.denormalize(normalized),
            Mapper::Pow(mapper) => mapper.denormalize(normalized),
            Mapper::Log2(mapper) => mapper.denormalize(normalized),
            Mapper::Discrete(mapper) => mapper.denormalize_float(normalized),
        }
    }

    /// Un-map an array of normalized values to the corresponding `f64` value.
    ///
    /// Values will be processed up to the length of the shortest array.
    pub fn denormalize_array(&self, in_normalized: &[f64], out_values: &mut [f64]) {
        match &self.mapper {
            Mapper::Lin(mapper) => mapper.denormalize_array(in_normalized, out_values),
            Mapper::Pow(mapper) => mapper.denormalize_array(in_normalized, out_values),
            Mapper::Log2(mapper) => mapper.denormalize_array(in_normalized, out_values),
            Mapper::Discrete(mapper) => mapper.denormalize_array_float(in_normalized, out_values),
        }
    }
}
