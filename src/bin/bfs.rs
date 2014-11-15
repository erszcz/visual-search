extern crate png;

pub fn write_image() {
    let mut img = png::Image {
        width: 10,
        height: 10,
        pixels: png::RGB8(Vec::from_elem(10 * 10 * 3, 255u8)),
    };
    let res = png::store_png(&mut img, &Path::new("test/store.png"));
    assert!(res.is_ok());
}

fn main() {
    write_image();
}
