use super::{utils::*, *};

mod camera;
use camera::Camera;

mod game_logic;
use game_logic::GameLogic;

use std::f32::consts::PI;
use std::mem::size_of;
use std::sync::mpsc::Receiver;

use glm::identity;
use nalgebra_glm as glm;

use glfw::{Action, Context, Key, WindowHint, WindowMode};
use glfw::{CursorMode, WindowEvent};

pub struct Tetris {
    screen_width: u32,
    screen_height: u32,

    sector_height: f32,

    cursor_disabled: bool,

    shader: Shader,
    camera: Camera,
    game: GameLogic,

    events: Receiver<(f64, glfw::WindowEvent)>,
    window: glfw::Window,
    glfw: glfw::Glfw,
}

impl Tetris {
    pub fn new(screen_width: u32, screen_height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("cannot init glfw");

        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, events) = glfw
            .create_window(screen_width, screen_height, title, WindowMode::Windowed)
            .expect("cannot create window");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_mode(CursorMode::Disabled);

        let _gl = gl::load_with(|s| glfw.get_proc_address_raw(s));

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1)); // open vsync

        gl_call!(gl::Enable(gl::BLEND));
        gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));

        // gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)); // wireframe mode

        gl_call!(gl::Enable(gl::DEPTH_TEST));

        Tetris {
            screen_width,
            screen_height,

            sector_height: 1.0,
            cursor_disabled: true,

            camera: Camera::new(0.05, 0.005),

            shader: Shader::new("./res/vertex.glsl", "./res/fragment.glsl"),

            game: GameLogic::new(15, 20),

            glfw,
            window,
            events,
        }
    }

    pub fn run(&mut self) {
        let (vertices, indices) = self.create_shape(2.7, 3.0);

        let vao = VertexArrayObject::new();

        let vbo = BufferObject::new(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);

        let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, &indices, gl::STATIC_DRAW);

        vao.bind();
        vbo.bind();
        ebo.bind();

        BufferObject::create_vertex(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<GLfloat>(), 0);

        self.shader.bind();

        let u_final_mat = self.shader.get_uniform("u_final_mat");

        let u_color = self.shader.get_uniform("u_color");

        let aspect_ratio = (self.screen_width as f32) / (self.screen_height as f32);

        let projection = glm::perspective(PI * 0.25, aspect_ratio, 0.1, 100.0);

        let sector_angle = 2.0 * PI / (self.game.grid_width as f32);

        while !self.window.should_close() {
            self.glfw.poll_events();

            self.handle_events();

            self.camera.handle_key_events(&self.window);

            self.game.update();

            gl_call!(gl::ClearColor(
                135.0 / 255.0,
                206.0 / 255.0,
                235.0 / 255.0,
                1.0
            ));
            gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));

            self.game.draw_grid_with(|x, y, color| {
                if color.is_none() {
                    return;
                }

                let color = color.unwrap();

                let pos_y = (self.game.grid_height as f32 - y - 1.0) * self.sector_height;

                let model = glm::scale(&identity::<f32, 4>(), &glm::vec3(0.2, 0.2, 0.2));

                let model = glm::rotate(
                    &model,
                    x * sector_angle - PI * 0.5,
                    &glm::vec3(0.0, 1.0, 0.0),
                );

                let model = glm::translate(&model, &glm::vec3(0.0, pos_y, 0.0));

                let final_mat = projection * self.camera.view * model;

                gl_call!(gl::UniformMatrix4fv(
                    u_final_mat,
                    1,
                    gl::FALSE,
                    final_mat.as_ptr()
                ));

                gl_call!(gl::Uniform4f(u_color, color.x, color.y, color.z, color.w));

                gl_call!(gl::DrawElements(
                    gl::TRIANGLES,
                    36,
                    gl::UNSIGNED_INT,
                    std::ptr::null()
                ));
            });

            self.window.swap_buffers();
        }
    }

    fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    gl_call!(gl::Viewport(0, 0, width, height));

                    self.screen_width = width as u32;
                    self.screen_height = height as u32;
                }
                WindowEvent::Key(key, _, action, _) => {
                    if key == Key::Down {
                        match action {
                            Action::Repeat => {}
                            _ => self.game.toggle_piece_drop(action == Action::Press),
                        }
                    }

                    if action == Action::Release {
                        break;
                    }

                    match key {
                        Key::Escape => self.window.set_should_close(true),
                        Key::F1 => {
                            self.window.set_cursor_mode(
                                if self.window.get_cursor_mode() == CursorMode::Disabled {
                                    CursorMode::Normal
                                } else {
                                    CursorMode::Disabled
                                },
                            );

                            self.cursor_disabled = !self.cursor_disabled;
                            self.camera.handle_mouse = false;
                        }
                        Key::Left => self.game.move_piece(-1),
                        Key::Right => self.game.move_piece(1),
                        Key::Space => self.game.hard_drop_piece(),
                        Key::Z if action == Action::Press => self.game.rotate_piece(),
                        _ => {}
                    }
                }
                WindowEvent::CursorPos(x, y) if self.cursor_disabled => {
                    self.camera.look_at(glm::vec2(x as f32, y as f32))
                }
                _ => {}
            }
        }
    }

    fn create_shape(&self, inner_radius: f32, outer_radius: f32) -> (Vec<glm::Vec3>, &[GLuint]) {
        const INDICES: [GLuint; 36] = [
            0, 1, 2, 4, 5, 6, 1, 2, 3, 5, 6, 7, 0, 4, 2, 1, 3, 7, 4, 2, 6, 1, 7, 5, 0, 1, 4, 2, 3,
            6, 1, 4, 5, 3, 6, 7,
        ];

        let sector_angle = 2.0 * PI / (self.game.grid_width as f32);

        let angle_vec = glm::vec3(sector_angle.cos(), 0.0, sector_angle.sin());

        let mut vertices = vec![
            glm::vec3(outer_radius, 0.0, 0.0),
            angle_vec * outer_radius,
            glm::vec3(inner_radius, 0.0, 0.0),
            angle_vec * inner_radius,
        ];

        let size = vertices.len();

        vertices.reserve(size * 2);

        for i in 0..size {
            let mut vec = vertices[i];

            vec.y = self.sector_height;

            vertices.push(vec);
        }

        (vertices, &INDICES)
    }
}
