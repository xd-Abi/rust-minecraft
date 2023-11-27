extern crate glfw;

use glfw::{Glfw, PWindow, Context};

pub struct App {
    glfw: Glfw,
    window: PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl App {
    pub fn new() -> Self {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut window, events) = glfw.create_window(
            1280,
            720, 
            "Rust Minecraft | by xdAbi", 
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window!");

        window.make_current();
        window.set_key_polling(true);

        Self {
            glfw: glfw,
            window: window,
            events: events,
        }
    }

    pub fn run(mut self) {
        while !self.window.should_close() {
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }
}