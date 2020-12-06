#[macro_use] extern crate log;

use graph::{ BFSSearch, SearchNode };
use map::{ Field, Map, Position };
use std::collections::{ BinaryHeap, HashMap, HashSet };
use std::fmt::Debug;
use std::rc::Rc;

pub mod frame_counter;
pub mod graph;
pub mod map;

pub type Path = Vec<Position>;

#[derive(Clone, Copy)]
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
    fn iter() -> Directions {
        Directions { dir: Some (Direction::N) }
    }

    fn from_u8(d: u8) -> Option<Direction> {
        match d {
            0 => Some (Direction::N),
            1 => Some (Direction::NE),
            2 => Some (Direction::E),
            3 => Some (Direction::SE),
            4 => Some (Direction::S),
            5 => Some (Direction::SW),
            6 => Some (Direction::W),
            7 => Some (Direction::NW),
            _ => None
        }
    }

    fn displacement(self) -> (isize, isize) {
        match self {
            Direction::N  => ( 0, -1),
            Direction::NE => ( 1, -1),
            Direction::E  => ( 1,  0),
            Direction::SE => ( 1,  1),
            Direction::S  => ( 0,  1),
            Direction::SW => (-1,  1),
            Direction::W  => (-1,  0),
            Direction::NW => (-1, -1)
        }
    }
}

struct Directions { dir: Option<Direction> }

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        match self.dir {
            None => None,
            Some (dir) => {
                let this = self.dir;
                self.dir = Direction::from_u8(dir as u8 + 1);
                this
            }
        }
    }
}

#[test]
fn moves_test() {
    // (x,y): top-left is (0,0), top-right is (1,0).
    // o.
    // ..
    assert_eq!(vec![(1,0), (1,1), (0,1)],
               moves((0,0), (2, 2)));
    // ..
    // o.
    assert_eq!(vec![(0,0), (1,0), (1,1)],
               moves((0,1), (2, 2)));
    // ..
    // .o
    assert_eq!(vec![(1,0), (0,1), (0,0)],
               moves((1,1), (2, 2)));
    // .o
    // ..
    assert_eq!(vec![(1,1), (0,1), (0,0)],
               moves((1,0), (2, 2)));
    // Numbers are indices into the example positions vector.
    // 7 0 1
    // 6 o 2
    // 5 4 3
    assert_eq!(vec![(1,0), (2,0),
                           (2,1),
                           (2,2), (1,2), (0,2),
                                         (0,1),
                                         (0,0)],
               moves((1,1), (3, 3)));
    // Dots are unreachable in 1 step.
    // . 0 1
    // . o 2
    // . 4 3
    assert_eq!(vec![(0,0), (1,0),
                           (1,1),
                           (1,2), (0,2)],
               moves((0,1), (3, 3)));
    // . 4 0
    // . 3 o
    // . 2 1
    assert_eq!(vec![(2,0),
                    (2,2), (1,2),
                           (1,1),
                           (1,0)],
               moves((2,1), (3, 3)));
    // 4 o 0
    // 3 2 1
    // . . .
    assert_eq!(vec![(2,0),
                    (2,1), (1,1), (0,1),
                                  (0,0)],
               moves((1,0), (3, 3)));
    // . . .
    // 4 0 1
    // 3 o 2
    assert_eq!(vec![(1,1), (2,1),
                           (2,2), (0,2),
                                  (0,1)],
               moves((1,2), (3, 3)));
}

fn moves((px,py): Position, dimensions: (isize, isize)) -> Vec<Position> {
    let (x0,y0) = (px as isize, py as isize);
    Direction::iter()
              .filter_map(|dir| { let (dx, dy) = dir.displacement();
                                  crop((x0 + dx, y0 + dy), dimensions) })
              .collect()
}

