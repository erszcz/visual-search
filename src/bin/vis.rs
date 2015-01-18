extern crate shader_version;
extern crate input;
extern crate event;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate clock_ticks;

use std::cell::RefCell;
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;

fn main() {
    let start = ( clock_ticks::precise_time_ns() / 1000 / 1000 ) as usize;
    println!("start  : {}ms", start);
    std::io::timer::sleep(std::time::Duration::milliseconds(100));
    let end = ( clock_ticks::precise_time_ns() / 1000 / 1000 ) as usize;
    println!("end    : {}ms", end);
    println!("elapsed: {}ms", end - start);
}
