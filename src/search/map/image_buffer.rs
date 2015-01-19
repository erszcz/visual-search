extern crate image;

use super::{ Field, Map };

pub type Image = image::ImageBuffer<Vec<u8>, u8, image::Rgba<u8>>;

pub fn map_to_image_buffer(map: &Map) -> Image {
    image::ImageBuffer::from_fn(map.width as u32, map.height as u32,
                                Box::new(|x, y| {
                                    let pos = (x as usize, y as usize);
                                    let pixel = field_to_pixel(map[pos]);
                                    image::Rgba (pixel)
                                }))
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
