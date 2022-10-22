use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul};
use webgl_rc::{DataType, IntoUniform, TypeMark, UniformValue, Writable};

use super::mix::Mix;

/// Premultiplied linear RGBA color
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

fn linear(v: f32) -> f32 {
    v.powf(2.2)
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: r * a,
            g: g * a,
            b: b * a,
            a,
        }
    }
    pub fn srgb(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: linear(r) * a,
            g: linear(g) * a,
            b: linear(b) * a,
            a,
        }
    }
    pub fn opaque(&self) -> Color {
        if self.a > 0.0001 {
            let k = 1.0 / self.a;
            Color::new(self.r * k, self.g * k, self.b * k, 1.0)
        } else {
            Color::new(0.0, 0.0, 0.0, 1.0)
        }
    }
}

impl Zero for Color {
    fn zero() -> Self {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.r.is_zero() && self.g.is_zero() && self.b.is_zero() && self.a.is_zero()
    }
}

impl One for Color {
    fn one() -> Self {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scale: f32) -> Color {
        Color {
            r: self.r * scale,
            g: self.g * scale,
            b: self.b * scale,
            a: self.a * scale,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color {
            r: color.r * self,
            g: color.g * self,
            b: color.b * self,
            a: color.a * self,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<Color> for f32 {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        (1.0 / self) * rhs
    }
}

impl Mix for Color {
    type Fraction = f32;

    fn mix(self, other: Self, t: f32) -> Self {
        Color {
            r: self.r.mix(other.r, t),
            g: self.g.mix(other.g, t),
            b: self.b.mix(other.b, t),
            a: self.a.mix(other.a, t),
        }
    }
}

impl TypeMark for Color {
    fn data_type() -> DataType {
        DataType::Vec4
    }
}

impl Writable for Color {
    fn write(&self, output: &mut Vec<f32>) {
        output.push(self.r);
        output.push(self.g);
        output.push(self.b);
        output.push(self.a);
    }
    fn stride() -> usize {
        4
    }
}

impl IntoUniform for Color {
    fn into_uniform(&self) -> UniformValue {
        UniformValue::Vec4([self.r, self.g, self.b, self.a])
    }
}

impl From<[f32; 4]> for Color {
    fn from(values: [f32; 4]) -> Self {
        Color::new(values[0], values[1], values[2], values[3])
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(values: (f32, f32, f32, f32)) -> Self {
        Color::new(values.0, values.1, values.2, values.3)
    }
}
