extern crate png;

use image;
use png::{Image};

pub struct Map {
    pub width: uint,
    pub height: uint,
    pub fields: Vec<Field>
}

#[deriving(Clone)]
pub enum Field {
    Normal,
    Impassable
}

pub fn from_png(img: &Image) -> Map {
    let fields = match img.pixels {
        png::RGB8(ref pixels) => rgbpixels_to_fields(pixels, img.width as uint,
                                                     img.height as uint, 3),
        png::RGBA8(ref pixels) => rgbpixels_to_fields(pixels, img.width as uint,
                                                      img.height as uint, 4),
        png::K8(_) => panic_mode("K8"),
        png::KA8(_) => panic_mode("KA8"),
    };
    Map { width: img.width as uint, height: img.height as uint,
          fields: fields }
}

fn rgbpixels_to_fields(pixels: &Vec<u8>, width: uint, height: uint,
                       bytes_per_pixel: uint) -> Vec<Field> {
    let mut fields = Vec::from_elem(width * height, Field::Normal);
    for i in range(0, width * height) {
        let j = i * bytes_per_pixel;
        let color: ColorRGB8 = (pixels[j], pixels[j+1], pixels[j+2]);
        fields[i] = color_to_field(color);
    }
    fields
}

fn panic_mode(mode: &str) -> ! {
    panic!("only RGB8 mode is supported: found {}", mode)
}

type ColorRGB8 = (u8, u8, u8);

fn color_to_field((r,g,b): ColorRGB8) -> Field {
    match (r,g,b) {
        image::BLUE => Impassable,
        _ => Normal
    }
}
