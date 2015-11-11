extern crate clock_ticks;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate png;
extern crate search;
extern crate sfml;

use frame_counter::{ FrameCounter, FrameUpdate };
use search::graph::{ BFSSearch, GraphSearch };
use search::{ map, MapField };
use sfml::graphics::{RenderWindow, Color, Shape, RenderTarget};
use sfml::system::Vector2f;
use sfml::traits::ShapeImpl;
use sfml::window::keyboard::Key;
use sfml::window::{VideoMode, ContextSettings, event, Close};
use std::path::Path;
use std::rc::Rc;

mod frame_counter;

const DEFAULT_SCALE_FACTOR: usize = 4;

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

    let mut image = map::to_image_buffer(&map, DEFAULT_SCALE_FACTOR);
    let mut search = search_method(map.clone(), shape);
    let mut fc = FrameCounter::from_fps(20);
    let (w, h) = image.dimensions();
    let mut app = AppState { pause: false,
                             scale_factor: DEFAULT_SCALE_FACTOR,
                             exit: false,
                             window: create_window(w, h) };

    'exit: while app.window.is_open() {
        for e in app.window.events() {
            app.process_input_event(&e)
        }
        if app.exit { break 'exit; }
        else if app.pause { continue }
        else if let FrameUpdate::NewFrame{elapsed_frames: fs, elapsed_ns: ns} = fc.update() {
            println!("new frame: ms={:?} skipped={:?}",
                     ns / 1_000_000,
                     fs - 1);
            app.window.clear(&Color::black());
            search.step();
            //app.window.draw(&search);
            app.window.display();
        }
    }
}



struct AppState {
    pause: bool,
    scale_factor: usize,
    exit: bool,
    window: RenderWindow
}

impl AppState {

    fn process_input_event(&mut self, e: &event::Event) {
        match e {
            &event::Closed => self.window.close(),
            &event::KeyPressed{code, ..} => match code {
                Key::Escape => {
                    self.window.close();
                    self.exit = true;
                },
                _ => {}
            },
            _ => {}
        }
    }

}

fn create_window(width: u32, height: u32) -> RenderWindow {
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = match RenderWindow::new(VideoMode::new_init(width, height, 32),
                                                           "SFML Shape Example", Close, &setting)
    {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);
    window
}
