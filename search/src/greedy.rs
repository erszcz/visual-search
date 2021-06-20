use crate::graph::*;
use std::collections::{ HashMap, HashSet };

#[derive(Clone)]
pub struct GreedySearch<V: SearchNode> {
    pub result: SearchState<V>,
    pub visited: HashSet<V::Id>,
    pub steps: HashMap<V::Id, V::Id>

    let map = map.clone();
    let mut pq = BinaryHeap::new();
    pq.push( appraise(start[0], vgoals[0]) );
    let goals = vec_to_set(vgoals.clone());
    let mut visited = vec_to_set(start.clone());
    let mut steps = HashMap::new();
}

impl<V: SearchNode> GraphSearch<Node2d> for GreedySearch<V> {

    fn step(&mut self) {
        if self.result.is_over() {
            return
        }
        if self.frontier.is_empty() {
            self.result = SearchState::Failed("goal unreachable".to_string());
            return
        }
        let current = self.frontier.remove(0);
        debug!(target: "bfs", "visited: {:?}", self.visited);
        debug!(target: "bfs", "current: {:?}", current.id());
        debug!(target: "bfs", "steps  : {:?}", self.steps);
        if current.is_goal() {
            debug!(target: "bfs", "goal found: {:?}", current.id());
            let path = build_path::<V>(&self.steps, current.id());
            self.result = SearchState::Finished(path);
            return
        }
        let neighbours = current.neighbours();
        let n_ids: Vec<V::Id> = neighbours.iter().map(|n| n.id()).collect();
        debug!(target: "bfs", "allowed: {:?}", n_ids);
        for next in neighbours.iter() {
            if !self.visited.contains(&next.id()) {
                self.frontier.push(next.clone());
                self.visited.insert(next.id());
                self.steps.insert(next.id(), current.id());
            }
        }
    }

    fn nodes(&self) -> Box<dyn Iterator<Item=Node2d> + '_> {
        let visited = self.visited.iter()
            .map(|pos| Node2d(pos.pos2d(), NodeState::Visited));
        let frontier = self.frontier.iter()
            .map(|v| Node2d(v.id().pos2d(), NodeState::Frontier));
        if let SearchState::Finished(ref path) = self.result {
            let path = path.iter().map(|pos| Node2d(pos.pos2d(), NodeState::Path));
            Box::new( visited.chain(frontier).chain(path) )
        } else {
            Box::new( visited.chain(frontier) )
        }
    }

}

fn greedy(start: Vec<Position>, vgoals: Vec<Position>, map: &Map) -> SearchResult {
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
