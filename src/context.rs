use crate::frame::Frame;
use crate::geometry::Vector;
use crate::pool::{Pool, PoolEntry, PoolRequest};
use crate::tool::ToolRef;
use crate::tools::WavePencil;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use webgl_rc::{
    load_glsl, Attributes, BufferUsage, DepthBuffer, Gl, GlError, PrimitiveType, Program, Settings,
    Texture, TextureContent, TextureFilter, TextureFormat, TextureType, Uniforms,
};

#[derive(Debug, Clone)]
struct TextureRequest {
    gl: Gl,
    size: (u32, u32),
    format: TextureFormat,
    filter: TextureFilter,
}

impl PoolRequest for TextureRequest {
    type Item = Texture;
    type Error = GlError;

    fn distance(&self, item: &Texture) -> Option<f32> {
        if item.format() == self.format && item.size() == self.size {
            Some(0.0)
        } else {
            None
        }
    }

    fn prepare(&self, item: &mut Texture) {
        item.set_filter(self.filter)
    }

    fn create(&self) -> Result<Texture, GlError> {
        self.gl.texture(
            self.size.0,
            self.size.1,
            TextureType::Byte,
            self.format,
            TextureContent::None,
        )
    }
}

#[derive(Debug, Clone)]
struct ProgramRequest<'a> {
    gl: Gl,
    fragment: &'a str,
    vertex: &'a str,
}

impl<'a> PoolRequest for ProgramRequest<'a> {
    type Item = Program;
    type Error = GlError;

    fn distance(&self, item: &Self::Item) -> Option<f32> {
        if self.vertex == item.vertex_source() && self.fragment == item.fragment_source() {
            Some(0.0)
        } else {
            None
        }
    }

    fn prepare(&self, _item: &mut Self::Item) {}

    fn create(&self) -> Result<Self::Item, Self::Error> {
        self.gl.program(self.fragment, self.vertex)
    }
}

#[derive(Debug, Clone)]
struct DepthRequest {
    gl: Gl,
    size: (u32, u32),
}

impl PoolRequest for DepthRequest {
    type Item = DepthBuffer;
    type Error = GlError;

    fn distance(&self, item: &DepthBuffer) -> Option<f32> {
        if item.width() == self.size.0 && item.height() == self.size.1 {
            Some(0.0)
        } else {
            None
        }
    }

    fn prepare(&self, _item: &mut DepthBuffer) {}

    fn create(&self) -> Result<DepthBuffer, GlError> {
        self.gl.depth_buffer(self.size.0, self.size.1)
    }
}

