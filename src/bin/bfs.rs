extern crate png;
extern crate search;

fn draw_map(map: search::Map, img: &mut png::Image) {

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

fn draw_points(points: Vec<(uint,uint)>, color: Color, img: &mut png::Image) {
    for point in points.iter()
        { putpixel(*point, color, img) }
}

fn draw_path(path: search::Path, img: &mut png::Image) {
    draw_points(path.fields, WHITE, img)
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
    let path = search::bfs(start, goals.clone(),
                           search::Map { width: 0, height: 0, fields: vec!() });
    draw_path(path, &mut img);
    draw_goals(goals, &mut img);
    write_image(&mut img);
}
