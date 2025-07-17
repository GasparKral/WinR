pub fn lerp<T>(start: T, end: T, t: f32) -> T
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>,
{
    start + (end - start) * t
}

pub fn lerp_clamp<T>(start: T, end: T, t: f32) -> T
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>,
{
    lerp(start, end, t.clamp(0.0, 1.0))
}
pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd + Copy,
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
