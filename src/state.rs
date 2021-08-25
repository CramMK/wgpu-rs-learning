use wgpu::{
    PrimitiveTopology,
    util::DeviceExt,
};
use winit::{
    event::*,
    window::Window,
};

use crate::{texture, vertex::Vertex};

/// Hold state with important information
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,

    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,


    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    aqua_bind_group: wgpu::BindGroup,
    aqua_texture: texture::Texture,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        // actual screen size
        let size = window.inner_size();

        // handle to gpu
        // PRIMARY, VULKAN, DX12, METAL, BROWSER_WEBGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // a surface to draw to
        let surface = unsafe { instance.create_surface(window) };

        // get a physical adapter for the current system
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        // logical device and command queue to work with
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                // no special features required
                features: wgpu::Features::empty(),
                // limits of the adapter
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        // description of the swap_chain
        let sc_desc = wgpu::SwapChainDescriptor {
            // how textures will be used
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            // how swap_chain textures will be stored
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            // size of the swap_chain
            width: size.width,
            height: size.height,
            // how to sync with the display
            present_mode: wgpu::PresentMode::Fifo,
        };

        // actually create a swap_chain
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let aqua_bytes = include_bytes!("../img/aqua.png");
        let aqua_texture = texture::Texture::from_bytes(&device, &queue, aqua_bytes, "aqua").unwrap();

        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        );

        // bind groups can be changed on the fly, as long as they're in the same layout
        // every texture and sampler needs to be added to a bindgroup
        let aqua_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&aqua_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&aqua_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        // load shader file
        let shader = device.create_shader_module(
            &wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                flags: wgpu::ShaderFlags::all(),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            }
        );

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layput"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            }
        );

        // add everything to the render_pipeline
        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    // function name in shader.wgsl for [[stage(vertex)]]
                    entry_point: "main",
                    // specify memory layout
                    buffers: &[Vertex::desc()],

                },
                // needed to sotre color data to swap_chain
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "main",
                    // setup of color outputs
                    targets: &[wgpu::ColorTargetState {
                        format: sc_desc.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    // triangle facing forward when Counter Clock Wise
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    // only use 1 sample => no extra sampling
                    count: 1,
                    // use all
                    mask: !0,
                    // not using anti aliasing
                    alpha_to_coverage_enabled: false,
                }
            }
        );

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(crate::vertex::VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(crate::vertex::INDICES),
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        let num_indices = crate::vertex::INDICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            sc_desc, // saved, so we can create a new swap_chain later
            swap_chain,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            aqua_bind_group,
            aqua_texture,
        }
    }

    /// Corecctly resize the window
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        }
    }

    /// Idicate, whether an event has been fully processed
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
//            WindowEvent::CursorMoved { position, .. } => {
//                let x = (position.x as f64) / self.size.width as f64;
//                let y = (position.y as f64) / self.size.height as f64;
//                self.clear_color = wgpu::Color { r: x, g: x, b: y, a: 1.0};
//                true
//            },
//            WindowEvent::KeyboardInput {
//                input: KeyboardInput {
//                    state: ElementState::Pressed,
//                    virtual_keycode: Some(VirtualKeyCode::Space),
//                    ..
//                },
//                ..
//            } => {
//                self.use_pentagon = !self.use_pentagon;
//                true
//            }
            _ => false
        }
    }

    /// TODO
    pub fn update(&mut self) {
    }

    /// Generate commands for gpu to render to frame
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        // current screen
        let frame = self.swap_chain
            .get_current_frame()?
            .output;
        // encoder to talk to the gpu
        let mut encoder = self.device
            .create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        // create a render_pass
        let mut render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &frame.view, // draw to current screen
                        resolve_target: None, // no multisampling yet
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color { r: 0.2, g: 0.5, b: 0.5, a: 1.0 }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_bind_group(0, &self.aqua_bind_group, &[]);
        // draw triangle
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        //render_pass.draw(0..self.num_vertices, 0..1);

        // drop so encoder isn't borrowed mutually anymore
        drop(render_pass);

        // submit finished command buffers
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
