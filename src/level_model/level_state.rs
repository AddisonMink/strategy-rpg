use super::action::Action;
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
    SelectingAction {
        actions: Vec<Action>,
        panel: Panel,
        panel_origin: (f32, f32),
        target_coords: Option<HashSet<Coord>>,
    },
}
