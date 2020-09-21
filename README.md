# Normalized Range Mapper
![Test](https://github.com/BillyDM/normal_map/workflows/Test/badge.svg)
[![Documentation](https://docs.rs/normal_map/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/normal_map.svg)](https://crates.io/crates/normal_map)
[![License](https://img.shields.io/crates/l/normal_map.svg)](https://github.com/BillyDM/normal_map/blob/master/LICENSE)

A Rust helper than maps ranges of values to and from the normalized range [0.0, 1.0] using various gradients, useful for DSP applications.

_(currently in beta)_

## Gradient Types
* `LinearMap` - Linear mapping. This can use either generic or decibel units.
* `PowerMap` - Exponential mapping where the normalized value is raised to the supplied exponent. This can use either generic or decibel units.
* `Log2Map` - Logarithmic mapping using `log2`. This is useful for frequency (Hz) values.
* `Discrete` - Discrete `isize` integer mapping. A supplied enum may also be used as well as long as it implements `From<isize> + Into<isize> + Copy + Clone`. This mapper has methods for converting to and from either float values or `isize`/`enum` values.

## Example

```rust
// Import normal map that uses internal f32 values.
// (f64 is available as well)
use normal_map::f32::*;

// Linear mapper.
let lin_map = LinearMap::new(-50.0, 50.0, Unit::Generic);

assert!((lin_map.normalize(25.0) - 0.75).abs() <= 0.0001);
assert!((lin_map.denormalize(0.25) - (-25.0)).abs() <= 0.0001);

// Efficiently map an array/slice of values.
let in_normals = [0.0f32, 1.0, 0.25, -0.25];
let mut out_values = [0.0f32; 4];
lin_map.denormalize_array(&in_normals, &mut out_values);

// Generic type for any mapper.
let normal_map = NormalMap::discrete::<isize>(-5, 5);

assert!((normal_map.normalize(3 as f32) - 0.8).abs() <= 0.0001);
```

[documentation]: https://docs.rs/normal_map/
