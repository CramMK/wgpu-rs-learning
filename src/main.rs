use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod state;
mod vertex;
mod texture;
mod camera;
mod uniform;
mod camera_controller;

use crate::state::State;

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
