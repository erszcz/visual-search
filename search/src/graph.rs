use std::collections::{ HashMap, HashSet };
use std::fmt::Debug;
use std::hash::Hash;

use super::{ MapField };

pub trait SearchNode: Clone + Eq + Hash {
    type Id: Clone + Debug + Eq + Hash;
    fn id(&self) -> Self::Id;
    fn is_goal(&self) -> bool;
    fn neighbours(&self) -> Vec<Self>;
}

pub trait GraphSearch {
    fn step(&mut self);
}

#[derive(Clone)]
pub enum SearchState<V: SearchNode> {
    NotStarted,
    InProgress,
    Finished(Vec<V::Id>),
    Failed(String)
}

impl<V: SearchNode> SearchState<V> {
    pub fn is_over(&self) -> bool {
        match *self {
            SearchState::Finished(_) => true,
            SearchState::Failed(_) => true,
            SearchState::NotStarted => false,
            SearchState::InProgress => false
        }
    }
}

#[derive(Clone)]
pub struct BFSSearch<V: SearchNode> {
    pub result: SearchState<V>,
    pub frontier: Vec<V>,
    pub visited: HashSet<V::Id>,
    pub steps: HashMap<V::Id, V::Id>
}

enum NodeState {
    //Regular,
    //Start,
    //Goal,
    Visited,
    Path,
    Frontier
}

pub struct Node((usize, usize), NodeState);

impl BFSSearch<MapField> {
    pub fn nodes(&self) -> Box<dyn Iterator<Item=Node> + '_> {
        let visited = self.visited.iter()
            .map(|pos| Node(*pos, NodeState::Visited));
        let frontier = self.frontier.iter()
            .map(|map_field| Node(map_field.pos, NodeState::Frontier));
        if let SearchState::Finished(ref path) = self.result {
            let path = path.iter().map(|pos| Node(*pos, NodeState::Path));
            Box::new( visited.chain(frontier).chain(path) )
        } else {
            Box::new( visited.chain(frontier) )
        }
    }
}

impl<Node: SearchNode> GraphSearch for BFSSearch<Node> {

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
            let path = build_path::<Node>(&self.steps, current.id());
            self.result = SearchState::Finished(path);
            return
        }
        let neighbours = current.neighbours();
        let n_ids: Vec<Node::Id> = neighbours.iter().map(|n| n.id()).collect();
        debug!(target: "bfs", "allowed: {:?}", n_ids);
        for next in neighbours.iter() {
            if !self.visited.contains(&next.id()) {
                self.frontier.push(next.clone());
                self.visited.insert(next.id());
                self.steps.insert(next.id(), current.id());
            }
        }
    }

}

fn build_path<Node: SearchNode>(steps: &HashMap<Node::Id, Node::Id>,
                                goal: Node::Id) -> Vec<Node::Id>
{
    let mut path = vec!();
    let mut last = goal;
    loop {
        path.push(last.clone());
        match steps.get(&last) {
            None => break,
            Some (step) => last = step.clone()
        }
    }
    path
}
