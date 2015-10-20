use std::ops::{ Index, IndexMut };

pub use self::image_buffer::map_to_image_buffer as to_image_buffer;
pub use self::png::map_from_png as from_png;
pub use self::png::map_to_png as to_png;

pub mod image_buffer;
pub mod png;

pub type Position = (usize, usize);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub fields: Vec<Field>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Field {
    Start,
    Goal,
    Path,
    Current,
    Passable,
    Impassable,
    Visited,
    Frontier
}

impl Field {
    pub fn is_passable(&self) -> bool {
        match self {
            &Field::Impassable => false,
            _ => true
        }
    }
}

impl Map {

    pub fn start(&self) -> Vec<Position> {
        self.positions()
            .filter(|&(x,y)| match self[(x,y)] {
                Field::Start => true,
                _ => false
            }).collect()
    }

    //#[deprecated = "remove once all search methods read goals directly from the map"]
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

#[derive(Clone, Copy)]
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

    fn index<'a>(&'a self, pos: Position) -> &'a Field {
        &self.fields[index(pos, self.width)]
    }
}

impl IndexMut<Position> for Map {
    fn index_mut<'a>(&'a mut self, pos: Position) -> &'a mut Field {
        &mut self.fields[index(pos, self.width)]
    }
}

#[inline]
pub fn index((x,y): (usize,usize), width: usize) -> usize { y * width + x }

#[test]
fn test_map_positions() {
    let m1 = Map { width: 1, height: 1, fields: vec![] };
    assert_eq!(vec![(0,0)], m1.positions().collect::<Vec<Position>>());
    let m2 = Map { width: 3, height: 2, fields: vec![] };
    assert_eq!(vec![(0,0),(1,0),(2,0),
                    (0,1),(1,1),(2,1)], m2.positions().collect::<Vec<Position>>());
    let m3 = Map { width: 2, height: 3, fields: vec![] };
    assert_eq!(vec![(0,0),(1,0),
                    (0,1),(1,1),
                    (0,2),(1,2)], m3.positions().collect::<Vec<Position>>());
}
