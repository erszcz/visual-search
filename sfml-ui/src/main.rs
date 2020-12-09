extern crate clock_ticks;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate png;
extern crate search;
extern crate sfml;

mod frame_counter;
mod image_buffer;

use frame_counter::{ FrameCounter, FrameUpdate };
use search::graph::{ BFSSearch, GraphSearch };
use search::{ map, MapField };
use sfml::graphics::{
    Color,
    Drawable,
    PrimitiveType,
    RenderStates,
    RenderTarget,
    RenderWindow,
    Vertex,
    VertexArray,
    View
};
use sfml::system::Vector2f;
use sfml::window::{ Event, Key, VideoMode, ContextSettings };

const DEFAULT_SCALE_FACTOR: usize = 4;

fn main() {
    env_logger::init();
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2
        { panic!("expected MAP") }
    let ref arg_map = args[1];

    let map = map::png::load(arg_map);
    let search_method = search::bfs as fn(search::map::Map) -> BFSSearch<MapField>;

    let image = image_buffer::from_map(&map, DEFAULT_SCALE_FACTOR);
    let search = search_method(map.clone());
    let mut fc = FrameCounter::from_fps(20);
    let (w, h) = image.dimensions();
    let mut app = AppState {
        pause: false,
        single_step: false,
        search: search,
        saved_search: None,
        window: create_window(w, h)
    };

    let mut snapshot = BFSSearchSnapshot::new((w, h));
    snapshot.update(&app.search);
    app.window.clear(Color::BLACK);
    app.save();

    while app.window.is_open() {
        while let Some(ref e) = app.window.poll_event() {
            app.process_input_event(e)
        }
        if !app.pause {
            app.search.step();
        } else if app.single_step {
            app.search.step();
            app.single_step = false;
            app.pause = true;
        }
        if let FrameUpdate::NewFrame{elapsed_frames: fs, elapsed_ns: ns} = fc.update() {
            info!(target: "tick", "new frame: ms={:?} skipped={:?}", ns / 1_000_000, fs - 1);
            snapshot.update(&app.search);
            app.window.clear(Color::BLACK);
            app.window.draw(&snapshot);
            app.window.display();
        }
    }
}

struct BFSSearchSnapshot {
    vertices: VertexArray
}

impl BFSSearchSnapshot {

    fn new((width, height): (u32, u32)) -> BFSSearchSnapshot {
        let size = width * height;
        // allocate in one go
        let va = VertexArray::new(PrimitiveType::Points, size as usize);
        BFSSearchSnapshot { vertices: va }
    }

    fn update(&mut self, search: &BFSSearch<MapField>) {
        // TODO: try not to redraw the whole buffer each frame
        self.vertices.clear();
        for node in search.visited.iter() {
            self.vertices.append(&pos_to_vertex(*node, Color::CYAN));
        }
        for field in search.frontier.iter() {
            self.vertices.append(&pos_to_vertex(field.pos, Color::RED));
        }
        if let Some (Ok (ref path)) = search.result {
            for node in path.iter() {
                self.vertices.append(&pos_to_vertex(*node, Color::BLUE));
            }
        }
    }

}

fn pos_to_vertex((x, y): (usize, usize), color: Color) -> Vertex {
    let v2f = Vector2f::new(x as f32, y as f32);
    Vertex::with_pos_color(v2f, color)
}

impl Drawable for BFSSearchSnapshot {


    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>)
    {
        target.draw_vertex_array(&self.vertices, states)
    }

}

struct AppState {
    pause: bool,
    single_step: bool,
    search: BFSSearch<MapField>,
    window: RenderWindow,
    saved_search: Option<BFSSearch<MapField>>
}

impl AppState {

    fn process_input_event(&mut self, e: &Event) {
        info!(target: "events", "event: {:?}", e);
        match e {
            &Event::Closed => self.window.close(),
            &Event::TextEntered{unicode, ..} => match unicode {
                '=' => self.zoom(0.8),
                '-' => self.zoom(1.25),
                ___ => info!(target: "events", "unhandled text entered: {:?}", unicode)
            }
            &Event::KeyPressed{code, ..} => match code {
                Key::Escape => self.window.close(),
                Key::Space  => self.pause = !self.pause,
                Key::Right  => self.single_step = true,
                Key::S      => self.save(),
                Key::R      => self.restore(),
                _           => info!(target: "events", "unhandled key pressed: {:?}", code)
            },
            _ => {}
        }
    }

    fn zoom(&mut self, factor: f32) {
        let default_view = self.window.view();
        let mut zoomed_view = View::new(
            default_view.center() * factor,
            default_view.size()
        );
        zoomed_view.zoom(factor);
        self.window.set_view(&zoomed_view);
        info!(target: "events", "zoom by {:?}", factor);
    }

    fn save(&mut self) {
        self.saved_search = Some (self.search.clone());
        info!(target: "events", "saved search state");
    }

    fn restore(&mut self) {
        if let Some (ref saved) = self.saved_search {
            self.search = saved.clone();
            self.window.clear(Color::BLACK);
            info!(target: "events", "restored search state");
        } else {
            info!(target: "events", "no saved search state!");
        }
    }

}

fn create_window(width: u32, height: u32) -> RenderWindow {
    let settings: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = RenderWindow::new(VideoMode::new(width, height, 32),
                                                     "SFML Shape Example",
                                                     sfml::window::Style::CLOSE,
                                                     &settings);
    window.set_vertical_sync_enabled(true);
    window
}
