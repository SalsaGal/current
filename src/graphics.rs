use std::ops::Index;
use std::{num::NonZeroU32, path::Path};

use glam::Vec2;
use image::{DynamicImage, GenericImageView};
use indexmap::IndexMap;
use text_to_png::TextRenderer;
use wgpu::{
    BindGroup, BindGroupLayout, Color, Device, Queue, RenderPass, RenderPipeline, Sampler, Surface,
    SurfaceConfiguration, TextureView,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::sprite::{ColorVertex, Filter, TextureVertex, Transform};

/// A unique identifier for each font stored.
pub type FontID = usize;

/// The core component that handles all general purpose graphics.
pub struct Graphics {
    pub device: Device,
    pub queue: Queue,
    surface: Surface,
    pub(crate) config: SurfaceConfiguration,
    /// If this is `Some`, then it specifies the size of the rendering space.
    /// Otherwise the rendering space is just the same as the pixel position. For example,
    /// in a 640x480 window, 320,0 would be on the right of the window, but if frame_size
    /// was Some(Vec2::new(2.0, 2.0)), then the right side of the window would be 1,0.
    pub frame_size: Option<Vec2>,

    pub fonts: IndexMap<FontID, TextRenderer>,
    next_font: FontID,
    pub texture_manager: TextureManager,
    depth_texture: TextureView,

    color_pipeline: RenderPipeline,
    texture_pipeline: RenderPipeline,
    /// The color used to clear the screen every frame. Black by default.
    pub background_color: Color,
}

impl Graphics {
    pub(crate) async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let texture_manager = TextureManager::new(&device, &queue);
        let depth_texture = Self::make_depth_texture(&device, &config);

        let color_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("color_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("color.wgsl").into()),
        });

        let color_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let color_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("color_pipeline"),
            layout: Some(&color_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &color_shader,
                entry_point: "vertex_main",
                buffers: &[ColorVertex::desc(), Transform::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &color_shader,
                entry_point: "fragment_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let texture_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("texture_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("texture.wgsl").into()),
        });

        let texture_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&texture_manager.bind_group_layout],
                push_constant_ranges: &[],
            });

        let texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("texture_pipeline"),
            layout: Some(&texture_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &texture_shader,
                entry_point: "vertex_main",
                buffers: &[TextureVertex::desc(), Transform::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &texture_shader,
                entry_point: "fragment_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            device,
            queue,
            surface,
            config,
            frame_size: None,

            fonts: IndexMap::new(),
            next_font: 0,
            texture_manager,
            depth_texture,

            color_pipeline,
            texture_pipeline,
            background_color: Color::BLACK,
        }
    }

    fn make_depth_texture(device: &Device, config: &SurfaceConfiguration) -> TextureView {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub(crate) fn render<F: FnMut(Frame)>(&mut self, mut function: F) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            let frame = Frame {
                frame_size: self.frame_size.unwrap_or_else(|| self.get_window_size()),
                texture_manager: &self.texture_manager,
                render_pass,
                color_pipeline: &self.color_pipeline,
                texture_pipeline: &self.texture_pipeline,
                queue: &self.queue,
            };

            function(frame);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub(crate) fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.depth_texture = Self::make_depth_texture(&self.device, &self.config);
    }

    /// Get the size of the window
    pub fn get_window_size(&self) -> Vec2 {
        glam::UVec2::new(self.config.width, self.config.height).as_vec2()
    }

    /// Get the size of the renderable frame
    pub fn get_frame_size(&self) -> Vec2 {
        self.frame_size.unwrap_or_else(|| self.get_window_size())
    }

    /// Load a font from the true type font at `path`.
    pub fn load_font<T: AsRef<Path>>(&mut self, path: T) -> FontID {
        let contents = std::fs::read(path).unwrap();
        let font = TextRenderer::try_new_with_ttf_font_data(&contents).unwrap();
        self.fonts.insert(self.next_font, font);
        self.next_font += 1;
        self.next_font - 1
    }

    /// Empty the font cache
    pub fn clear_fonts(&mut self) {
        self.fonts.clear();
        self.next_font = 0;
    }
}

/// A handle for structures that are needed during rendering itself.
pub struct Frame<'a> {
    /// The size of the window or frame if `Graphics::frame_size` is some.
    pub frame_size: Vec2,
    pub texture_manager: &'a TextureManager,
    pub render_pass: RenderPass<'a>,
    pub queue: &'a Queue,
    pub(crate) color_pipeline: &'a RenderPipeline,
    pub(crate) texture_pipeline: &'a RenderPipeline,
}

/// An identifier used to locate textures within a `TextureManager`'s list of textures.
pub type TextureID = usize;

/// Contains all textures and a collection of everything required for them
pub struct TextureManager {
    textures: IndexMap<TextureID, BindGroup>,
    error_texture: BindGroup,
    next_id: TextureID,

    bind_group_layout: BindGroupLayout,
    linear_sampler: Sampler,
    nearest_sampler: Sampler,
}

impl TextureManager {
    fn new(device: &Device, queue: &Queue) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            textures: IndexMap::new(),
            error_texture: Self::make_error_texture(
                device,
                queue,
                &nearest_sampler,
                &bind_group_layout,
            ),
            next_id: 0,

            bind_group_layout,
            linear_sampler,
            nearest_sampler,
        }
    }

    fn make_error_texture(
        device: &Device,
        queue: &Queue,
        sampler: &Sampler,
        bind_group_layout: &BindGroupLayout,
    ) -> BindGroup {
        let size = wgpu::Extent3d {
            width: 2,
            height: 2,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image::load_from_memory(include_bytes!("error.png"))
                .unwrap()
                .as_rgba8()
                .unwrap(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * 2),
                rows_per_image: NonZeroU32::new(2),
            },
            size,
        );

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    /// Deletes all values in the texture cache.
    pub fn clear(&mut self) {
        self.textures.clear();
        self.next_id = 0;
    }

    /// Get the texture if it is available. Index into the manager if you want
    /// to get an error texture to replace missing textures.
    pub fn get(&self, id: TextureID) -> Option<&BindGroup> {
        self.textures.get(&id)
    }

    /// Create a texture from `image` and store it in the texture cache. Returns the
    /// newly loaded texture's ID.
    pub fn make_texture(
        &mut self,
        device: &Device,
        queue: &Queue,
        image: DynamicImage,
        filter: Filter,
    ) -> TextureID {
        let (width, height) = image.dimensions();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image.as_rgba8().unwrap(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * width),
                rows_per_image: NonZeroU32::new(height),
            },
            size,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(match filter {
                        Filter::Linear => &self.linear_sampler,
                        Filter::Nearest => &self.nearest_sampler,
                    }),
                },
            ],
        });

        self.textures.insert(self.next_id, bind_group);
        self.next_id += 1;

        self.next_id - 1
    }
}

impl Index<TextureID> for TextureManager {
    type Output = BindGroup;

    /// Get the texture at `index` from the texture cache. If it is missing return the
    /// error texture that is baked into the program.
    fn index(&self, index: TextureID) -> &Self::Output {
        if let Some(texture) = self.textures.get(&index) {
            texture
        } else {
            &self.error_texture
        }
    }
}
