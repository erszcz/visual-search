extern crate clock_ticks;
extern crate env_logger;
extern crate graphics;
extern crate image;
extern crate input;
#[macro_use] extern crate log;
extern crate opengl_graphics;
extern crate piston;
extern crate png;
extern crate rustc_serialize;
extern crate sdl2_window;
extern crate search;
extern crate shader_version;

use frame_counter::{ FrameCounter, FrameUpdate };
use graphics::{ clear };
use opengl_graphics::{ GlGraphics, Texture };
use piston::event_loop::Events;
use piston::input::{ Button, Key, PressEvent, RenderEvent };
use piston::window::{ WindowSettings };
use sdl2_window::{ OpenGL, Sdl2Window };
use std::path::Path;

use search::{ map, MapField };
use search::graph::{ BFSSearch, GraphSearch };

mod frame_counter;

fn main() {
    env_logger::init().unwrap();
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2
        { panic!("expected MAP") }
    let ref arg_map = args[1];

    let img = png::load_png(&Path::new(arg_map)).unwrap();
    let map = map::from_png(&img);
    let shape = search::WorldShape::Torus{ width: map.width, height: map.height };
    let search_method = search::bfs
      as fn(search::map::Map, search::WorldShape) -> BFSSearch<MapField>;

    let mut scale_factor = 4;
    let mut image = map::to_image_buffer(&map, scale_factor);
    let mut search = search_method(map.clone(), shape);

    let mut fc = FrameCounter::from_fps(20);
    let opengl = OpenGL::V3_2;
    let (width, height) = image.dimensions();
    let window: Sdl2Window = WindowSettings::new("Graph Search".to_string(),
                                                 [width, height])
                                            .exit_on_esc(true)
                                            .build().unwrap();
    let mut texture = Texture::from_image(&image);
    let ref mut gl = GlGraphics::new(opengl);
    let mut pause = false;
    for e in window.events() {
        if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
            pause = !pause;
            println!("pause: {}", pause);
        };
        if let Some(Button::Keyboard(Key::Equals)) = e.press_args() {
            scale_factor *= 2;
            println!("scale_factor: {}", scale_factor);
        };
        if let Some(Button::Keyboard(Key::Minus)) = e.press_args() {
            scale_factor /= 2;
            if scale_factor == 0 { scale_factor = 1 }
            println!("scale_factor: {}", scale_factor);
        };
        if let Some(Button::Keyboard(Key::Q)) = e.press_args() {
            println!("exit");
            break
        };
        if let Some(args) = e.render_args() {
            if pause
                { continue }
            if let FrameUpdate::NewFrame{elapsed_frames: fs,
                                         elapsed_ns: ns} = fc.update() {
                println!("new frame: ms={:?} skipped={:?}",
                         ns / 1_000_000,
                         fs - 1);
                search.step();
                gl.draw(args.viewport(), |c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    image = map::to_image_buffer(&map, scale_factor);
                    texture = Texture::from_image(&image);
                    graphics::image(&texture, c.transform, g);
                });
            }
        };
    }
}
