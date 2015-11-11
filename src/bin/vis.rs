extern crate clock_ticks;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate png;
extern crate search;
extern crate sfml;

use frame_counter::{ FrameCounter, FrameUpdate };
use search::graph::{ BFSSearch, GraphSearch };
use search::{ map, MapField };
use sfml::graphics::{
    Color,
    PrimitiveType,
    rc,
    RenderStates,
    RenderTarget,
    RenderWindow,
    Shape,
    Vertex,
    VertexArray
};
use sfml::system::Vector2f;
use sfml::traits::{ Drawable, ShapeImpl };
use sfml::window::keyboard::Key;
use sfml::window::{ VideoMode, ContextSettings, event, Close };
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
    let search_method = search::bfs as fn(search::map::Map) -> BFSSearch<MapField>;

    let mut image = map::to_image_buffer(&map, DEFAULT_SCALE_FACTOR);
    let mut search = search_method(map.clone());
    let mut fc = FrameCounter::from_fps(20);
    let (w, h) = image.dimensions();
    let mut app = AppState { pause: false,
                             scale_factor: DEFAULT_SCALE_FACTOR,
                             exit: false,
                             window: create_window(w, h) };

    let mut snapshot = BFSSearchSnapshot::new((w, h));
    snapshot.update(&search);
    app.window.clear(&Color::black());

    'exit: while app.window.is_open() {
        for e in app.window.events() {
            app.process_input_event(&e)
        }
        if app.exit { break 'exit; }
        else if app.pause { continue }
        search.step();
        if let FrameUpdate::NewFrame{elapsed_frames: fs, elapsed_ns: ns} = fc.update() {
            println!("new frame: ms={:?} skipped={:?}",
                     ns / 1_000_000,
                     fs - 1);
            snapshot.update(&search);
            app.window.draw(&snapshot);
            app.window.display();
        }
    }
}

struct BFSSearchSnapshot {
    size: u32,
    vertices: VertexArray
}

impl BFSSearchSnapshot {

    fn new((width, height): (u32, u32)) -> BFSSearchSnapshot {
        let size = width * height;
        // allocate in one go
        let va = VertexArray::new_init(PrimitiveType::Points, size)
            .expect("can't allocate vertex array");
        BFSSearchSnapshot { size: size, vertices: va }
    }

    fn update(&mut self, search: &BFSSearch<MapField>) {
        // TODO: ok, this is fucking lame for now... but let's have something,
        //       and make it good later
        self.vertices.clear();
        for node in search.visited.iter() {
            self.vertices.append(&pos_to_vertex(*node, &Color::cyan()));
        }
    }

}

fn pos_to_vertex((x, y): (usize, usize), color: &Color) -> Vertex {
    let v2f = Vector2f::new(x as f32, y as f32);
    Vertex::new_with_pos_color(&v2f, color)
}

impl Drawable for BFSSearchSnapshot {

    fn draw<RT: RenderTarget>(&self, render_target: &mut RT) -> () {
        render_target.draw_vertex_array(&self.vertices)
    }

    fn draw_rs<RT: RenderTarget>(&self,
                                 render_target: &mut RT,
                                 render_states: &mut RenderStates) -> () {
        render_target.draw_vertex_array_rs(&self.vertices, render_states)
    }

    fn draw_rs_rc<RT: RenderTarget>(&self,
                                    render_target: &mut RT,
                                    render_states: &mut rc::RenderStates) -> () {
        render_target.draw_vertex_array_rs_rc(&self.vertices, render_states)
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
                Key::Equal => {
                    self.zoom(0.8);
                    println!("=");
                },
                Key::Dash => {
                    self.zoom(1.25);
                    println!("-");
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn zoom(&mut self, factor: f32) {
        let mut v = self.window.get_default_view();
        let center = v.get_center() * factor;
        v.set_center(&center);
        v.zoom(factor);
        self.window.set_view(&v);
        self.window.clear(&Color::black());
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
