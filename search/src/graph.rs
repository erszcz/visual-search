use std::collections::{ HashMap };
use std::fmt::Debug;
use std::hash::Hash;

pub trait Positionable {
    fn pos2d(&self) -> (usize, usize);
    fn pos3d(&self) -> (usize, usize, usize);
}

pub trait SearchNode: Clone + Eq + Hash {
    type Id: Clone + Debug + Eq + Hash + Positionable;
    fn id(&self) -> Self::Id;
    fn is_goal(&self) -> bool;
    fn neighbours(&self) -> Vec<Self>;
}

pub trait GraphSearch<NodeId> {
    fn step(&mut self);
    fn nodes(&self) -> Box<dyn Iterator<Item=NodeId> + '_>;
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

pub enum NodeState {
    Visited,
    Frontier,
    Path
}

pub struct Node2d(pub (usize, usize), pub NodeState);
pub struct Node3d(pub (usize, usize, usize), pub NodeState);

pub fn build_path<Node: SearchNode>(steps: &HashMap<Node::Id, Node::Id>,
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
