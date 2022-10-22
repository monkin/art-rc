use super::distance::Distance;
use super::vector::Vector;
use crate::geometry::{Mix, Normal};
use std::ops;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Touch {
    pub point: Vector,
    pub pressure: f32,
}

impl Touch {
    pub fn new(x: f32, y: f32, pressure: f32) -> Touch {
        Touch {
            point: Vector { x, y },
            pressure,
        }
    }
    pub fn x(&self) -> f32 {
        self.point.x
    }
    pub fn y(&self) -> f32 {
        self.point.y
    }
}

impl Distance for Touch {
    type Output = f32;

    fn distance(self, other: Self) -> Self::Output {
        self.point.distance(other.point)
    }
}

impl ops::Mul<f32> for Touch {
    type Output = Touch;
    fn mul(self, v: f32) -> Touch {
        Touch {
            point: self.point * v,
            pressure: self.pressure * v,
        }
    }
}

impl ops::Mul<Touch> for f32 {
    type Output = Touch;
    fn mul(self, v: Touch) -> Touch {
        v * self
    }
}

impl ops::Add<Touch> for Touch {
    type Output = Touch;
    fn add(self, v: Touch) -> Touch {
        Touch {
            point: self.point + v.point,
            pressure: self.pressure + v.pressure,
        }
    }
}

impl From<Touch> for Vector {
    fn from(touch: Touch) -> Self {
        touch.point
    }
}

impl Mix for Touch {
    type Fraction = f32;

    fn mix(self, other: Self, t: f32) -> Self {
        Touch {
            point: self.point.mix(other.point, t),
            pressure: self.pressure.mix(other.pressure, t),
        }
    }
}

impl Normal for Touch {
    fn normal(p1: Self, p2: Self) -> Vector {
        (p2.point - p1.point).normal()
    }
}
