use wgpu::{
    PrimitiveTopology,
    util::DeviceExt,
};
use winit::{
    event::*,
    window::Window,
};

use crate::vertex::Vertex;

/// Hold state with important information
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    //challenge_render_pipeline: wgpu::RenderPipeline,
    use_challenge_render_pipeline: bool,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
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

        // load shader file
        let shader = device.create_shader_module(
            &wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                flags: wgpu::ShaderFlags::all(),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            }
        );

        // TODO
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layput"),
                bind_group_layouts: &[],
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
        /*
        // overwrite challenge shader file to shader
        let shader = device.create_shader_module(
            &wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                flags: wgpu::ShaderFlags::all(),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("challenge_shader.wgsl").into()),
            }
        );

        // TODO
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layput"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }
        );

        // add everything to the challnge_render_pipeline
        let challenge_render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    // function name in shader.wgsl for [[stage(vertex)]]
                    entry_point: "main",
                    // already specified in the shader
                    buffers: &[],

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
        */

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(crate::vertex::VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let num_vertices = crate::vertex::VERTICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            sc_desc, // saved, so we can create a new swap_chain later
            swap_chain,
            size,
            clear_color: wgpu::Color { r: 0.6, g: 0.6, b: 0.1, a: 1.0 },
            render_pipeline,
            //challenge_render_pipeline,
            use_challenge_render_pipeline: false,
            vertex_buffer,
            num_vertices,
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
            WindowEvent::CursorMoved { position, .. } => {
                let x = (position.x as f64) / self.size.width as f64;
                let y = (position.y as f64) / self.size.height as f64;
                self.clear_color = wgpu::Color { r: x, g: x, b: y, a: 1.0};
                true
            },
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
                ..
            } => {
                self.use_challenge_render_pipeline = !self.use_challenge_render_pipeline;
                true
            }
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
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });

        // set pipeline
        /*
        if self.use_challenge_render_pipeline {
            render_pass.set_pipeline(&self.challenge_render_pipeline);
        } else {
            render_pass.set_pipeline(&self.render_pipeline);
    }
         */
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // draw triangle
        render_pass.draw(0..self.num_vertices, 0..1);

        // drop so encoder isn't borrowed mutually anymore
        drop(render_pass);

        // submit finished command buffers
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
