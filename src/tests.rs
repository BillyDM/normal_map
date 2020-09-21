use crate::*;

#[test]
fn linear_map_f32() {
    let normal_map = f32::NormalMap::linear(-50.0, 50.0, f32::Unit::Generic);

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
    let normal_map = f64::NormalMap::linear(-50.0, 50.0, f64::Unit::Generic);

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
    let normal_map = f32::NormalMap::power(-50.0, 50.0, 0.5, f32::Unit::Generic);

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
    let normal_map = f64::NormalMap::power(-50.0, 50.0, 0.5, f64::Unit::Generic);

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
    let normal_map = f32::NormalMap::log2(20.0, 20480.0);

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
    let normal_map = f64::NormalMap::log2(20.0, 20480.0);

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
    let normal_map = f32::NormalMap::discrete::<isize>(-5, 5);

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
    let normal_map = f64::NormalMap::discrete::<isize>(-5, 5);

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
    assert!(
        (a - b).abs() <= 0.0001,
        "Values are not approximate: a = {}, b = {}",
        a,
        b
    );
}

fn assert_approximate_f64(a: f64, b: f64) {
    assert!(
        (a - b).abs() < 0.000000000001,
        "Values are not approximate: a = {}, b = {}",
        a,
        b
    );
}
