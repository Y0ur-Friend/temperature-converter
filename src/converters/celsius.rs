pub fn to_fahrenheit(value: u32) -> f32 {
    (value as f32) * (9.0 / 5.0) + 32.0
}

pub fn to_kelvins(value: u32) -> f32 {
    (value as f32) + 273.15
}
