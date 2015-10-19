extern crate env_logger;
#[macro_use] extern crate log;
extern crate png;
extern crate rustc_serialize;

extern crate search;

use search::map;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 3
        { panic!("expected SRC and DST args") }
    let ref arg_src = args[1];
    let ref arg_dst = args[2];
    let img = png::load_png(&Path::new(arg_src)).unwrap();
    let map = map::from_png(&img);
    let start = map.start();
    let goals = map.goals();
    let world_shape = search::WorldShape::Torus{ width: map.width, height: map.height };
    let search_result = search::bfs(start.clone(), goals.clone(), &map, world_shape);
    match search_result {
        Err (e) => panic!("error: {:?}", e),
        Ok (search) => search::save(&map, &search, arg_dst.clone()).unwrap()
    }
}
