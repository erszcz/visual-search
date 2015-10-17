extern crate image;

use super::{ Field, Map };

pub type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn map_to_image_buffer(map: &Map, scale_factor: usize) -> Image {
    let w = (map.width * scale_factor) as u32;
    let h = (map.height * scale_factor) as u32;
    let f = |x, y| {
        let pos = ((x / scale_factor as u32) as usize,
        (y / scale_factor as u32) as usize);
        let pixel = field_to_pixel(map[pos]);
        image::Rgba (pixel)
    };
    image::ImageBuffer::from_fn(w, h, f)
}

fn field_to_pixel(field: Field) -> [u8; 4] {
    match field {
        Field::Start      => [    0, 255u8,     0, 255u8],
        Field::Goal       => [255u8,     0,     0, 255u8],
        Field::Path       => [255u8, 255u8, 255u8, 255u8],
        Field::Current    => [220u8,  10u8, 100u8, 255u8],
        Field::Passable   => [    0,     0,     0, 255u8],
        Field::Impassable => [    0,     0, 255u8, 255u8],
        Field::Visited    => [128u8, 128u8, 128u8, 255u8],
        Field::Frontier   => [255u8, 255u8,     0, 255u8],
    }
}
