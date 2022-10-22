use crate::context::Context;
use crate::frame::Frame;
use crate::geometry::{Color, Touch, Vector};
use crate::path::{
    SmoothIterator, SplitIterator, WithNormal, WithNormalIterator, WithOffset, WithOffsetIterator,
};
use crate::pool::{PoolEntry, PoolRequest};
use crate::tool::Tool;
use crate::touch_list::TouchList;
use std::f32::consts::PI;
use std::ops::Deref;
use webgl_rc::{
    load_glsl, Attributes, BufferUsage, DepthFunction, Gl, GlError, PrimitiveType, Program,
    Settings, Texture, TextureFilter, TextureFormat, Uniforms,
};

const MAX_THICKNESS_EASING_LEN: f32 = 10.0;
const MAX_AMPLITUDE_EASING_LEN: f32 = 0.75;

fn easing(t: f32) -> f32 {
    if t >= 1.0 {
        1.0
    } else if t <= 0.0 {
        0.0
    } else {
        ((PI * t).cos() - 1.0) * -0.5
    }
}

// --------------------------------------
// Draw Shaders

#[derive(Clone, Copy, Uniforms)]
struct DrawPhaseUniforms {
    resolution: Vector,
}

#[derive(Clone, Copy, Debug, Attributes)]
struct DrawPhaseAttributes {
    position: Vector,
    /// 0, or 1
    offset: f32,
    width: f32,
}

// --------------------------------------
// Pencil

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WavePencil {
    pub thickness: f32,
    pub amplitude: f32,
    pub period: f32,
}

impl WavePencil {
    fn draw_phase_list(
        &self,
        context: Context,
        size: (u32, u32),
        path: &TouchList,
        phases: [f32; 3],
    ) -> Result<PoolEntry<Texture>, GlError> {
        let target = context.texture(size, TextureFormat::Rgb, TextureFilter::Nearest)?;
        let points: Vec<_> = WithOffsetIterator::new(WithNormalIterator::new(SplitIterator::new(
            SmoothIterator::new(SmoothIterator::new(SplitIterator::new(
                path.iter().map(|v| v.clone()),
                path.pixel_size(),
            ))),
            1.0,
        )))
        .collect();

        let gl = context.gl();

        gl.apply(
            Gl::settings()
                .blend(false)
                .clear_color(0.0, 0.0, 0.0, 0.0)
                .frame_buffer(gl.frame_buffer_with_depth(
                    target.deref().clone(),
                    gl.depth_buffer(size.0, size.1)?,
                )?)
                .depth_function(DepthFunction::Less)
                .clear_depth(1.0)
                .viewport(0, 0, size.0 as i32, size.1 as i32),
            || -> Result<(), GlError> {
                let draw_phase_program = context.program(
                    load_glsl!("tools/pencil/phase.f.glsl"),
                    load_glsl!("tools/pencil/phase.v.glsl"),
                )?;
                gl.clear_color_buffer();
                for i in 0..2 {
                    self.draw_phase(
                        context.gl(),
                        &draw_phase_program,
                        &points,
                        Vector::new(size.0 as f32, size.1 as f32),
                        phases[i],
                        i as i32,
                    )?
                }
                Ok(())
            },
        )?;
        Ok(target)
    }

    fn draw_phase(
        &self,
        gl: Gl,
        program: &PoolEntry<Program>,
        path: &Vec<WithOffset<WithNormal<Touch>>>,
        resolution: Vector,
        _phase: f32,
        channel: i32,
    ) -> Result<(), GlError> {
        // wave it here
        let transformed: Vec<_> = path
            .iter()
            .map(|p: &WithOffset<WithNormal<Touch>>| WithOffset {
                offset: p.offset,
                point: WithNormal {
                    normal: p.point.normal,
                    point: p.point.point.point,
                },
            })
            .collect();

        let points: Vec<DrawPhaseAttributes> = transformed
            .iter()
            .flat_map(|point: &WithOffset<WithNormal<Vector>>| {
                let _offset = point.offset;
                let normal = point.point.normal;
                let position = point.point.point;

                let shift = normal * self.thickness;

                [
                    DrawPhaseAttributes {
                        position: position - shift,
                        offset: -1.0,
                        width: self.thickness,
                    },
                    DrawPhaseAttributes {
                        position,
                        offset: 0.0,
                        width: self.thickness,
                    },
                    DrawPhaseAttributes {
                        position: position + shift,
                        offset: 1.0,
                        width: self.thickness,
                    },
                ]
            })
            .collect();

        let elements: Vec<u32> = (1..transformed.len())
            .flat_map(|i| {
                let p1 = (i * 3) as u32;
                let p2 = p1 + 3;

                [
                    // t1
                    p1,
                    p2,
                    p1 + 1,
                    // t2
                    p1 + 1,
                    p2,
                    p2 + 1,
                    // t3
                    p1 + 1,
                    p2 + 1,
                    p1 + 2,
                    // t4
                    p1 + 2,
                    p2 + 1,
                    p2 + 2,
                ]
            })
            .collect();

        gl.clear_depth_buffer();
        gl.apply(
            Gl::settings().color_mask(channel == 0, channel == 1, channel == 2, channel == 3),
            || -> Result<(), GlError> {
                program.deref().draw_element_arrays(
                    PrimitiveType::Triangles,
                    &DrawPhaseUniforms { resolution },
                    &gl.items_buffer(&points, BufferUsage::Stream)?,
                    &gl.elements_buffer(&elements, BufferUsage::Stream)?,
                );
                Ok(())
            },
        )
    }
}

impl Tool for WavePencil {
    fn draw(
        &self,
        frame: &mut Frame,
        path: &TouchList,
        _color: Color,
        phases: [f32; 3],
        _seed: i32,
    ) -> Result<(), GlError> {
        let texture = self.draw_phase_list(frame.context(), frame.size(), path, phases)?;
        frame.replace_texture(texture);
        Ok(())
    }
}
