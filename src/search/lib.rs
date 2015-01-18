#[macro_use] extern crate log;
extern crate png;

use map::{ Map, Position };
use std::collections::{ BinaryHeap, HashMap, HashSet };
use std::hash::{ Hash, Hasher };
use std::num::Float;

pub mod map;

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

#[derive(Clone)]
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
        DirectionIter { dir: Direction::N }
    }
}

struct DirectionIter { dir: Direction }

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        match self.dir {
            Direction::N  => { self.dir = Direction::NE; Some (Direction::NE) },
            Direction::NE => { self.dir = Direction::E;  Some (Direction::E)  },
            Direction::E  => { self.dir = Direction::SE; Some (Direction::SE) },
            Direction::SE => { self.dir = Direction::S;  Some (Direction::S)  },
            Direction::S  => { self.dir = Direction::SW; Some (Direction::SW) },
            Direction::SW => { self.dir = Direction::W;  Some (Direction::W)  },
            Direction::W  => { self.dir = Direction::NW; Some (Direction::NW) }
            Direction::NW => None
        }
    }
}

fn get_move_function(shape: WorldShape)
        -> fn(Position, Direction, &SearchMap) -> Option<Position> {
    match shape {
        WorldShape::Rectangle => move_in_rectangle,
        //Torus => |(x,y)| {

        //}
    }
}

fn move_in_rectangle((x,y): Position, d: Direction, m: &SearchMap)
        -> Option<Position> {
    match d {
        Direction::N  => if y > 0 { Some ((x, y-1)) } else { None },
        Direction::NE => if y > 0 && x < m.width-1 { Some ((x+1, y-1)) } else { None },
        Direction::E  => if x < m.width-1 { Some ((x+1, y)) } else { None },
        Direction::SE => if y < m.height-1 && x < m.width-1 { Some ((x+1, y+1)) } else { None },
        Direction::S  => if y < m.height-1 { Some ((x, y+1)) } else { None },
        Direction::SW => if y < m.height-1 && x > 0 { Some ((x-1, y+1)) } else { None },
        Direction::W  => if x > 0 { Some ((x-1, y)) } else { None },
        Direction::NW => if y > 0 && x > 0 { Some ((x-1, y-1)) } else { None }
    }
}

#[test]
fn test_move_in_rectangle() {
    let mv = get_move_function(WorldShape::Rectangle);
    let (w,h) = (10,10);
    let map = SearchMap { width: w, height: h, fields: vec!() };
    // top-left
    assert_eq!(None, mv((0,0), Direction::NW, &map));
    assert_eq!(None, mv((0,0), Direction::N, &map));
    assert_eq!(None, mv((0,0), Direction::W, &map));
    // top-right
    assert_eq!(None, mv((9,0), Direction::NE, &map));
    assert_eq!(None, mv((9,0), Direction::N, &map));
    assert_eq!(None, mv((9,0), Direction::E, &map));
    // bottom-right
    assert_eq!(None, mv((9,9), Direction::SE, &map));
    assert_eq!(None, mv((9,9), Direction::S, &map));
    assert_eq!(None, mv((9,9), Direction::E, &map));
    // bottom-left
    assert_eq!(None, mv((0,9), Direction::SW, &map));
    assert_eq!(None, mv((0,9), Direction::S, &map));
    assert_eq!(None, mv((0,9), Direction::W, &map));
    // top
    for x in range(0,w)
        { assert_eq!(None, mv((x,0), Direction::N, &map)) }
    // bottom
    for x in range(0,w)
        { assert_eq!(None, mv((x,h-1), Direction::S, &map)) }
    // left
    for y in range(0,h)
        { assert_eq!(None, mv((0,y), Direction::W, &map)) }
    // right
    for y in range(0,h)
        { assert_eq!(None, mv((w-1,y), Direction::E, &map)) }
    // middle
    for x in range(1,w-1) {
        for y in range(1,h-1) {
            for d in Direction::iter() {
                assert!(mv((x,y), d, &map).is_some())
            }
        }
    }
}

#[derive(Show)]
pub enum Error {
    GoalUnreachable
}

pub type Result = std::result::Result<Search, Error>;

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
           initial_map: &map::Map, world_shape: WorldShape) -> Result {
    let map = &SearchMap::from_map(initial_map);
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mv = get_move_function(world_shape);
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
                                paths: vec![Path { fields: path }],
                                visited: visited.into_iter().collect() })
        }
        let rated_moves: Vec<(isize, Position)> = Direction::iter()
            .map(|d| mv(pos, d, map))
            .map(|maybe_new_pos| match maybe_new_pos {
                None => None,
                Some (new_pos) =>
                    if !map.is_allowed(new_pos) { None }
                    else { Some ((distance(new_pos, vgoals[0]), new_pos)) }
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
    Err (Error::GoalUnreachable)
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
                                paths: vec![Path { fields: path }],
                                visited: visited.into_iter().collect() })
        }
        let moves: Vec<(isize, Position)> = Direction::iter()
            .map(|d| mv(pos, d, map))
            .map(|maybe_new_pos| match maybe_new_pos {
                None => None,
                Some (new_pos) =>
                    if !map.is_allowed(new_pos) { None }
                    else { Some ((- distance(new_pos, vgoals[0]), new_pos)) }
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

pub fn astar(start: Vec<Position>, vgoals: Vec<Position>,
             initial_map: &map::Map, world_shape: WorldShape) -> Result {
    let map = &SearchMap::from_map(initial_map);
    assert_eq!(1, start.len());
    assert_eq!(1, vgoals.len());
    let mv = get_move_function(world_shape);
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
                                paths: vec![Path { fields: path }],
                                visited: visited.into_iter().collect() })
        }
        let moves: Vec<Position> = Direction::iter()
            .map(|d| mv(pos, d, map))
            .filter_map(|maybe_new_pos| match maybe_new_pos {
                Some (new_pos) if map.is_allowed(new_pos) => Some (new_pos),
                _ => None
            }).collect();
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

pub fn save(map: &Map, search: &Search, dest: String) {
    let mut img = map::to_png(map);
    map::png::draw_visited(&search.visited, &mut img);
    let path = search.paths[0].clone();
    map::png::draw_path(path, &mut img);
    map::png::draw_start(&search.start, &mut img);
    map::png::draw_goals(&search.goals, &mut img);
    map::png::write_image(&mut img, dest.as_slice());
}
