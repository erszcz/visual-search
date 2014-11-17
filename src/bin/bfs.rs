#![feature(macro_rules, phase)]

#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate png;
extern crate search;
extern crate serialize;

use search::image::{mod, BLUE, GREEN, RED, WHITE};
use search::map::{mod, Field, Map};
use std::io;

macro_rules! stderr(($fmt:expr$(, $msg:expr)*) => {
    (writeln![io::stderr(), $fmt $(, $msg)*]).ok().expect("log failed")
})

docopt!(Args deriving Show, "
Usage: bfs <src> <dst>
       bfs --help

Options:
  -h, --help       Show this message.
")

fn main() {
    let cmdline: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let mut img = png::load_png(&Path::new(cmdline.arg_src)).unwrap();
    let start = vec![(1,1)];
    let goals = vec![(5,5)];
    let map = map::from_png(&img);
    match search::bfs(start.clone(), goals.clone(), &map,
                      search::WorldShape::Rectangle) {
        Err (e) => stderr!("error: {}", e),
        Ok (path) => {
            draw_path(path, &mut img);
            draw_start(start, &mut img);
            draw_goals(goals, &mut img);
            draw_map(&map, &mut img);
            write_image(&mut img, cmdline.arg_dst.as_slice());
        }
    }
}

fn draw_map(map: &Map, img: &mut png::Image) {
    let points: Vec<(uint,uint)> = range(0, map.width)
        .flat_map(|x| repeat(x).zip(range(0, map.height)))
        .filter(|&(x,y)| match map.fields[index((x,y), map.width, 1)] {
            Field::Normal => false,
            Field::Impassable => true
        }).collect();
    image::draw_points(points, BLUE, img);
}

fn draw_path(path: search::Path, img: &mut png::Image) {
    image::draw_points(path.fields, WHITE, img)
}

fn draw_start(start: Vec<search::Position>, img: &mut png::Image) {
    image::draw_points(start, GREEN, img)
}

fn draw_goals(goals: Vec<search::Position>, img: &mut png::Image) {
    image::draw_points(goals, RED, img)
}

pub fn write_image(img: &mut png::Image, dst: &str) {
    let res = png::store_png(img, &Path::new(dst));
    assert!(res.is_ok());
}

fn index((x,y): (uint,uint), width: uint, bytes_per_color: uint) -> uint {
    y * width * bytes_per_color + x * bytes_per_color
}
