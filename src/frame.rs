use crate::context::Context;
use crate::geometry::Color;
use crate::pool::PoolEntry;
use crate::tool::ToolRef;
use crate::touch_list::TouchList;
use std::mem::replace;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;
use webgl_rc::{Gl, GlError, Settings, Texture, TextureFilter, TextureFormat, TextureType};

#[wasm_bindgen]
pub struct Frame {
    context: Context,
    texture: PoolEntry<Texture>,
}

#[wasm_bindgen]
impl Frame {
    pub(crate) fn new(context: Context, size: (u32, u32)) -> Result<Self, GlError> {
        Ok(Self {
            texture: context.texture(size, TextureFormat::Rgba, TextureFilter::Linear)?,
            context,
        })
    }

    pub(crate) fn from_texture(context: Context, texture: PoolEntry<Texture>) -> Frame {
        Frame { context, texture }
    }

    pub(crate) fn from_image(context: Context, image: HtmlImageElement) -> Result<Frame, GlError> {
        let texture = context.texture(
            (image.width(), image.height()),
            TextureFormat::Rgba,
            TextureFilter::Linear,
        )?;
        texture.write_image(&image)?;

        Ok(Frame { context, texture })
    }

    pub(crate) fn gl(&self) -> Gl {
        self.context.gl()
    }

    pub(crate) fn context(&self) -> Context {
        self.context.clone()
    }

    pub(crate) fn texture(&self) -> Texture {
        self.texture.clone()
    }

    pub(crate) fn replace_texture(&mut self, texture: PoolEntry<Texture>) -> PoolEntry<Texture> {
        replace(&mut self.texture, texture)
    }

    pub(crate) fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn width(&self) -> u32 {
        self.texture.width()
    }
    pub fn height(&self) -> u32 {
        self.texture.height()
    }

    /// @param color array containing [r, g, b, a]
    /// @param phases three elements array [p1, p2, p3]
    #[wasm_bindgen(catch)]
    pub fn draw(
        &mut self,
        tool: &ToolRef,
        path: &TouchList,
        color: Vec<f32>,
        phases: Vec<f32>,
        seed: i32,
    ) -> Result<(), JsValue> {
        assert_eq!(color.len(), 4);
        assert_eq!(phases.len(), 3);

        tool.draw(
            self,
            path,
            Color::new(color[0], color[1], color[2], color[3]),
            [phases[0], phases[1], phases[2]],
            seed,
        )?;
        Ok(())
    }

    #[wasm_bindgen(catch)]
    pub fn clear(&self, r: f32, g: f32, b: f32, alpha: f32) -> Result<(), JsValue> {
        self.texture.clear(r, g, b, alpha)?;
        Ok(())
    }
}
