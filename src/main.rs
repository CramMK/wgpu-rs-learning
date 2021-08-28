use winit::{dpi::PhysicalPosition, event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

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
    let mut window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    //window.set_cursor_visible(false);
    //window.set_cursor_grab(true).unwrap();

    // wait until Future is ready
    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event, // forward event
            window_id
        } if window_id == window.id() => {
            state.input(event, control_flow);
        }
        Event::RedrawRequested(_) => {
            // update the entire scene
            state.update(&mut window);

            // render the update
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
