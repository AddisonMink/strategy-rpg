use crate::{battle::model::Action, engine::*};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub enum BattleState {
    Starting,
    SelectingMove {
        valid_moves: HashSet<Coord>,
        path: Option<VecDeque<Coord>>,
    },
    ExecutingMove {
        path: VecDeque<Coord>,
        timer: Timer,
    },
    SelectingAction {
        valid_actions: Vec<Action>,
    },
}
