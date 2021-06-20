use crate::graph::*;
use std::collections::{ HashMap, HashSet };

#[derive(Clone)]
pub struct BFSSearch<V: SearchNode> {
    pub result: SearchState<V>,
    pub frontier: Vec<V>,
    pub visited: HashSet<V::Id>,
    pub steps: HashMap<V::Id, V::Id>
}

impl<V: SearchNode> GraphSearch<Node2d> for BFSSearch<V> {

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
