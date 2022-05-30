use std::mem::size_of;

use glam::{Vec3, Quat, Mat4};
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use wgpu::{VertexBufferLayout, Buffer, Color, VertexAttribute};

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
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
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

    pub transform: Transform,
    transform_buffer: Buffer,
    // TODO Move this bool to just updating it after changing it
    transform_outdated: bool,
}

impl Sprite {
    pub fn new_color_rect(graphics: &Graphics, color: Color) -> Self {
        let transform = Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        };

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

            transform_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[transform.matrix()]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            transform,
            transform_outdated: false,
        }
    }

    pub fn render<'a>(&'a self, frame: &mut Frame<'a>) {
        if self.transform_outdated {
            frame.queue.write_buffer(&self.transform_buffer, 0, bytemuck::cast_slice(&[self.transform.matrix()]));
        }

        match self.ty {
            SpriteType::Color => {
                frame.render_pass.set_pipeline(frame.color_pipeline);
                frame.render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                frame.render_pass.set_vertex_buffer(1, self.transform_buffer.slice(..));
                frame.render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                frame.render_pass.draw_indexed(0..self.index_count, 0, 0..1);
            },
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.transform_outdated = true;
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self.transform_outdated = true;
        self
    }
}

enum SpriteType {
    Color,
}

#[derive(Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    fn matrix(&self) -> [[f32; 4]; 4] {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation).to_cols_array_2d()
    }

    pub(crate) fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: size_of::<[f32; 4]>() as u64,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: size_of::<[f32; 8]>() as u64,
                    shader_location: 4,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: size_of::<[f32; 12]>() as u64,
                    shader_location: 5,
                },
            ],
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self { translation: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::ONE }
    }
}
