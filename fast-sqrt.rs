fn fast_sqrt(number: f32) -> f32 {
    // If the inverse square root is 1/sqrt(x), then sqrt(x) = x * (1/sqrt(x))
    number * fast_inverse_sqrt(number)
}

fn fast_inverse_sqrt(number: f32) -> f32 {
    let xhalf = 0.5f32 * number;
    let mut i: i32 = unsafe { std::mem::transmute(number) }; // Reinterpret float as integer
    i = 0x5f3759df - (i >> 1); // Magic constant and bit shift // I believe this is the "evil floating point hack" id speaks of in their source
    let mut res: f32 = unsafe { std::mem::transmute(i) }; // Reinterpret integer as float
    res = res * (1.5f32 - xhalf * res * res); // Newton's method iteration
    res
}
