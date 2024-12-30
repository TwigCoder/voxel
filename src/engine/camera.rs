use glam::{Mat4, Vec3};
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new(position: Vec3, aspect: f32) -> Self {
        Self {
            position,
            yaw: -45.0,
            pitch: -35.0,
            up: Vec3::Y,
            aspect,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn get_view_direction(&self) -> Vec3 {
        let (yaw_sin, yaw_cos) = self.yaw.to_radians().sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.to_radians().sin_cos();
        Vec3::new(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize()
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_to_rh(self.position, self.get_view_direction(), self.up);
        let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
        proj * view
    }
}
pub struct CameraController {
    speed: f32,
    sensitivity: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_run_pressed: bool,
    mouse_pressed: bool,
    last_mouse_pos: Option<(f64, f64)>,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            sensitivity: 0.038,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            is_run_pressed: false,
            mouse_pressed: false,
            last_mouse_pos: None,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent, camera: &mut Camera) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LControl => {
                        self.is_run_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Right,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                if !self.mouse_pressed {
                    self.last_mouse_pos = None;
                }
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_pressed {
                    if let Some((last_x, last_y)) = self.last_mouse_pos {
                        let dx = position.x - last_x;
                        let dy = position.y - last_y;

                        camera.yaw -= dx as f32 * self.sensitivity;
                        camera.pitch =
                            (camera.pitch - dy as f32 * self.sensitivity).clamp(-89.0, 89.0);
                    }
                    self.last_mouse_pos = Some((position.x, position.y));
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        let forward = camera.get_view_direction();
        let right = forward.cross(camera.up).normalize();

        let mut velocity = Vec3::ZERO;

        if self.is_forward_pressed {
            velocity += forward;
        }
        if self.is_backward_pressed {
            velocity -= forward;
        }
        if self.is_right_pressed {
            velocity += right;
        }
        if self.is_left_pressed {
            velocity -= right;
        }
        if self.is_up_pressed {
            velocity += Vec3::Y;
        }
        if self.is_down_pressed {
            velocity -= Vec3::Y;
        }

        if velocity.length_squared() > 0.0 {
            velocity = velocity.normalize();
        }

        let actual_speed = if self.is_run_pressed {
            self.speed * 2.0
        } else {
            self.speed
        };

        camera.position += velocity * actual_speed;
    }
}
