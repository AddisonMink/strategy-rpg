use crate::engine::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub enum LevelState {
    Starting,
    SelectingMove {
        valid_moves: HashSet<Coord>,
        path: Option<VecDeque<Coord>>,
    },
    ResolvingMove,
}
