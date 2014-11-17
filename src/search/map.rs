extern crate png;

use image;
use png::Image;

pub type Position = (uint, uint);

pub struct Map {
    pub width: uint,
    pub height: uint,
    pub fields: Vec<Field>
}

#[deriving(Clone)]
pub enum Field {
    Start,
    Goal,
    Normal,
    Impassable
}

impl Map {

    pub fn start(&self) -> Vec<Position> {
        self.positions()
            .filter(|&(x,y)| match self[(x,y)] {
                Start => true,
                _ => false
            }).collect()
    }

    pub fn goals(&self) -> Vec<Position> {
        self.positions()
            .filter(|&(x,y)| match self[(x,y)] {
                Goal => true,
                _ => false
            }).collect()
    }

    pub fn positions(&self) -> MapPositions {
        MapPositions { x: 0, y: 0, width: self.width,
                       size: self.width * self.height }
    }

}

pub struct MapPositions { x: uint, y: uint, width: uint, size: uint }

impl Iterator<Position> for MapPositions {
    fn next(&mut self) -> Option<Position> {
        let xy = (self.x, self.y);
        if index(xy, self.width) >= self.size { None }
        else {
            if self.x < self.width-1 { self.x += 1 }
            else {
                self.x = 0;
                self.y += 1;
            }
            Some (xy)
        }
    }
}

impl Index<Position, Field> for Map {
    fn index<'a>(&'a self, pos: &Position) -> &'a Field {
        &self.fields[index(*pos, self.width)]
    }
}

#[test]
fn test_map_positions() {
    let m1 = Map { width: 1, height: 1, fields: vec![] };
    assert_eq!(vec![(0,0)], m1.positions().collect());
    let m2 = Map { width: 3, height: 2, fields: vec![] };
    assert_eq!(vec![(0,0),(1,0),(2,0),
                    (0,1),(1,1),(2,1)], m2.positions().collect());
    let m3 = Map { width: 2, height: 3, fields: vec![] };
    assert_eq!(vec![(0,0),(1,0),
                    (0,1),(1,1),
                    (0,2),(1,2)], m3.positions().collect());
}

#[inline]
pub fn index((x,y): (uint,uint), width: uint) -> uint { y * width + x }

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
        image::RED => Goal,
        image::GREEN => Start,
        image::BLUE => Impassable,
        _ => Normal
    }
}
