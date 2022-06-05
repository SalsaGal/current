use std::num::NonZeroU32;
use std::ops::Index;

use image::{DynamicImage, GenericImageView};
use indexmap::IndexMap;
use wgpu::{
    BindGroup, BindGroupLayout, Color, Device, Queue, RenderPass, RenderPipeline, Sampler, Surface,
    SurfaceConfiguration,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::sprite::{ColorVertex, TextureVertex, Transform};

pub struct Graphics {
    pub device: Device,
    pub queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,

    pub texture_manager: TextureManager,

    color_pipeline: RenderPipeline,
    texture_pipeline: RenderPipeline,
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
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let mut texture_manager = TextureManager::new(&device);
        texture_manager.make_texture(
            &device,
            &queue,
            image::load_from_memory(include_bytes!("error.png")).unwrap(),
        );

        let color_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
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
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let texture_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
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
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
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
            depth_stencil: None,
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

            texture_manager,

            color_pipeline,
            texture_pipeline,
        }
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
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            let frame = Frame {
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
    }
}

pub struct Frame<'a> {
    pub texture_manager: &'a TextureManager,
    pub(crate) render_pass: RenderPass<'a>,
    pub(crate) color_pipeline: &'a RenderPipeline,
    pub(crate) texture_pipeline: &'a RenderPipeline,
    pub(crate) queue: &'a Queue,
}

pub type TextureID = usize;

pub struct TextureManager {
    textures: IndexMap<TextureID, BindGroup>,
    next_id: TextureID,
    bind_group_layout: BindGroupLayout,
    nearest_sampler: Sampler,
}

impl TextureManager {
    fn new(device: &Device) -> Self {
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
            next_id: 0,
            bind_group_layout,
            nearest_sampler,
        }
    }

    pub fn make_texture(
        &mut self,
        device: &Device,
        queue: &Queue,
        image: DynamicImage,
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
                    resource: wgpu::BindingResource::Sampler(&self.nearest_sampler),
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

    fn index(&self, index: TextureID) -> &Self::Output {
        if let Some(texture) = self.textures.get(&index) {
            texture
        } else {
            todo!("Return error texture")
        }
    }
}