fn crop((x0, y0): (isize, isize), (width, height): (isize, isize)) -> Option<Position> {
    if x0 >= 0 && x0 < width && y0 >= 0 && y0 < height {
        Some ((x0 as usize, y0 as usize))
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub enum Error {
    GoalUnreachable
}

pub type SearchResult = std::result::Result<Search, Error>;

pub struct Search {
    pub start: Vec<Position>,
    pub goals: Vec<Position>,
    pub paths: Vec<Path>,
    pub visited: Vec<Position>
}

fn distance((x1,y1): Position, (x2,y2): Position) -> isize {
    let (fx1, fy1, fx2, fy2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);
    let dx = fx2 - fx1;
    let dy = fy2 - fy1;
    (dx * dx + dy * dy).sqrt().round() as isize
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MapField {
    pub pos:    Position,
    map:        Rc<Map>
}

impl std::fmt::Debug for MapField {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str(&format!("{:?}", self.pos));
        Ok (())
    }
}

impl SearchNode for MapField {

    type Id = Position;

    fn id(&self) -> Position { self.pos }

    fn is_goal(&self) -> bool {
        self.map[self.pos] == Field::Goal
    }

    fn neighbours(&self) -> Vec<MapField> {
        moves(self.pos, self.map.isize_dimensions())
            .iter()
            .filter(|moved| self.map[**moved].is_passable())
            .map(|moved| MapField { map: self.map.clone(),
                                    pos: *moved })
            .collect()
    }

}

pub fn bfs<'a>(map: Map) -> BFSSearch<MapField> {
    let rc_map = Rc::new(map);
    let start: Vec<MapField> = rc_map.start()
        .iter()
        .map(|pos| MapField { pos: *pos,
                              map: rc_map.clone() })
        .collect();
    BFSSearch { result: None,
                frontier: start.clone(),
                visited: start.iter().map(|field| field.pos).collect(),
                steps: HashMap::new() }
}

fn appraise(pos: Position, goal: Position) -> (isize, Position) {
    ( -distance(pos, goal), pos )
}

pub fn greedy(start: Vec<Position>, vgoals: Vec<Position>,
              map: &Map) -> SearchResult {
    let map = map.clone();
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mut pq = BinaryHeap::new();
    pq.push( appraise(start[0], vgoals[0]) );
    let goals = vec_to_set(vgoals.clone());
    let mut visited = vec_to_set(start.clone());
    let mut steps = HashMap::new();
    loop {
        let (_, pos) = match pq.pop() {
            None => break,
            Some (pos) => pos
        };
        debug!("visited: {:?}", visited);
        debug!("current: {:?}", pos);
        debug!("steps  : {:?}", steps);
        if goals.contains(&pos) {
            let path = reconstruct_path(pos, &steps);
            return Ok (Search { start: start,
                                goals: vgoals,
                                paths: vec![path],
                                visited: visited.into_iter().collect() })
        }
        let moves: Vec<(isize, Position)> = moves(pos, map.isize_dimensions()).iter()
            .map(|new_pos| {
                if !map[*new_pos].is_passable() { None }
                else { Some (appraise(*new_pos, vgoals[0])) }
            }).filter_map(|new_pos| new_pos).collect();
        for &(cost, new_pos) in moves.iter() {
            if !visited.contains(&new_pos) {
                pq.push((cost, new_pos));
                visited.insert(new_pos);
                steps.insert(new_pos, pos);
            }
        }
    }
    Err (Error::GoalUnreachable)
}

#[allow(unused_parens)]
pub fn astar(start: Vec<Position>, vgoals: Vec<Position>,
             map: &Map) -> SearchResult {
    let map = map.clone();
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let goals = vec_to_set(vgoals.clone());
    let mut visited = vec_to_set(start.clone());
    let mut steps = HashMap::new();
    let mut g_score = HashMap::new();
    let start0 = start[0].clone();
    g_score.insert(start0, 0);
    let mut f_score = HashMap::new();
    f_score.insert(start0, g_score[&start0] + distance(start0, vgoals[0]));
    let mut pq = BinaryHeap::new();
    pq.push( ( - f_score[&start0], start0 ) );
    loop {
        let pos = match pq.pop() {
            None => break,
            Some ((_, pos)) => pos
        };
        debug!("visited: {:?}", visited);
        debug!("current: {:?}", pos);
        debug!("steps  : {:?}", steps);
        if goals.contains(&pos) {
            let path = reconstruct_path(pos, &steps);
            return Ok (Search { start: start,
                                goals: vgoals,
                                paths: vec![path],
                                visited: visited.into_iter().collect() })
        }
        let moves: Vec<Position> = moves(pos, map.isize_dimensions()).iter()
            .filter_map(|new_pos|
                        if map[*new_pos].is_passable() { Some (*new_pos) }
                        else { None }).collect();
        for new_pos in moves.iter() {
            let tentative_g_score = g_score[&pos] + 1;
            if (!visited.contains(new_pos)
                || (g_score.contains_key(new_pos)
                    && tentative_g_score < g_score[new_pos])) {
                g_score.insert(*new_pos, tentative_g_score);
                f_score.insert
                    (*new_pos, (tentative_g_score
                                + distance(*new_pos, vgoals[0])));
                pq.push((- f_score[new_pos], *new_pos));
                visited.insert(*new_pos);
                steps.insert(*new_pos, pos);
            }
        }
    }
    Err (Error::GoalUnreachable)
}

#[inline]
fn vec_to_set(v: Vec<Position>) -> HashSet<Position> {
    v.into_iter().collect()
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
