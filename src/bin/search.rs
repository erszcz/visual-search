#![allow(unstable)]
#![feature(plugin)]
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
#[plugin] #[no_link] extern crate docopt_macros;
extern crate png;

extern crate search;

use search::map;
use std::io;

macro_rules! errorln {
    ($fmt:expr) => {
        (writeln![&mut io::stdio::stderr(), $fmt]).ok().expect("log failed")
    };
    ($fmt:expr, $($msg:tt)*) => {
        (writeln![&mut io::stdio::stderr(), $fmt, $($msg)*]).ok().expect("log failed")
    };
}

docopt!{Args derive Show, "
Usage: search [-m METHOD] <src> <dst>
       search --help

Options:
  -m METHOD         Search method: bfs, greedy or astar.
  -h, --help        Show this message.
"}

fn main() {
    let cmdline: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let img = png::load_png(&Path::new(cmdline.arg_src)).unwrap();
    let map = map::from_png(&img);
    let start = map.start();
    let goals = map.goals();
    let search_result = match cmdline.flag_m.as_slice() {
        "greedy" =>
            search::greedy(start.clone(), goals.clone(), &map,
                           search::WorldShape::Rectangle{ width: map.width,
                                                          height: map.height }),
        "astar" =>
            search::astar(start.clone(), goals.clone(), &map,
                          search::WorldShape::Rectangle{ width: map.width,
                                                         height: map.height }),
        "bfs" | _ =>
            search::bfs(start.clone(), goals.clone(), &map,
                        search::WorldShape::Rectangle{ width: map.width,
                                                       height: map.height })
    };
    match search_result {
        Err (e) => errorln!("error: {:?}", e),
        Ok (search) => search::save(&map, &search, cmdline.arg_dst).unwrap()
    }
}
