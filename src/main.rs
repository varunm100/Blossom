#[macro_use]
extern crate log;

mod v;
use v::{Context, RenderGraph};

use glfw::{Action, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(1280, 720, "Blossom", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let c = Context::create(&window);
    let gpu = c.get_highest_vram_gpu();
    let rg = RenderGraph::create();
    rg.draw(&c, &[gpu]);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape | Key::Q, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {}
    }
}
