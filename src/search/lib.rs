extern crate png;

use std::collections::{HashMap, HashSet};

pub enum Field {
    Normal,
    Impassable
}

pub struct Map {
    pub width: uint,
    pub height: uint,
    pub fields: Vec<Field>
}

pub type Position = (uint, uint);

pub struct Path {
    pub fields: Vec<Position>
}

pub enum WorldShape {
    Rectangle,
    //Torus
}

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Direction {
    fn iter() -> DirectionIter {
        DirectionIter { dir: N }
    }
}

struct DirectionIter { dir: Direction }

impl Iterator<Direction> for DirectionIter {
    fn next(&mut self) -> Option<Direction> {
        match self.dir {
            N  => { self.dir = NE; Some (NE) },
            NE => { self.dir = E;  Some (E)  },
            E  => { self.dir = SE; Some (SE) },
            SE => { self.dir = S;  Some (S)  },
            S  => { self.dir = SW; Some (SW) },
            SW => { self.dir = W;  Some (W)  },
            W  => { self.dir = NW; Some (NW) }
            NW => None
        }
    }
}

fn get_move_function(shape: WorldShape)
        -> fn(Position, Direction, &Map) -> Option<Position> {
    match shape {
        Rectangle => move_in_rectangle,
        //Torus => |(x,y)| {

        //}
    }
}

fn move_in_rectangle((x,y): Position, d: Direction, m: &Map) -> Option<Position> {
    match d {
        N  => if y > 0 { Some ((x, y-1)) } else { None },
        NE => if y > 0 && x < m.width-1 { Some ((x+1, y-1)) } else { None },
        E  => if x < m.width-1 { Some ((x+1, y)) } else { None },
        SE => if y < m.height-1 && x < m.width-1 { Some ((x+1, y+1)) } else { None },
        S  => if y < m.height-1 { Some ((x, y+1)) } else { None },
        SW => if y < m.height-1 && x > 0 { Some ((x-1, y+1)) } else { None },
        W  => if x > 0 { Some ((x-1, y)) } else { None },
        NW => if y > 0 && x > 0 { Some ((x-1, y-1)) } else { None }
    }
}

#[test]
fn test_move_in_rectangle() {
    let mv = get_move_function(WorldShape::Rectangle);
    let (w,h) = (10,10);
    let map = Map { width: w, height: h, fields: vec!() };
    // top-left
    assert_eq!(None, mv((0,0), NW, &map));
    assert_eq!(None, mv((0,0), N, &map));
    assert_eq!(None, mv((0,0), W, &map));
    // top-right
    assert_eq!(None, mv((9,0), NE, &map));
    assert_eq!(None, mv((9,0), N, &map));
    assert_eq!(None, mv((9,0), E, &map));
    // bottom-right
    assert_eq!(None, mv((9,9), SE, &map));
    assert_eq!(None, mv((9,9), S, &map));
    assert_eq!(None, mv((9,9), E, &map));
    // bottom-left
    assert_eq!(None, mv((0,9), SW, &map));
    assert_eq!(None, mv((0,9), S, &map));
    assert_eq!(None, mv((0,9), W, &map));
    // top
    for x in range(0,w)
        { assert_eq!(None, mv((x,0), N, &map)) }
    // bottom
    for x in range(0,w)
        { assert_eq!(None, mv((x,h-1), S, &map)) }
    // left
    for y in range(0,h)
        { assert_eq!(None, mv((0,y), W, &map)) }
    // right
    for y in range(0,h)
        { assert_eq!(None, mv((w-1,y), E, &map)) }
    // middle
    for x in range(1,w-1) {
        for y in range(1,h-1) {
            for d in Direction::iter() {
                assert!(mv((x,y), d, &map).is_some())
            }
        }
    }
}

#[deriving(Show)]
pub enum Error {
    GoalUnreachable
}

pub type Result = std::result::Result<Path, Error>;

pub fn bfs(start: Vec<Position>, vgoals: Vec<Position>,
           map: &Map, world_shape: WorldShape) -> Result {
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mv = get_move_function(world_shape);
    let mut q = start.clone();
    let goals = {
        let mut goals = HashSet::new();
        for goal in vgoals.iter()
            { goals.insert(*goal); }
        goals
    };
    let mut visited = HashSet::new();
    for pos in start.iter()
        { visited.insert(pos.clone()); }
    let mut steps = HashMap::new();
    loop { match q.pop() {
        None => break,
        Some (pos) => {
            println!("visited: {}", visited);
            println!("current: {}", pos);
            println!("steps  : {}", steps);
            println!("");
            if goals.contains(&pos) {
                let path = reconstruct_path(pos, &steps);
                return Ok (Path { fields: path })
            }
            for dir in Direction::iter() {
                match mv(pos, dir, map) {
                    None => (),
                    Some (new_pos) if !visited.contains(&new_pos) => {
                        q.push(new_pos);
                        visited.insert(new_pos);
                        steps.insert(new_pos, pos);
                    },
                    _ => ()
                }
            }
        }
    }}
    Err (GoalUnreachable)
}

fn reconstruct_path(goal: Position, steps: &HashMap<Position, Position>)
        -> Vec<Position> {
    let mut path = vec!();
    let mut last = goal;
    loop {
        path.push(last);
        match steps.get(&last) {
            None => break,
            Some (step) => last = *step
        }
    }
    path
}
