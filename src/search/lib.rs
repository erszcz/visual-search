extern crate png;

pub enum Field {
    Normal,
    Impassable
}

pub struct Map {
    pub width: uint,
    pub height: uint,
    pub fields: Vec<Vec<Field>>
}

pub type Position = (uint, uint);

pub struct Path {
    pub fields: Vec<Position>
}

pub fn bfs(start: Vec<Position>, goals: Vec<Position>, map: Map) -> Path {
    Path { fields: start }
}
