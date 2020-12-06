extern crate png;

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

#[derive(Debug)]
pub enum Pixels {
    RGB8(Vec<u8>),
    //RGBA8(Vec<u8>)
}

#[derive(Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Pixels,
}

pub fn load(source: &str) -> Map {
    let image = load_image(source);
    let w = image.width as usize;
    let h = image.height as usize;
    let Pixels::RGB8(ref pixels) = image.pixels;
    Map {
        width: w,
        height: h,
        fields: pixels_to_fields(pixels, w, h, 3)
    }
}

pub fn load_image(source: &str) -> Image {
    let decoder = png::Decoder::new(std::fs::File::open(source).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();
    Image {
        width: info.width,
        height: info.height,
        pixels: Pixels::RGB8(buf)
    }
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
            pixels: Pixels::RGB8(pixels) }
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
    let pixel_width: u8 = 3;
    //match img.pixels {
    //    Pixels::RGB8(_) => 3,
    //    //Pixels::RGBA8(_) => 4,
    //    _ => panic!("only RGB8 and RGBA8 modes are supported")
    //};
    match img.pixels {
        //Pixels::RGB8(ref mut pixels) |
        //Pixels::RGBA8(ref mut pixels) => {
        Pixels::RGB8(ref mut pixels) => {
            for i in (0 .. pixel_width) {
                pixels[index(pos, img.width as usize, pixel_width) + i as usize] =
                    color_by_width(color, pixel_width, i)
            }
        }
        //_ => panic!("only RGB8 and RGBA8 modes are supported")
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


pub fn write_image(img: &mut Image, dest: &str) -> () {
    let path = std::path::Path::new(dest);
    let file = std::fs::File::create(path).unwrap();
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, img.width, img.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let Pixels::RGB8(ref data) = img.pixels;
    writer.write_image_data(&data).unwrap();
}
