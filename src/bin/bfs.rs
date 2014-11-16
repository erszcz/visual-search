extern crate png;
extern crate search;

fn draw_map(map: &search::Map, img: &mut png::Image) {
    for x in range(0, map.width) {
        for y in range(0, map.height) {
            match map.fields[index((x,y), map.width, 1)] {
                search::Normal => (),
                search::Impassable => putpixel((x,y), BLUE, img)
            }
        }
    }
}

fn putpixel(pos: (uint,uint), (r,g,b): Color, img: &mut png::Image) {
    let w = img.width as uint;
    match img.pixels {
        png::K8(ref mut pixels) => pixels[index(pos, w, 1)] = (r+g+b) / 3,
        png::KA8(ref mut pixels) => { pixels[index(pos, w, 2)    ] = (r+g+b) / 3;
                                      pixels[index(pos, w, 2) + 1] = 255u8 },
        png::RGB8(ref mut pixels) => { pixels[index(pos, w, 3)    ] = r;
                                       pixels[index(pos, w, 3) + 1] = g;
                                       pixels[index(pos, w, 3) + 2] = b },
        png::RGBA8(ref mut pixels) => { pixels[index(pos, w, 4)    ] = r;
                                        pixels[index(pos, w, 4) + 1] = g;
                                        pixels[index(pos, w, 4) + 2] = b;
                                        pixels[index(pos, w, 4) + 3] = 255u8 }
    }
}

fn index((x,y): (uint,uint), width: uint, bytes_per_color: uint) -> uint {
    y * width * bytes_per_color + x * bytes_per_color
}

type Color = (u8,u8,u8);

const WHITE: (u8,u8,u8) = (255,255,255);
const BLACK: (u8,u8,u8) = (255,255,255);
const RED  : (u8,u8,u8) = (255,  0,  0);
const GREEN: (u8,u8,u8) = (  0,255,  0);
const BLUE : (u8,u8,u8) = (  0,  0,255);

fn draw_points(points: Vec<(uint,uint)>, color: Color, img: &mut png::Image) {
    for point in points.iter()
        { putpixel(*point, color, img) }
}

fn draw_path(path: search::Path, img: &mut png::Image) {
    draw_points(path.fields, WHITE, img)
}

fn draw_start(start: Vec<search::Position>, img: &mut png::Image) {
    draw_points(start, GREEN, img)
}

fn draw_goals(goals: Vec<search::Position>, img: &mut png::Image) {
    draw_points(goals, RED, img)
}

pub fn write_image(img: &mut png::Image) {
    let res = png::store_png(img, &Path::new("test/store.png"));
    assert!(res.is_ok());
}

fn main() {
    let mut img = png::Image {
        width: 10,
        height: 10,
        pixels: png::RGB8(Vec::from_elem(10 * 10 * 3, 0u8)),
    };
    let start = vec!((1,1));
    let goals = vec!((5,5));
    let map = {
        let mut fields = Vec::from_elem(10 * 10, search::Normal);
        fields[index((5,3), 10, 1)] = search::Impassable;
        fields[index((4,3), 10, 1)] = search::Impassable;
        fields[index((3,3), 10, 1)] = search::Impassable;
        fields[index((3,4), 10, 1)] = search::Impassable;
        fields[index((3,5), 10, 1)] = search::Impassable;
        search::Map { width: 10, height: 10, fields: fields }
    };
    match search::bfs(start.clone(), goals.clone(), &map,
                      search::WorldShape::Rectangle) {
        Err (e) => println!("error: {}", e),
        Ok (path) => {
            draw_path(path, &mut img);
            draw_start(start, &mut img);
            draw_goals(goals, &mut img);
            draw_map(&map, &mut img);
            write_image(&mut img);
        }
    }
}
