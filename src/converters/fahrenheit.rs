pub fn to_celsius(value: u32) -> f32{
    (((value as f32) - 32.0) * 5.0) / 9.0
}

pub fn to_kelvins(value: u32) -> f32 {
    to_celsius(value) + 273.15
}
