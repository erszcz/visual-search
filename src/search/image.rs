extern crate png;

pub type ColorRGB8 = (u8,u8,u8);

pub const WHITE: ColorRGB8 = (255,255,255);
pub const GRAY : ColorRGB8 = (128,128,128);
pub const BLACK: ColorRGB8 = (  0,  0,  0);
pub const RED  : ColorRGB8 = (255,  0,  0);
pub const GREEN: ColorRGB8 = (  0,255,  0);
pub const BLUE : ColorRGB8 = (  0,  0,255);

pub fn draw_points(points: Vec<(uint,uint)>, color: ColorRGB8,
                   img: &mut png::Image) {
    for point in points.iter()
        { putpixel(*point, color, img) }
}

fn putpixel(pos: (uint,uint), (r,g,b): ColorRGB8, img: &mut png::Image) {
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
