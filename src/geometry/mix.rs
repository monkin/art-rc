use num_traits::Float;

pub trait Mix {
    type Fraction: Float + Copy + Clone;

    fn mix(self, other: Self, t: Self::Fraction) -> Self;
}

impl Mix for f32 {
    type Fraction = f32;

    fn mix(self, other: Self, t: Self::Fraction) -> Self {
        self + (other - self) * t
    }
}

impl Mix for f64 {
    type Fraction = f64;

    fn mix(self, other: Self, t: Self::Fraction) -> Self {
        self + (other - self) * t
    }
}
