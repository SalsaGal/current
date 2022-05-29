use std::mem::size_of;

use wgpu::util::{DeviceExt, BufferInitDescriptor};
use wgpu::{VertexBufferLayout, Buffer, Color};

use crate::graphics::{Graphics, Frame};

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl ColorVertex {
    pub(crate) fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: size_of::<[f32; 3]>() as u64,
                    shader_location: 1,
                },
            ],
        }
    }
}

pub struct Sprite {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
    ty: SpriteType,
}

impl Sprite {
    pub fn new_color_rect(graphics: &Graphics, color: Color) -> Self {
        Self {
            vertex_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[
                    ColorVertex {
                        position: [0.0, 0.0, 0.0],
                        color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
                    },
                    ColorVertex {
                        position: [1.0, 0.0, 0.0],
                        color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
                    },
                    ColorVertex {
                        position: [1.0, 1.0, 0.0],
                        color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
                    },
                    ColorVertex {
                        position: [0.0, 1.0, 0.0],
                        color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
                    },
                ]),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice::<u16, u8>(&[
                    0, 1, 2,
                    0, 2, 3,
                ]),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: 6,
            ty: SpriteType::Color,
        }
    }

    pub fn render<'a>(&'a self, frame: &mut Frame<'a>) {
        match self.ty {
            SpriteType::Color => {
                frame.render_pass.set_pipeline(frame.color_pipeline);
                frame.render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                frame.render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                frame.render_pass.draw_indexed(0..self.index_count, 0, 0..1);
            },
        }
    }
}

enum SpriteType {
    Color,
}
