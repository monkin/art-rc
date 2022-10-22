use crate::geometry::Touch;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TouchList {
    pixel_size: f32,
    points: Vec<Touch>,
}

#[wasm_bindgen]
impl TouchList {
    #[wasm_bindgen(constructor)]
    pub fn new(pixel_size: f32) -> Self {
        Self {
            pixel_size,
            points: Default::default(),
        }
    }

    pub fn with_capacity(pixel_size: f32, capacity: usize) -> Self {
        Self {
            pixel_size,
            points: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn push(&mut self, x: f32, y: f32, pressure: f32) {
        self.points.push(Touch::new(x, y, pressure));
    }

    pub fn pixel_size(&self) -> f32 {
        self.pixel_size
    }
}

impl Deref for TouchList {
    type Target = Vec<Touch>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl DerefMut for TouchList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}