#[derive(Debug)]
struct Data {
    gl: Gl,
    canvas: HtmlCanvasElement,
    textures: Pool<Texture>,
    depths: Pool<DepthBuffer>,
    programs: Pool<Program>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Context {
    data: Rc<Data>,
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(catch, constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Context, JsValue> {
        Ok(Context {
            data: Rc::new(Data {
                gl: Gl::new(&canvas)?,
                textures: Pool::new(8),
                depths: Pool::new(4),
                programs: Pool::new(32),
                canvas,
            }),
        })
    }

    pub(crate) fn canvas(&self) -> &HtmlCanvasElement {
        &self.data.canvas
    }

    pub(crate) fn gl(&self) -> Gl {
        self.data.gl.clone()
    }

    pub(crate) fn texture(
        &self,
        size: (u32, u32),
        format: TextureFormat,
        filter: TextureFilter,
    ) -> Result<PoolEntry<Texture>, GlError> {
        self.data.textures.take(&TextureRequest {
            gl: self.gl(),
            size,
            format,
            filter,
        })
    }

    pub(crate) fn depth(&self, width: u32, height: u32) -> Result<PoolEntry<DepthBuffer>, GlError> {
        self.data.depths.take(&DepthRequest {
            gl: self.gl(),
            size: (width, height),
        })
    }

    pub(crate) fn program(
        &self,
        fragment: &str,
        vertex: &str,
    ) -> Result<PoolEntry<Program>, GlError> {
        self.data.programs.take(&ProgramRequest {
            gl: self.gl(),
            fragment,
            vertex,
        })
    }

    #[wasm_bindgen(catch)]
    pub fn frame_with_size(&self, width: u32, height: u32) -> Result<Frame, JsValue> {
        Ok(Frame::new(self.clone(), (width, height))?)
    }

    pub fn wave_pencil(&self, thickness: f32, amplitude: f32, period: f32) -> ToolRef {
        ToolRef::new(WavePencil {
            thickness,
            amplitude,
            period,
        })
    }

    /// @param bounds is an array [left, top, right, bottom]
    /// @param color is an array [r, g, b, a]
    pub fn clear(&self, bounds: Vec<i32>, color: Vec<f32>) {
        assert_eq!(bounds.len(), 4);
        assert_eq!(color.len(), 4);
        self.gl().apply(
            Gl::settings()
                .viewport(bounds[0], bounds[1], bounds[2], bounds[3])
                .clear_color(color[0], color[1], color[2], color[3]),
            || self.gl().clear_color_buffer(),
        )
    }

    pub fn clear_full_screen(&self, color: Vec<f32>) {
        self.clear(
            vec![
                0,
                0,
                self.canvas().width() as i32,
                self.canvas().height() as i32,
            ],
            color,
        );
    }

    /// @param bounds is an array [left, top, right, bottom]
    #[wasm_bindgen(catch)]
    pub fn draw_frame_1(&self, frame: &Frame, bounds: Vec<i32>) -> Result<(), JsValue> {
        let program = self.program(
            load_glsl!("draw-frame/1.f.glsl"),
            load_glsl!("draw-frame/1.v.glsl"),
        )?;

        let w = self.canvas().width();
        let h = self.canvas().height();

        self.gl().apply(
            Gl::settings()
                .viewport(0, 0, w as i32, h as i32)
                .blend(false)
                .depth_test(false),
            || -> Result<(), GlError> {
                program.draw_arrays(
                    PrimitiveType::TriangleFan,
                    &Draw1Uniforms {
                        resolution: (w as f32, h as f32),
                        area: (
                            bounds[0] as f32,
                            bounds[1] as f32,
                            bounds[2] as f32,
                            bounds[3] as f32,
                        ),
                        source: frame.texture(),
                    },
                    &self.gl().items_buffer(
                        &[
                            Draw1Attributes::new(0.0, 0.0),
                            Draw1Attributes::new(1.0, 0.0),
                            Draw1Attributes::new(1.0, 1.0),
                            Draw1Attributes::new(0.0, 1.0),
                        ],
                        BufferUsage::Stream,
                    )?,
                );
                Ok(())
            },
        )?;

        Ok(())
    }

    #[wasm_bindgen(catch)]
    pub fn draw_frame_1_full_screen(&self, frame: &Frame) -> Result<(), JsValue> {
        self.draw_frame_1(
            frame,
            vec![
                0,
                0,
                self.canvas().width() as i32,
                self.canvas().height() as i32,
            ],
        )
    }

    /// Mix background and foreground in linear colorspace
    #[wasm_bindgen(catch)]
    pub fn draw_frame_2(
        &self,
        _background: &Frame,
        _foreground: &Frame,
        _bounds: Vec<i32>,
    ) -> Result<(), JsValue> {
        Ok(())
    }

    #[wasm_bindgen(catch)]
    pub fn draw_frame_2_full_screen(
        &self,
        background: &Frame,
        foreground: &Frame,
    ) -> Result<(), JsValue> {
        self.draw_frame_2(
            background,
            foreground,
            vec![
                0,
                0,
                self.canvas().width() as i32,
                self.canvas().height() as i32,
            ],
        )
    }
}

#[derive(Clone, Debug, Uniforms)]
struct Draw1Uniforms {
    resolution: (f32, f32),
    area: (f32, f32, f32, f32),
    source: Texture,
}

#[derive(Clone, Copy, Debug, Attributes)]
struct Draw1Attributes {
    position: Vector,
}

impl Draw1Attributes {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector::new(x, y),
        }
    }
}
