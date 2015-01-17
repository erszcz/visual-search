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

fn putpixel(pos: (usize,usize), (r,g,b): ColorRGB8, img: &mut png::Image) {
    let w = img.width as usize;
    match img.pixels {
        K8(ref mut pixels) => pixels[index(pos, w, 1)] = (r+g+b) / 3,
        KA8(ref mut pixels) => { pixels[index(pos, w, 2)    ] = (r+g+b) / 3;
                                 pixels[index(pos, w, 2) + 1] = 255u8 },
        RGB8(ref mut pixels) => { pixels[index(pos, w, 3)    ] = r;
                                  pixels[index(pos, w, 3) + 1] = g;
                                  pixels[index(pos, w, 3) + 2] = b },
        RGBA8(ref mut pixels) => { pixels[index(pos, w, 4)    ] = r;
                                   pixels[index(pos, w, 4) + 1] = g;
                                   pixels[index(pos, w, 4) + 2] = b;
                                   pixels[index(pos, w, 4) + 3] = 255u8 }
    }
}

fn index((x,y): (usize,usize), width: usize, bytes_per_color: usize) -> usize {
    y * width * bytes_per_color + x * bytes_per_color
}
