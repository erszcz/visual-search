#![allow(unstable)]
#[macro_use] extern crate log;
extern crate png;

use map::{ Map, Position };
use std::collections::{ BinaryHeap, HashMap, HashSet };
use std::num::Float;
use std::ops::Add;

pub mod map;

// TODO: integrate with map::Map
struct SearchMap {
    width: usize,
    height: usize,
    fields: Vec<map::Field>
}

impl SearchMap {
    fn is_allowed(&self, pos: Position) -> bool {
        match self.fields[map::index(pos, self.width)] {
            map::Field::Impassable => false,
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

pub type Path = Vec<Position>;

#[derive(Copy)]
pub enum WorldShape {
    Rectangle { width: usize, height: usize },
    Torus { width: usize, height: usize }
}

#[derive(Copy, FromPrimitive)]
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
        std::num::FromPrimitive::from_u8(d)
    }

    fn displacement(self) -> (isize, isize) {
        match self {
            Direction::N  => ( 0is,-1is),
            Direction::NE => ( 1is,-1is),
            Direction::E  => ( 1is, 0is),
            Direction::SE => ( 1is, 1is),
            Direction::S  => ( 0is, 1is),
            Direction::SW => (-1is, 1is),
            Direction::W  => (-1is, 0is),
            Direction::NW => (-1is,-1is)
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

#[cfg(test)]
fn rectangle(w: usize, h: usize) -> WorldShape {
    WorldShape::Rectangle { width: w, height: h }
}

#[cfg(test)]
fn torus(w: usize, h: usize) -> WorldShape {
    WorldShape::Torus { width: w, height: h }
}

#[test]
fn moves_in_a_rectangle() {
    // (x,y): top-left is (0,0), top-right is (1,0).
    // o.
    // ..
    assert_eq!(vec![(1,0), (1,1), (0,1)],
               moves((0,0), rectangle(2, 2)));
    // ..
    // o.
    assert_eq!(vec![(0,0), (1,0), (1,1)],
               moves((0,1), rectangle(2, 2)));
    // ..
    // .o
    assert_eq!(vec![(1,0), (0,1), (0,0)],
               moves((1,1), rectangle(2, 2)));
    // .o
    // ..
    assert_eq!(vec![(1,1), (0,1), (0,0)],
               moves((1,0), rectangle(2, 2)));
    // Numbers are indices into the example positions vector.
    // 7 0 1
    // 6 o 2
    // 5 4 3
    assert_eq!(vec![(1,0), (2,0),
                           (2,1),
                           (2,2), (1,2), (0,2),
                                         (0,1),
                                         (0,0)],
               moves((1,1), rectangle(3, 3)));
    // Dots are unreachable in 1 step.
    // . 0 1
    // . o 2
    // . 4 3
    assert_eq!(vec![(0,0), (1,0),
                           (1,1),
                           (1,2), (0,2)],
               moves((0,1), rectangle(3, 3)));
    // . 4 0
    // . 3 o
    // . 2 1
    assert_eq!(vec![(2,0),
                    (2,2), (1,2),
                           (1,1),
                           (1,0)],
               moves((2,1), rectangle(3, 3)));
    // 4 o 0
    // 3 2 1
    // . . .
    assert_eq!(vec![(2,0),
                    (2,1), (1,1), (0,1),
                                  (0,0)],
               moves((1,0), rectangle(3, 3)));
    // . . .
    // 4 0 1
    // 3 o 2
    assert_eq!(vec![(1,1), (2,1),
                           (2,2), (0,2),
                                  (0,1)],
               moves((1,2), rectangle(3, 3)));
}

#[test]
fn moves_in_a_torus() {
    // (x,y): top-left is (0,0), top-right is (1,0).
    // o.
    // ..
    assert_eq!(vec![(0,1), (1,1),
                           (1,0),
                           (1,1), (0,1), (1,1),
                                         (1,0),
                                         (1,1)],
               moves((0,0), torus(2, 2)));
    // ..
    // o.
    assert_eq!(vec![(0,0), (1,0),
                           (1,1),
                           (1,0), (0,0), (1,0),
                                         (1,1),
                                         (1,0)],
               moves((0,1), torus(2, 2)));
    // ..
    // .o
    assert_eq!(vec![(1,0), (0,0),
                           (0,1),
                           (0,0), (1,0), (0,0),
                                         (0,1),
                                         (0,0)],
               moves((1,1), torus(2, 2)));
    // .o
    // ..
    assert_eq!(vec![(1,1), (0,1),
                           (0,0),
                           (0,1), (1,1), (0,1),
                                         (0,0),
                                         (0,1)],
               moves((1,0), torus(2, 2)));
    // Numbers are indices into the example positions vector.
    // 7 0 1
    // 6 o 2
    // 5 4 3
    assert_eq!(vec![(1,0), (2,0),
                           (2,1),
                           (2,2), (1,2), (0,2),
                                         (0,1),
                                         (0,0)],
               moves((1,1), torus(3, 3)));
    // Dots are unreachable in 1 step.
    // 0 1 7
    // o 2 6
    // 4 3 5
    assert_eq!(vec![(0,0), (1,0),
                           (1,1),
                           (1,2), (0,2), (2,2),
                                         (2,1),
                                         (2,0)],
               moves((0,1), torus(3, 3)));
    // 1 7 0
    // 2 6 o
    // 3 5 4
    assert_eq!(vec![(2,0), (0,0),
                           (0,1),
                           (0,2), (2,2), (1,2),
                                         (1,1),
                                         (1,0)],
               moves((2,1), torus(3, 3)));
    // 6 o 2
    // 5 4 3
    // 7 0 1
    assert_eq!(vec![(1,2), (2,2),
                           (2,0),
                           (2,1), (1,1), (0,1),
                                         (0,0),
                                         (0,2)],
               moves((1,0), torus(3, 3)));
    // 5 4 3
    // 7 0 1
    // 6 o 2
    assert_eq!(vec![(1,1), (2,1),
                           (2,2),
                           (2,0), (1,0), (0,0),
                                         (0,2),
                                         (0,1)],
               moves((1,2), torus(3, 3)));
}

#[derive(Copy)]
struct WorldPosition {
    x: isize,
    y: isize,
    shape: WorldShape
}

impl WorldPosition {

    fn pos(&self) -> Position { (self.x as usize, self.y as usize) }

    fn moves(&self) -> WorldPositions {
        WorldPositions {
            wp: *self,
            directions: Direction::iter() }
    }

    // Return Some (adjusted_wp) within the WorldShape
    // or None if the position doesn't fit in the shape.
    fn shear_off(&self) -> Option<WorldPosition> {
        match self.shape {
            WorldShape::Torus {width, height} => {
                let (swidth, sheight) = (width as isize, height as isize);
                Some (WorldPosition { x: (self.x + swidth) % swidth,
                                      y: (self.y + sheight) % sheight,
                                      shape: self.shape })
            }
            WorldShape::Rectangle {width, height}
                if (self.x >= 0 && self.x < width as isize &&
                    self.y >= 0 && self.y < height as isize) =>
                Some (*self),
            _ =>
                None
        }
    }

}

impl Add<(isize, isize)> for WorldPosition {
    type Output = WorldPosition;

    fn add(self, (x,y): (isize, isize)) -> WorldPosition {
        WorldPosition { x: self.x + x,
                        y: self.y + y,
                        shape: self.shape }
    }
}

struct WorldPositions {
    wp: WorldPosition,
    directions: Directions
}

impl Iterator for WorldPositions {
    type Item = WorldPosition;

    fn next(&mut self) -> Option<WorldPosition> {
        match self.directions.next() {
            None => None,
            Some (dir) => {
                let new_wp = (self.wp + dir.displacement()).shear_off();
                if new_wp.is_some()
                    { new_wp } else { self.next() }
                }
            }
    }
}

fn moves((x,y): Position, shape: WorldShape) -> Vec<Position> {
    WorldPosition { x: x as isize,
                    y: y as isize,
                    shape: shape }.moves().map(|wp| wp.pos()).collect()
}

#[derive(Show)]
enum Error {
    GoalUnreachable
}

pub type SearchResult = std::result::Result<Search, Error>;

pub struct Search {
    start: Vec<Position>,
    goals: Vec<Position>,
    paths: Vec<Path>,
    visited: Vec<Position>
}

fn distance((x1,y1): Position, (x2,y2): Position) -> isize {
    let (fx1, fy1, fx2, fy2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);
    ( (fx2-fx1) * (fx2-fx1) + (fy2-fy1) * (fy2-fy1) ).sqrt() as isize
}

pub fn bfs(start: Vec<Position>, vgoals: Vec<Position>,
           initial_map: &map::Map, shape: WorldShape) -> SearchResult {
    let map = &SearchMap::from_map(initial_map);
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mut q = start.clone();
    let goals = vec_to_set(vgoals.clone());
    let mut visited = vec_to_set(start.clone());
    let mut steps = HashMap::new();
    loop {
        let pos = q.remove(0);
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
        let rated_moves: Vec<(isize, Position)> = moves(pos, shape).iter()
            .map(|new_pos| {
                if !map.is_allowed(*new_pos) { None }
                else { Some ((distance(*new_pos, vgoals[0]), *new_pos)) }
            }).filter_map(|new_pos| new_pos).collect();
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
}

pub fn greedy(start: Vec<Position>, vgoals: Vec<Position>,
              initial_map: &map::Map, shape: WorldShape) -> SearchResult {
    let map = &SearchMap::from_map(initial_map);
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mut pq = BinaryHeap::new();
    pq.push( ( - distance(start[0], vgoals[0]), start[0] ) );
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
        let moves: Vec<(isize, Position)> = moves(pos, shape).iter()
            .map(|new_pos| {
                if !map.is_allowed(*new_pos) { None }
                else { Some ((- distance(*new_pos, vgoals[0]), *new_pos)) }
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
             initial_map: &map::Map, shape: WorldShape) -> SearchResult {
    let map = &SearchMap::from_map(initial_map);
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let goals = vec_to_set(vgoals.clone());
    let mut visited = vec_to_set(start.clone());
    let mut steps = HashMap::new();
    let mut g_score = HashMap::new();
    let start0 = start[0].clone();
    g_score.insert(start0, 0);
    let mut f_score = HashMap::new();
    f_score.insert(start0, g_score[start0] + distance(start0, vgoals[0]));
    let mut pq = BinaryHeap::new();
    pq.push( ( - f_score[start0], start0 ) );
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
        let moves: Vec<Position> = moves(pos, shape).iter()
            .filter_map(|new_pos|
                        if map.is_allowed(*new_pos) { Some (*new_pos) }
                        else { None }).collect();
        for new_pos in moves.iter() {
            let tentative_g_score = g_score[pos] + 1;
            if (!visited.contains(new_pos)
                || (g_score.contains_key(new_pos)
                    && tentative_g_score < g_score[*new_pos])) {
                g_score.insert(*new_pos, tentative_g_score);
                f_score.insert(*new_pos, tentative_g_score + distance(*new_pos, vgoals[0]));
                pq.push((- f_score[*new_pos], *new_pos));
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

pub fn save(map: &Map, search: &Search, dest: String)
    -> std::result::Result<(), String>
{
    let mut img = map::to_png(map);
    map::png::draw_points(&search.visited, map::png::GRAY, &mut img);
    map::png::draw_points(&search.paths[0], map::png::WHITE, &mut img);
    map::png::draw_points(&search.start, map::png::GREEN, &mut img);
    map::png::draw_points(&search.goals, map::png::RED, &mut img);
    map::png::write_image(&mut img, dest.as_slice())
}
