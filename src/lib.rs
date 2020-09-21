//! A struct that maps a range of values to the normalized range `[0.0, 1.0]` using various
//! gradients, useful for DSP applications.
//!
//! (prerelease)
//!
//! ## Example
//! ```
//! // Import normal map that uses internal f32 values,
//! // f64 is available as well.
//! use normal_map::f32::*;
//! 
//! // Linear mapper
//! let lin_map = LinearMap::new(-50.0, 50.0, Unit::Generic);
//! 
//! assert!((lin_map.normalize(25.0) - 0.75).abs() <= 0.0001);
//! assert!((lin_map.denormalize(0.25) - (-25.0)).abs() <= 0.0001);
//! 
//! // Generic type for all mappers
//! let normal_map = NormalMap::discrete::<isize>(-5, 5);
//! 
//! assert!((normal_map.normalize(3 as f32) - 0.8).abs() <= 0.0001);
//! ```

#[cfg(test)]
mod tests;

pub mod f32;
pub mod f64;
