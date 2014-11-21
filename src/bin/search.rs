#![feature(macro_rules, phase)]

#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate png;
extern crate search;
extern crate serialize;

use search::image::{mod, BLUE, GRAY, GREEN, RED, WHITE};
use search::map::{mod, Field, Map};
use std::io;

macro_rules! stderr(($fmt:expr$(, $msg:expr)*) => {
    (writeln![io::stderr(), $fmt $(, $msg)*]).ok().expect("log failed")
})

docopt!(Args deriving Show, "
Usage: search [-m METHOD] <src> <dst>
       search --help

Options:
  -m METHOD         Search method: bfs, greedy or astar.
  -h, --help        Show this message.
")

fn main() {
    let cmdline: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let mut img = png::load_png(&Path::new(cmdline.arg_src)).unwrap();
    let map = map::from_png(&img);
    let start = map.start();
    let goals = map.goals();
    let method = match cmdline.flag_m.as_slice() {
        "bfs" => search::bfs,
        "greedy" => search::greedy,
        "astar" => search::astar,
        _ => search::bfs
    };
    match method(start.clone(), goals.clone(), &map,
                      search::WorldShape::Rectangle) {
        Err (e) => stderr!("error: {}", e),
        Ok (search) => {
            draw_visited(search.visited, &mut img);
            let path = search.paths[0].clone();
            draw_path(path, &mut img);
            draw_start(search.start, &mut img);
            draw_goals(search.goals, &mut img);
            draw_map(&map, &mut img);
            write_image(&mut img, cmdline.arg_dst.as_slice());
        }
    }
}

fn draw_map(map: &Map, img: &mut png::Image) {
    let points: Vec<(uint,uint)> = map.positions()
        .filter(|&(x,y)| match map[(x,y)] {
            Field::Impassable => true,
            _ => false
        }).collect();
    image::draw_points(points, BLUE, img);
}


fn draw_visited(visited: Vec<search::map::Position>, img: &mut png::Image) {
    image::draw_points(visited, GRAY, img)
}

fn draw_path(path: search::Path, img: &mut png::Image) {
    image::draw_points(path.fields, WHITE, img)
}

fn draw_start(start: Vec<search::map::Position>, img: &mut png::Image) {
    image::draw_points(start, GREEN, img)
}

fn draw_goals(goals: Vec<search::map::Position>, img: &mut png::Image) {
    image::draw_points(goals, RED, img)
}

pub fn write_image(img: &mut png::Image, dst: &str) {
    let res = png::store_png(img, &Path::new(dst));
    assert!(res.is_ok());
}
