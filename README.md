# Normalized Range Mapper
![Test](https://github.com/BillyDM/normal_map/workflows/Test/badge.svg)
[![Documentation](https://docs.rs/normal_map/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/normal_map.svg)](https://crates.io/crates/normal_map)
[![License](https://img.shields.io/crates/l/normal_map.svg)](https://github.com/BillyDM/normal_map/blob/master/LICENSE)

A Rust helper than maps ranges of values to the normalized range [0.0, 1.0] using various gradients, useful for DSP applications.

## Example

```rust
// Import normal map that uses internal f32 values,
// f64 is available as well.
use normal_map::f32::*;

// Linear mapper
let lin_map = LinearMap::new(-50.0, 50.0, Unit::Generic);

assert!((lin_map.normalize(25.0) - 0.75).abs() <= 0.0001);
assert!((lin_map.denormalize(0.25) - (-25.0)).abs() <= 0.0001);

// Generic type for all mappers
let normal_map = NormalMap::discrete::<isize>(-5, 5);

assert!((normal_map.normalize(3 as f32) - 0.8).abs() <= 0.0001);
```

[documentation]: https://docs.rs/normal_map/
