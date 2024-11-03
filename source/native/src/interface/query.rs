use {
    super::triple::Node,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum MoveDirection {
    Down,
    Up,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct StepMove {
    pub dir: MoveDirection,
    pub predicate: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct StepRecurse {
    pub chain: Vec<Step>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Step {
    Move(StepMove),
    Recurse0(StepRecurse),
    Recurse1(StepRecurse),
    First,
}

pub struct Chain {
    pub steps: Vec<Step>,
    pub select: Option<String>,
    pub children: Vec<Chain>,
}

pub struct Query {
    pub root: Option<Node>,
    pub chain: Chain,
}
