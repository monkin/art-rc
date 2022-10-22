use num_traits::Float;

pub trait Distance {
    type Output: Float;
    fn distance(self, other: Self) -> Self::Output;
}

impl Distance for f32 {
    type Output = f32;
    fn distance(self, v: Self) -> f32 {
        (v - self).abs()
    }
}

impl Distance for f64 {
    type Output = f64;
    fn distance(self, v: Self) -> f64 {
        (v - self).abs()
    }
}
