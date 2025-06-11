use super::action::Action;
use super::entity::Entity;
use crate::engine::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub enum LevelState {
    Starting,
    SelectingMove {
        valid_moves: HashSet<Coord>,
        path: Option<VecDeque<Coord>>,
        action_previews: Vec<ActionPreview>,
    },
    ResolvingMove,
    SelectingAction {
        actions: Vec<Action>,
        panel: Panel,
        panel_origin: (f32, f32),
        target_coords: Option<HashSet<Coord>>,
    },
    CompilingAction {
        action: Action,
    },
    SelectingSingleUnitTarget {
        action: Action,
        targets: HashMap<Coord, Entity>,
    },
    ResolvingAction,
    EndingTurn,
}

#[derive(Debug, Clone, Copy)]
pub struct ActionPreview {
    pub name: ShortString,
    pub valid: bool,
}
