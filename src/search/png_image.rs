extern crate png;

use png::PixelsByColorType::{K8, KA8, RGB8, RGBA8};

pub type ColorRGB8 = (u8,u8,u8);

pub const WHITE: ColorRGB8 = (255,255,255);
pub const GRAY : ColorRGB8 = (128,128,128);
pub const BLACK: ColorRGB8 = (  0,  0,  0);
pub const RED  : ColorRGB8 = (255,  0,  0);
pub const GREEN: ColorRGB8 = (  0,255,  0);
pub const BLUE : ColorRGB8 = (  0,  0,255);

pub fn draw_points(points: Vec<(usize,usize)>, color: ColorRGB8,
                   img: &mut png::Image) {
    for point in points.iter()
        { putpixel(*point, color, img) }
}

fn putpixel(pos: (usize,usize), color: ColorRGB8, img: &mut png::Image) {
    let pixel_width: u8 = match img.pixels {
        K8(_) => 1,
        KA8(_) => 2,
        RGB8(_) => 3,
        RGBA8(_) => 4
    };
    match img.pixels {
          K8(ref mut pixels)
        | KA8(ref mut pixels)
        | RGB8(ref mut pixels)
        | RGBA8(ref mut pixels) => {
            for i in range(0, pixel_width) {
                pixels[index(pos, img.width as usize, pixel_width) + i as usize] =
                    color_by_width(color, pixel_width, i)
            }
        }
    }
}

fn color_by_width((r,g,b): ColorRGB8, pixel_width: u8, channel_index: u8) -> u8 {
    match (pixel_width, channel_index) {
        // grayscale
        (1, _) => (r+g+b) / 3,
        // grayscale with alpha channel
        (2, 0) => (r+g+b) / 3,
        (2, 1) => 255u8,
        // RGB
        (3, 0) => r,
        (3, 1) => g,
        (3, 2) => b,
        // RGB with alpha channel
        (4, 0) => r,
        (4, 1) => g,
        (4, 2) => b,
        (4, 3) => 255u8,
        (_, _) => panic!("invalid pixel_width / channel_index combination")
    }
}

fn index((x,y): (usize,usize), width: usize, bytes_per_color: u8) -> usize {
    y * width * bytes_per_color as usize + x * bytes_per_color as usize
}
