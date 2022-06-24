use std::mem::size_of;

use glam::{Mat4, Quat, Vec2, Vec3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, Color, VertexAttribute, VertexBufferLayout};

use crate::graphics::{Frame, Graphics, TextureID};

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

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl TextureVertex {
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
                    format: wgpu::VertexFormat::Float32x2,
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
    pub transform_outdated: bool,
    transform_buffer: Buffer,
}

impl Sprite {
    pub fn new_color_mesh(graphics: &Graphics, vertices: &[ColorVertex], indices: &[u16]) -> Self {
        let transform = Transform::default();

        Self {
            vertex_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: indices.len() as u32,
            ty: SpriteType::Color,

            transform_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[transform.matrix(graphics.get_screen_size())]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            transform,
            transform_outdated: false,
        }
    }

    pub fn new_path_mesh(
        graphics: &mut Graphics,
        vertices: &[TextureVertex],
        indices: &[u16],
        path: &str,
        filter: Filter,
    ) -> Self {
        let id = graphics.texture_manager.make_texture(
            &graphics.device,
            &graphics.queue,
            image::open(path).unwrap(),
            filter,
        );
        Self::new_texture_mesh(graphics, vertices, indices, id)
    }

    pub fn new_texture_mesh(
        graphics: &Graphics,
        vertices: &[TextureVertex],
        indices: &[u16],
        texture_id: TextureID,
    ) -> Self {
        let transform = Transform::default();

        Self {
            vertex_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: indices.len() as u32,
            ty: SpriteType::Texture(texture_id),

            transform_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[transform.matrix(graphics.get_screen_size())]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            transform,
            transform_outdated: false,
        }
    }

    pub fn new_color_rect(graphics: &Graphics, color: Color) -> Self {
        let transform = Transform::default();

        Self {
            vertex_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[
                    ColorVertex {
                        position: [-0.5, -0.5, 0.0],
                        color: [
                            color.r as f32,
                            color.g as f32,
                            color.b as f32,
                            color.a as f32,
                        ],
                    },
                    ColorVertex {
                        position: [0.5, -0.5, 0.0],
                        color: [
                            color.r as f32,
                            color.g as f32,
                            color.b as f32,
                            color.a as f32,
                        ],
                    },
                    ColorVertex {
                        position: [0.5, 0.5, 0.0],
                        color: [
                            color.r as f32,
                            color.g as f32,
                            color.b as f32,
                            color.a as f32,
                        ],
                    },
                    ColorVertex {
                        position: [-0.5, 0.5, 0.0],
                        color: [
                            color.r as f32,
                            color.g as f32,
                            color.b as f32,
                            color.a as f32,
                        ],
                    },
                ]),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice::<u16, u8>(&[0, 1, 2, 0, 2, 3]),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: 6,
            ty: SpriteType::Color,

            transform_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[transform.matrix(graphics.get_screen_size())]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            transform,
            transform_outdated: false,
        }
    }

    pub fn new_path_rect(graphics: &mut Graphics, path: &str, filter: Filter) -> Self {
        let id = graphics.texture_manager.make_texture(
            &graphics.device,
            &graphics.queue,
            image::open(path).unwrap(),
            filter,
        );
        Self::new_texture_rect(graphics, id)
    }

    pub fn new_texture_rect(graphics: &Graphics, id: TextureID) -> Self {
        let transform = Transform::default();

        Self {
            vertex_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[
                    TextureVertex {
                        position: [-0.5, -0.5, 0.0],
                        tex_coords: [0.0, 1.0],
                    },
                    TextureVertex {
                        position: [0.5, -0.5, 0.0],
                        tex_coords: [1.0, 1.0],
                    },
                    TextureVertex {
                        position: [0.5, 0.5, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                    TextureVertex {
                        position: [-0.5, 0.5, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                ]),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice::<u16, u8>(&[0, 1, 2, 0, 2, 3]),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: 6,
            ty: SpriteType::Texture(id),

            transform_buffer: graphics.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[transform.matrix(graphics.get_screen_size())]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            transform,
            transform_outdated: false,
        }
    }

    pub fn render_to<'a>(&'a self, frame: &mut Frame<'a>) {
        if self.transform_outdated {
            frame.queue.write_buffer(
                &self.transform_buffer,
                0,
                bytemuck::cast_slice(&[self.transform.matrix(frame.window_size)]),
            );
        }

        match self.ty {
            SpriteType::Color => {
                frame.render_pass.set_pipeline(frame.color_pipeline);
                frame
                    .render_pass
                    .set_vertex_buffer(0, self.vertex_buffer.slice(..));
                frame
                    .render_pass
                    .set_vertex_buffer(1, self.transform_buffer.slice(..));
                frame
                    .render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                frame.render_pass.draw_indexed(0..self.index_count, 0, 0..1);
            }
            SpriteType::Texture(id) => {
                frame.render_pass.set_pipeline(frame.texture_pipeline);
                frame
                    .render_pass
                    .set_vertex_buffer(0, self.vertex_buffer.slice(..));
                frame
                    .render_pass
                    .set_vertex_buffer(1, self.transform_buffer.slice(..));
                frame
                    .render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                frame
                    .render_pass
                    .set_bind_group(0, &frame.texture_manager[id], &[]);
                frame.render_pass.draw_indexed(0..self.index_count, 0, 0..1);
            }
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
    Texture(TextureID),
}

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec2,
}

macro_rules! transform_methods {
    ($($i: ident: $t: ty),*) => {
        $(
            paste::paste! {
                pub fn [<set_ $i>](&mut self, $i: $t) {
                    self.$i = $i;
                }

                pub fn [<with_ $i>](mut self, $i: $t) -> Self {
                    self.$i = $i;
                    self
                }

                pub fn $i($i: $t) -> Self {
                    Self {
                        $i,
                        ..Default::default()
                    }
                }
            }
        )*
    };
}

impl Transform {
    transform_methods!(translation: Vec3, rotation: Quat, scale: Vec2);

    pub fn with_straight_rotation(mut self, angle: f32) -> Self {
        self.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, angle);
        self
    }

    fn matrix(&self, window_size: Vec2) -> [[f32; 4]; 4] {
        let half = window_size / 2.0;
        let projection = Mat4::orthographic_rh(-half.x, half.x, -half.y, half.y, -100.0, 100.0);

        (projection
            * Mat4::from_scale_rotation_translation(
                self.scale.extend(1.0),
                self.rotation,
                self.translation,
            ))
        .to_cols_array_2d()
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
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec2::ONE,
        }
    }
}

pub enum Filter {
    Linear,
    Nearest,
}
