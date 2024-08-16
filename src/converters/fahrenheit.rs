pub fn to_celsius(value: f32) -> f32 {
    ((value - 32.0) * 5.0) / 9.0
}

pub fn to_kelvins(value: f32) -> f32 {
    to_celsius(value) + 273.15
}
