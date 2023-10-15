use nalgebra_glm as glm;

use std::f32::consts::PI;

use glfw::{Action, Key};

pub struct Camera {
    pos: glm::Vec3,
    direction: glm::Vec3,
    up: glm::Vec3,

    speed_val: f32,
    mouse_sensitivity: f32,

    yaw: f32,
    pitch: f32,

    pub view: glm::Mat4,

    last_mouse_pos: glm::Vec2,

    pub handle_mouse: bool,
}

impl Camera {
    pub fn new(speed_value: f32, mouse_sen: f32) -> Self {
        let pos = glm::vec3(0.0, 3.0, 6.0);
        let direction = glm::vec3(0.0, 0.0, -1.0);
        let up = glm::vec3(0.0, 1.0, 0.0);

        Camera {
            pos,
            direction,
            up,

            speed_val: speed_value,
            mouse_sensitivity: mouse_sen,

            yaw: -PI * 0.5,
            pitch: 0.0,

            view: glm::look_at(&pos, &(pos + direction), &up),

            last_mouse_pos: glm::vec2(0.0, 0.0),

            handle_mouse: false,
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        *self = Camera::new(self.speed_val, self.mouse_sensitivity);
    }

    fn update(&mut self) {
        self.view = glm::look_at(&self.pos, &(self.pos + self.direction), &self.up);
    }

    pub fn handle_key_events(&mut self, window: &glfw::Window) {
        self.handle_key(
            window,
            Key::D,
            glm::normalize(&glm::cross(&self.direction, &self.up)),
        );
        self.handle_key(
            window,
            Key::A,
            -glm::normalize(&glm::cross(&self.direction, &self.up)),
        );

        self.handle_key(window, Key::W, self.direction);
        self.handle_key(window, Key::S, -self.direction);
    }

    fn handle_key(&mut self, window: &glfw::Window, key: Key, vec: glm::Vec3) {
        if window.get_key(key) == Action::Press {
            self.pos += vec * self.speed_val;

            self.update();
        }
    }

    pub fn look_at(&mut self, pos: glm::Vec2) {
        let mut offset = pos - self.last_mouse_pos;

        offset.y *= -1.0;

        self.last_mouse_pos = pos;

        if !self.handle_mouse {
            self.handle_mouse = true;
            return;
        }

        self.yaw += offset.x * self.mouse_sensitivity;
        self.pitch += offset.y * self.mouse_sensitivity;

        self.pitch = self.pitch.clamp(-PI * 0.47, PI * 0.47);

        let pitch_cos = self.pitch.cos();

        let direction = glm::vec3(
            self.yaw.cos() * pitch_cos,
            self.pitch.sin(),
            self.yaw.sin() * pitch_cos,
        );

        self.direction = glm::normalize(&direction);

        self.update();
    }
}
