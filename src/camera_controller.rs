use cgmath::Vector2;
use cgmath::Point3;
use winit::{dpi::PhysicalPosition, event::*};

const MOUSE_SLOWDOWN: f32 = 100.0;

#[derive(Debug)]
enum ButtonPress {
    Up(ElementState),
    Down(ElementState),
    Left(ElementState),
    Right(ElementState),
    Forward(ElementState),
    Backward(ElementState),
    Reset(ElementState),
}

pub struct CameraController {
    speed: f32,
    button_press: Option<ButtonPress>,
    old_mouse: PhysicalPosition<f64>,
    mouse_movement: Vector2<f64>,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            button_press: None,
            old_mouse: PhysicalPosition::new(0.0, 0.0),
            mouse_movement: Vector2::new(0.0, 0.0),
        }
    }

    pub fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => match key {
                VirtualKeyCode::W => {
                    self.button_press = Some(ButtonPress::Forward(*state));
                }
                VirtualKeyCode::A => {
                    self.button_press = Some(ButtonPress::Left(*state));
                }
                VirtualKeyCode::S => {
                    self.button_press = Some(ButtonPress::Backward(*state));
                }
                VirtualKeyCode::D => {
                    self.button_press = Some(ButtonPress::Right(*state));
                }
                VirtualKeyCode::Space => {
                    self.button_press = Some(ButtonPress::Up(*state));
                }
                VirtualKeyCode::LShift => {
                    self.button_press = Some(ButtonPress::Down(*state));
                }
                VirtualKeyCode::Return => {
                    self.button_press = Some(ButtonPress::Reset(*state));
                }
                _ => {},
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_movement = Vector2 {
                    x: (*position).x - self.old_mouse.x,
                    y: (*position).y - self.old_mouse.y,
                };

                self.old_mouse = *position;
            }
            _ => {},
        }
    }

    /// Update the camera vectors
    /// Vectors are casted from (0, 0, 0) to both the target and the eye
    pub fn update_camera(&mut self, camera: &mut crate::camera::Camera) {
        // TODO what about delta time?
        // TODO this still feels a bit chunky...
        use cgmath::InnerSpace;
        // Casted eye -> target
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // cross product of forwards and up => perpendicular right
        let right = forward_norm.cross(camera.up);
        let right_norm = right.normalize();

        match self.button_press {
            // keyboard buttons:
            // target stays in place
            // eye moves
            // check movement vector, if too close
            Some(ButtonPress::Forward(ElementState::Pressed)) if forward_mag > self.speed => {
                camera.eye += forward_norm * self.speed;
            }
            Some(ButtonPress::Backward(ElementState::Pressed)) => {
                camera.eye -= forward_norm * self.speed;
            }
            Some(ButtonPress::Right(ElementState::Pressed)) => {
                let vector = right_norm * self.speed;
                camera.eye += vector;
                camera.target += vector;
            }
            Some(ButtonPress::Left(ElementState::Pressed)) => {
                let vector = right_norm * self.speed;
                camera.eye -= vector;
                camera.target -= vector;
            }
            Some(ButtonPress::Up(ElementState::Pressed)) => {
                let vector = camera.up.normalize() * self.speed;
                camera.eye += vector;
                camera.target += vector;
            }
            Some(ButtonPress::Down(ElementState::Pressed)) => {
                let vector = camera.up.normalize() * self.speed;
                camera.eye -= vector;
                camera.target -= vector;
            }
            Some(ButtonPress::Reset(ElementState::Pressed)) => {
                camera.target = Point3::new(0.0, 0.0, 0.0);
            }
            _ => {}
        }

        // mouse movement:
        // target moves
        // eye stays in place
        camera.target.x += (self.mouse_movement.x as f32) / MOUSE_SLOWDOWN;
        camera.target.y -= (self.mouse_movement.y as f32) / MOUSE_SLOWDOWN;
        self.mouse_movement = Vector2 { x: 0.0, y: 0.0 };

        dbg!(&camera.eye);
        dbg!(&camera.target);
        dbg!(self.mouse_movement);
    }
}
