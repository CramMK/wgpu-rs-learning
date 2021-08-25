use winit::event::*;

#[derive(Debug)]
struct IsPressed(bool);

impl From<bool> for IsPressed {
    fn from(item: bool) -> Self {
        IsPressed(item)
    }
}

#[derive(Debug)]
enum ButtonPress {
    Up(IsPressed),
    Down(IsPressed),
    Left(IsPressed),
    Right(IsPressed),
    Forward(IsPressed),
    Backward(IsPressed),
}

pub struct CameraController {
    speed: f32,
    button_press: Option<ButtonPress>,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            button_press: None,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => {
                let pressed = (*state == ElementState::Pressed).into();
                match key {
                    VirtualKeyCode::W => {
                        self.button_press = Some(ButtonPress::Forward(pressed));
                        true
                    }
                    VirtualKeyCode::A => {
                        self.button_press = Some(ButtonPress::Left(pressed));
                        true
                    }
                    VirtualKeyCode::S => {
                        self.button_press = Some(ButtonPress::Backward(pressed));
                        true
                    }
                    VirtualKeyCode::D => {
                        self.button_press = Some(ButtonPress::Right(pressed));
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.button_press = Some(ButtonPress::Up(pressed));
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.button_press = Some(ButtonPress::Down(pressed));
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut crate::camera::Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        match self.button_press {
            Some(ButtonPress::Forward(IsPressed(true))) if forward_mag > self.speed => {
                camera.eye += forward_norm * self.speed;
            }
            Some(ButtonPress::Backward(IsPressed(true))) => {
                camera.eye -= forward_norm * self.speed;
            }
            _ => {}
        }
        // TODO add rest of the movement
    }
}
