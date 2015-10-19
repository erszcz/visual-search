extern crate env_logger;
#[macro_use] extern crate log;
extern crate png;
extern crate rustc_serialize;

extern crate search;

use search::{ GraphSearch, Search };
use search::map;
use std::path::Path;

#[derive(Debug)]
enum Method {
    BFS,
    BFS2
}

fn main() {
    env_logger::init().unwrap();
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 3
        { panic!("expected SRC and DST args") }
    let ref arg_src = args[1];
    let ref arg_dst = args[2];
    let img = png::load_png(&Path::new(arg_src)).unwrap();
    let map = map::from_png(&img);
    let method = std::env::var("SEARCH_METHOD")
        .map(|m| if m == "bfs"       { Method::BFS }
                 else if m == "bfs2" { Method::BFS2 }
                 else                { info!("unknown search method: {:?}", m);
                                       Method::BFS })
        .unwrap_or(Method::BFS);
    match do_search(&map, method) {
        Err (e) => panic!("error: {:?}", e),
        Ok (result) => search::save(&map, &result, arg_dst.clone()).unwrap()
    }
}

fn do_search(map: &search::map::Map, method: Method)
        -> Result<search::Search, search::Error> {
    let start = map.start();
    let goals = map.goals();
    let world_shape = search::WorldShape::Rectangle{ width: map.width,
                                                     height: map.height };
    info!("searching with {:?}", method);
    match method {
        Method::BFS  => search::bfs(start.clone(), goals.clone(), &map, world_shape),
        Method::BFS2 => do_bfs2(map.clone(), world_shape)
            .map(|path| Search { start: start,
                                 goals: goals,
                                 paths: vec![path],
                                 visited: vec![] })
    }
}

fn do_bfs2(map: search::map::Map, shape: search::WorldShape)
        -> Result<search::Path, search::Error> {
    let mut state = search::bfs2(map, shape);
    while let None = state.result {
        state.step();
    }
    state.result.expect("do_bfs2 error")
}
