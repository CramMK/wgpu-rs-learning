use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// Hold state with important information
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
}

impl State {
    async fn new(window: &Window) -> Self {
        // actual screen size
        let size = window.inner_size();
        // handle to gpu
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        // a surface to draw to
        let surface = unsafe { instance.create_surface(window) };
        // get a working adapter for the current system
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();
        // adapter device and queue
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

        Self {
            surface,
            device,
            queue,
            sc_desc, // saved, so we can create a new swap_chain later
            swap_chain,
            size,
            clear_color: wgpu::Color { r: 0.6, g: 0.6, b: 0.1, a: 1.0 },
        }
    }

    /// Corecctly resize the window
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        }
    }

    /// Idicate, whether an event has been fully processed
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let x = (position.x as f64) / 1000 as f64;
                let y = (position.y as f64) / 1000 as f64;
                self.clear_color = wgpu::Color { r: x, g: x, b: y, a: 1.0};
                true
            }
            _ => false
        }
    }

    /// TODO
    fn update(&mut self) {
    }

    /// Generate commands for gpu to render to frame
    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
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
        let _render_pass = encoder.begin_render_pass(
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

        // drop so encoder isn't borrowed mutually anymore
        drop(_render_pass);

        // submit finished command buffers
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    // wait until Future is ready
    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event, // forward event
            window_id
        } if window_id == window.id() // be sure to only use the current one
            => if !state.input(event) { // don't continue if input hasn't been processed yet
                match event {
                    // TODO can't this be moved to state.input()
                    WindowEvent::CloseRequested |
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            // ignore the other entries
                            ..
                        },
                        // ignore the other entries
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size)
                    },
                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                        state.resize(**new_inner_size);
                    },
                    // Discard all other WindowEvents
                    _ => {}
                }
            },
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {},
                // recreate swap_chain
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                // no memory left, so quit
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // Outdated, Timeout should be handled by the nnext frame
                Err(error) => eprintln!("{:?}", error),
            }
        },
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually request it
            window.request_redraw();
        }
        // Discard all other Events
        _ => {}
    });
}
