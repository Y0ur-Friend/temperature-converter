pub fn to_celsius(value: u32) -> f32 {
    (value as f32) - 273.15
}

pub fn to_fahrenheit(value: u32) -> f32 {
    to_celsius(value) * (9.0 / 5.0) + 32.0
}
