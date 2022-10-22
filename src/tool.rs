use crate::context::Context;
use crate::frame::Frame;
use crate::geometry::{Color, Touch};
use crate::touch_list::TouchList;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use webgl_rc::GlError;

pub trait Tool {
    fn draw(
        &self,
        frame: &mut Frame,
        path: &TouchList,
        color: Color,
        phases: [f32; 3],
        seed: i32,
    ) -> Result<(), GlError>;
}

#[wasm_bindgen]
pub struct ToolRef {
    tool: Box<dyn Tool>,
}

impl ToolRef {
    pub fn new(tool: impl Tool + 'static) -> Self {
        Self {
            tool: Box::new(tool),
        }
    }
}

impl Deref for ToolRef {
    type Target = Box<dyn Tool>;

    fn deref(&self) -> &Self::Target {
        &self.tool
    }
}
