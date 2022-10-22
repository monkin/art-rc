use super::distance::Distance;
use super::mix::Mix;
use crate::geometry::normal::Normal;
use num_traits::Zero;
use std::ops;
use webgl_rc::{DataType, TypeMark, Writable};
use webgl_rc::{IntoUniform, UniformValue};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }
    pub fn dot(self, v: Vector) -> f32 {
        self.x * v.x + self.y * v.y
    }
    pub fn square(self) -> f32 {
        self * self
    }
    pub fn length(self) -> f32 {
        (self * self).sqrt()
    }
    pub fn normalize(self) -> Vector {
        self / self.length()
    }
    pub fn normal(self) -> Vector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }
}

impl Zero for Vector {
    fn zero() -> Self {
        Default::default()
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl Distance for Vector {
    type Output = f32;
    fn distance(self, v: Self) -> f32 {
        (v - self).length()
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, v: Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, v: Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl ops::Mul for Vector {
    type Output = f32;

    fn mul(self, v: Vector) -> f32 {
        self.dot(v)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, s: f32) -> Vector {
        Vector {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, v: Vector) -> Vector {
        Vector {
            x: self * v.x,
            y: self * v.y,
        }
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, s: f32) -> Vector {
        Vector {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl ops::Div<Vector> for f32 {
    type Output = Vector;

    fn div(self, v: Vector) -> Vector {
        Vector {
            x: self / v.x,
            y: self / v.y,
        }
    }
}

impl ops::Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, v: Vector) -> Self::Output {
        Vector::new(self.x / v.x, self.y / v.y)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mix for Vector {
    type Fraction = f32;

    fn mix(self, other: Self, t: f32) -> Self {
        Vector {
            x: self.x.mix(other.x, t),
            y: self.y.mix(other.y, t),
        }
    }
}

impl TypeMark for Vector {
    fn data_type() -> DataType {
        DataType::Vec2
    }
}

impl IntoUniform for Vector {
    fn into_uniform(&self) -> UniformValue {
        UniformValue::Vec2([self.x, self.y])
    }
}

impl Writable for Vector {
    fn write(&self, output: &mut Vec<f32>) {
        output.push(self.x);
        output.push(self.y);
    }

    fn stride() -> usize {
        2
    }
}

impl Normal for Vector {
    fn normal(p1: Self, p2: Self) -> Vector {
        (p2 - p1).normal()
    }
}
