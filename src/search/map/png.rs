extern crate png;

use self::png::Image;
use self::png::PixelsByColorType::{ RGB8, RGBA8 };
use super::{ Field, Map, Position };
use std::iter::repeat;
use std::path::Path;

pub type ColorRGB8 = (u8,u8,u8);

pub const WHITE: ColorRGB8 = (255,255,255);
pub const GRAY : ColorRGB8 = (128,128,128);
pub const BLACK: ColorRGB8 = (  0,  0,  0);
pub const RED  : ColorRGB8 = (255,  0,  0);
pub const GREEN: ColorRGB8 = (  0,255,  0);
pub const BLUE : ColorRGB8 = (  0,  0,255);

pub fn map_from_png(img: &Image) -> Map {
    let fields = match img.pixels {
        RGB8(ref pixels) => pixels_to_fields(pixels, img.width as usize,
                                             img.height as usize, 3),
        RGBA8(ref pixels) => pixels_to_fields(pixels, img.width as usize,
                                              img.height as usize, 4),
        _ => panic!("only RGB8 and RGBA8 modes are supported")
    };
    Map { width: img.width as usize,
          height: img.height as usize,
          fields: fields }
}

pub fn map_to_png(map: &Map) -> Image {
    let mut pixels: Vec<u8> = Vec::with_capacity(3 * map.width * map.height);
    for f in map.fields.iter() {
        // TODO: for now this is enough, but later let's get rid of collections
        // in SearchResult and enable more clauses here;
        // even better - extract to field_to_pixel
        if let Field::Impassable = *f
            { pixels.extend(vec![0u8, 0u8, 255u8].into_iter()) }
        else
            { pixels.extend(vec![0u8, 0u8, 0u8].into_iter()) }
    }
    Image { width: map.width as u32,
            height: map.height as u32,
            pixels: RGB8 (pixels) }
}

fn pixels_to_fields(pixels: &Vec<u8>, width: usize, height: usize,
                    bytes_per_pixel: usize) -> Vec<Field> {
    let mut fields: Vec<Field> =
        repeat(Field::Passable).take(width * height).collect();
    for i in (0 .. width * height) {
        let j = i * bytes_per_pixel;
        let color: ColorRGB8 = (pixels[j], pixels[j+1], pixels[j+2]);
        fields[i] = pixel_to_field(color);
    }
    fields
}

fn pixel_to_field((r,g,b): ColorRGB8) -> Field {
    match (r,g,b) {
        RED => Field::Goal,
        GREEN => Field::Start,
        BLUE => Field::Impassable,
        _ => Field::Passable
    }
}

pub fn draw_points(points: &Vec<Position>, color: ColorRGB8,
                   img: &mut Image) {
    for point in points.iter()
        { putpixel(*point, color, img) }
}

fn putpixel(pos: (usize,usize), color: ColorRGB8, img: &mut Image) {
    let pixel_width: u8 = match img.pixels {
        RGB8(_) => 3,
        RGBA8(_) => 4,
        _ => panic!("only RGB8 and RGBA8 modes are supported")
    };
    match img.pixels {
        RGB8(ref mut pixels) |
        RGBA8(ref mut pixels) => {
            for i in (0 .. pixel_width) {
                pixels[index(pos, img.width as usize, pixel_width) + i as usize] =
                    color_by_width(color, pixel_width, i)
            }
        }
        _ => panic!("only RGB8 and RGBA8 modes are supported")
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

pub fn write_image(img: &mut png::Image, dst: &str) -> Result<(), String> {
    png::store_png(img, &Path::new(dst))
}
