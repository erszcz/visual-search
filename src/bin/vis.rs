#![allow(unstable)]
#![feature(plugin)]
#[plugin] #[no_link] extern crate docopt_macros;

extern crate "rustc-serialize" as rustc_serialize;
extern crate clock_ticks;
extern crate docopt;
extern crate event;
extern crate graphics;
extern crate image;
extern crate input;
extern crate opengl_graphics;
extern crate png;
extern crate sdl2_window;
extern crate search;
extern crate shader_version;

use frame_counter::{ FrameCounter, FrameUpdate };
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;
use search::map;
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

docopt!{Args derive Show, "
Usage: vis [-m METHOD] [-w WORLD] <map>
       vis --help

Options:
  -m METHOD         Search method: bfs, greedy or astar.
  -w WORLD          World to search in: torus or rectangle.
  -h, --help        Show this message.
"}

fn main() {
    let cmdline: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    let img = png::load_png(&Path::new(cmdline.arg_map)).unwrap();
    let map = map::from_png(&img);
    let shape = match cmdline.flag_w.as_slice() {
        "torus" =>
            search::WorldShape::Torus{ width: map.width, height: map.height },
        "rectangle" | _ =>
            search::WorldShape::Rectangle{ width: map.width, height: map.height },
    };
    let search_method = match cmdline.flag_m.as_slice() {
        _ => search::bfs2 as fn(&search::map::Map, search::WorldShape) -> search::BFSState
    };

    let scale_factor = 3;
    let mut image = map::to_image_buffer(&map, scale_factor);
    let mut search = search_method(&map, shape);

    let mut fc = FrameCounter::from_fps(20);
    let opengl = shader_version::OpenGL::_3_2;
    let (width, height) = image.dimensions();
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
    let mut texture = Texture::from_image(&image);
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in event::events(&window) {
        use event::{ RenderEvent };
        if let Some(args) = e.render_args() {
            if let FrameUpdate::NewFrame{skipped_frames, ..} = fc.update() {
                errorln!("new frame: skipped={:?}", skipped_frames);
                search.next();
                gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                    graphics::clear([0.0; 4], gl);
                    image = map::to_image_buffer(&search.map.to_map(), scale_factor);
                    texture = Texture::from_image(&image);
                    graphics::image(&texture, &c, gl);
                });
            }
        };
    }
}
