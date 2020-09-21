//! A helper that maps a range of values to and from the normalized
//! range `[0.0, 1.0]` using various gradients, useful for DSP applications.
//!
//! (currently in beta)
//!
//! ## Installation
//! Add `normal_map` as a dependency in your `Cargo.toml`:
//! ```text
//! normal_map = 0.2
//! ```
//!
//! ## Example
//! ```
//! // Import normal mappers that use internal f32 values.
//! // (f64 is available as well)
//! use normal_map::f32::*;
//!
//! // Linear mapper
//! let lin_map = LinearMap::new(-50.0, 50.0, Unit::Generic);
//!
//! assert!((lin_map.normalize(25.0) - 0.75).abs() <= 0.0001);
//! assert!((lin_map.denormalize(0.25) - (-25.0)).abs() <= 0.0001);
//!
//! // Efficiently map an array/slice of values.
//! let in_normals = [0.0f32, 1.0, 0.25, -0.25];
//! let mut out_values = [0.0f32; 4];
//!
//! lin_map.denormalize_array(&in_normals, &mut out_values);
//!
//! // Generic type for any mapper
//! let normal_map = NormalMap::discrete::<isize>(-5, 5);
//!
//! assert!((normal_map.normalize(3 as f32) - 0.8).abs() <= 0.0001);
//! ```

#[cfg(test)]
mod tests;

pub mod f32;
pub mod f64;
