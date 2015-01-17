extern crate png;

use image;
use png::Image;
use png::PixelsByColorType::{K8, KA8, RGB8, RGBA8};
use std::ops::Index;

pub type Position = (usize, usize);

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub fields: Vec<Field>
}

#[derive(Clone)]
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
                Field::Start => true,
                _ => false
            }).collect()
    }

    pub fn goals(&self) -> Vec<Position> {
        self.positions()
            .filter(|&(x,y)| match self[(x,y)] {
                Field::Goal => true,
                _ => false
            }).collect()
    }

    pub fn positions(&self) -> MapPositions {
        MapPositions { x: 0, y: 0, width: self.width,
                       size: self.width * self.height }
    }

}

pub struct MapPositions { x: usize, y: usize, width: usize, size: usize }

impl Iterator for MapPositions {
    type Item = Position;

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

impl Index<Position> for Map {
    type Output = Field;

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
pub fn index((x,y): (usize,usize), width: usize) -> usize { y * width + x }

pub fn from_png(img: &Image) -> Map {
    let fields = match img.pixels {
        RGB8(ref pixels) => rgbpixels_to_fields(pixels, img.width as usize,
                                                img.height as usize, 3),
        RGBA8(ref pixels) => rgbpixels_to_fields(pixels, img.width as usize,
                                                 img.height as usize, 4),
        K8(_) => panic_mode("K8"),
        KA8(_) => panic_mode("KA8"),
    };
    Map { width: img.width as usize, height: img.height as usize,
          fields: fields }
}

fn rgbpixels_to_fields(pixels: &Vec<u8>, width: usize, height: usize,
                       bytes_per_pixel: usize) -> Vec<Field> {
    let mut fields: Vec<Field> =
        range(0, width * height).map(|_| Field::Normal).collect();
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
        image::RED => Field::Goal,
        image::GREEN => Field::Start,
        image::BLUE => Field::Impassable,
        _ => Field::Normal
    }
}
