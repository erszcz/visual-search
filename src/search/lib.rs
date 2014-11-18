#![feature(phase)]

#[phase(plugin, link)] extern crate log;
extern crate png;

use map::Position;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::num::Float;

pub mod image;
pub mod map;

struct SearchMap {
    width: uint,
    height: uint,
    fields: Vec<map::Field>
}

impl SearchMap {
    fn is_allowed(&self, pos: Position) -> bool {
        match self.fields[map::index(pos, self.width)] {
            map::Impassable => false,
            _ => true
        }
    }

    fn from_map(map: &map::Map) -> SearchMap {
        SearchMap { width: map.width,
                    height: map.height,
                    // TODO: introduce SearchField, convert fields to SearchFields
                    fields: map.fields.clone() }
    }
}

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
        -> fn(Position, Direction, &SearchMap) -> Option<Position> {
    match shape {
        Rectangle => move_in_rectangle,
        //Torus => |(x,y)| {

        //}
    }
}

fn move_in_rectangle((x,y): Position, d: Direction, m: &SearchMap)
        -> Option<Position> {
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
    let map = SearchMap { width: w, height: h, fields: vec!() };
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

fn distance((x1,y1): Position, (x2,y2): Position) -> int {
    let (fx1, fy1, fx2, fy2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);
    ( (fx2-fx1) * (fx2-fx1) + (fy2-fy1) * (fy2-fy1) ).sqrt() as int
}

pub fn bfs(start: Vec<Position>, vgoals: Vec<Position>,
           initial_map: &map::Map, world_shape: WorldShape) -> Result {
    let map = &SearchMap::from_map(initial_map);
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mv = get_move_function(world_shape);
    let mut q = start.clone();
    let goals = vec_to_set(vgoals.clone());
    let mut visited = vec_to_set(start);
    let mut steps = HashMap::new();
    loop { match q.remove(0) {
        None => break,
        Some (pos) => {
            debug!("visited: {}", visited);
            debug!("current: {}", pos);
            debug!("steps  : {}", steps);
            if goals.contains(&pos) {
                let path = reconstruct_path(pos, &steps);
                return Ok (Path { fields: path })
            }
            let rated_moves: Vec<(int, Position)> = Direction::iter()
                .map(|d| mv(pos, d, map))
                .map(|maybe_new_pos| match maybe_new_pos {
                    None => None,
                    Some (new_pos) =>
                        if !map.is_allowed(new_pos) { None }
                        else { Some ((distance(new_pos, vgoals[0]), new_pos)) }
                })
                .filter(|new_pos| new_pos.is_some()).map(|new_pos| new_pos.unwrap())
                .collect();
            let heap = BinaryHeap::from_vec(rated_moves);
            let sorted_moves = heap.into_sorted_vec();
            for &(_, new_pos) in sorted_moves.iter() {
                if !visited.contains(&new_pos) {
                    q.push(new_pos);
                    visited.insert(new_pos);
                    steps.insert(new_pos, pos);
                }
            }
        }
    }}
    Err (GoalUnreachable)
}

pub fn greedy(start: Vec<Position>, vgoals: Vec<Position>,
              initial_map: &map::Map, world_shape: WorldShape) -> Result {
    let map = &SearchMap::from_map(initial_map);
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mv = get_move_function(world_shape);
    let mut pq = BinaryHeap::new();
    pq.push( ( - distance(start[0], vgoals[0]), start[0] ) );
    let goals = vec_to_set(vgoals.clone());
    let mut visited = vec_to_set(start);
    let mut steps = HashMap::new();
    loop {
        let (_, pos) = match pq.pop() {
            None => break,
            Some (pos) => pos
        };
        debug!("visited: {}", visited);
        debug!("current: {}", pos);
        debug!("steps  : {}", steps);
        if goals.contains(&pos) {
            let path = reconstruct_path(pos, &steps);
            return Ok (Path { fields: path })
        }
        let moves: Vec<(int, Position)> = Direction::iter()
            .map(|d| mv(pos, d, map))
            .map(|maybe_new_pos| match maybe_new_pos {
                None => None,
                Some (new_pos) =>
                    if !map.is_allowed(new_pos) { None }
                    else { Some ((- distance(new_pos, vgoals[0]), new_pos)) }
            })
            .filter(|new_pos| new_pos.is_some()).map(|new_pos| new_pos.unwrap())
            .collect();
        for &(cost, new_pos) in moves.iter() {
            if !visited.contains(&new_pos) {
                pq.push((cost, new_pos));
                visited.insert(new_pos);
                steps.insert(new_pos, pos);
            }
        }
    }
    Err (GoalUnreachable)
}

#[inline]
fn vec_to_set<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
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
