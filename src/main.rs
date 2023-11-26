extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, events) = glfw.create_window(
        1280, 
        720,
        "Rust Minecraft | by xdAbi", 
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();
    }
}
