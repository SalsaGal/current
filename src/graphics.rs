use wgpu::{Device, Queue, SurfaceConfiguration, Surface, RenderPipeline, Color, RenderPass};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::sprite::ColorVertex;

pub struct Graphics {
    pub device: Device,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,

    color_pipeline: RenderPipeline,
}

impl Graphics {
    pub(crate) async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe {instance.create_surface(&window)};
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        }, None).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let color_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("color_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("color.wgsl").into()),
        });

        let color_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
                buffers: &[ColorVertex::desc()],
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

        Self {
            device,
            queue,
            surface,
            config,

            color_pipeline,
        }
    }

    pub(crate) fn render<F: Fn(Frame)>(&mut self, function: F) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

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
                render_pass,
                color_pipeline: &self.color_pipeline,
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
    pub(crate) render_pass: RenderPass<'a>,
    pub(crate) color_pipeline: &'a RenderPipeline,
}
