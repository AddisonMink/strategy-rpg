use crate::engine::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub enum BattleState {
    Starting,
    SelectingMove { valid_moves: HashSet<Coord> },
    ExecutingMove { path: VecDeque<Coord>, timer: Timer },
}
