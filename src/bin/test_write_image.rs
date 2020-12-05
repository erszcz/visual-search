extern crate search;

use search::map;

fn main() {
    let data = vec![255, 0, 0,   0, 255,   0,   0,   0, 255,
                      0, 0, 0, 127, 127, 127, 255, 255, 255];
    let mut image = map::png::Image {
        width: 3,
        height: 2,
        pixels: map::png::Pixels::RGB8(data)
    };
    map::png::write_image(&mut image, "/Users/erszcz/work/erszcz/visual-search/test.png");
}
