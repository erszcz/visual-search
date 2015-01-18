#![allow(unstable)]

extern crate shader_version;
extern crate input;
extern crate event;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate clock_ticks;

use frame_counter::{ FrameCounter, FrameUpdate };
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;
use std::cell::RefCell;
use std::io;

macro_rules! errorln {
    ($fmt:expr) => {
        (writeln![&mut io::stdio::stderr(), $fmt]).ok().expect("log failed")
    };
    ($fmt:expr, $($msg:tt)*) => {
        (writeln![&mut io::stdio::stderr(), $fmt, $($msg)*]).ok().expect("log failed")
    };
}

mod frame_counter;

fn main() {
    let mut fc = FrameCounter::from_fps(2);
    let opengl = shader_version::OpenGL::_3_2;
    // TODO: determine based on input map
    let (width, height) = (300, 300);
    let window = Sdl2Window::new(
        opengl,
        event::WindowSettings {
            title: "Graph Search".to_string(),
            size: [width, height],
            //fullscreen: true,
            fullscreen: false,
            exit_on_esc: true,
            samples: 0
        }
    );
    let image = image::ImageBuffer::new(width, height);
    let mut frame = 0;
    let texture = Texture::from_image(&image);
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in event::events(&window) {
        use event::{ RenderEvent };
        if let Some(args) = e.render_args() {
            let color = match frame {
                0 => [1.0, 0.0, 0.0, 1.0],
                1 => [0.0, 1.0, 0.0, 1.0],
                2 => [0.0, 0.0, 1.0, 1.0],
                _ => [1.0, 1.0, 0.0, 1.0]
            };
            if let FrameUpdate::NewFrame{skipped_frames, ..} = fc.update() {
                errorln!("new frame: skipped={:?}", skipped_frames);
                frame = (frame + skipped_frames) % 4;
            }
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear(color, gl);
                graphics::image(&texture, &c, gl);
            });
        };
    }
}
