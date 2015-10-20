use std::collections::{ HashMap, HashSet };
use std::fmt::Debug;
use std::hash::Hash;

use super::Error;

pub trait SearchNode: Clone + Eq + Hash {
    type Id: Clone + Debug + Eq + Hash;
    fn id(&self) -> Self::Id;
    fn is_goal(&self) -> bool;
    fn neighbours(&self) -> Vec<Self>;
}

pub trait GraphSearch {

    fn step(&mut self);

}

pub struct BFSSearch<Node: SearchNode> {
    pub result: Option<Result<Vec<Node::Id>, Error>>,
    pub frontier: Vec<Node>,
    pub visited: HashSet<Node::Id>,
    pub steps: HashMap<Node::Id, Node::Id>
}

impl<Node: SearchNode> GraphSearch for BFSSearch<Node> {

    fn step(&mut self) {
        if self.result.is_some() {
            return
        }
        if self.frontier.is_empty() {
            self.result = Some (Err (Error::GoalUnreachable));
            return
        }
        let current = self.frontier.remove(0);
        debug!("visited: {:?}", self.visited);
        debug!("current: {:?}", current.id());
        debug!("steps  : {:?}", self.steps);
        if current.is_goal() {
            debug!("goal found: {:?}", current.id());
            let path = build_path::<Node>(&self.steps, current.id());
            self.result = Some (Ok (path));
            return
        }
        let neighbours = current.neighbours();
        let n_ids: Vec<Node::Id> = neighbours.iter().map(|n| n.id()).collect();
        debug!("allowed: {:?}", n_ids);
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
