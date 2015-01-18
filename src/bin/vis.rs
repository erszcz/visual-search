extern crate shader_version;
extern crate input;
extern crate event;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
    // TODO: determine based on input map
    let (width, height) = (300, 300);
    let window = Sdl2Window::new(
        opengl,
        event::WindowSettings {
            title: "Graph Search".to_string(),
            size: [width, height],
            fullscreen: true,
            exit_on_esc: true,
            samples: 0
        }
    );
    let window = RefCell::new(window);

    for _e in event::events(&window) {
    }
}
