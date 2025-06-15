use super::action::Action;
use super::entity::Entity;
use crate::{engine::*, level_model::ItemId};
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
        actions: Vec<ItemAction>,
        panel: Panel,
        selected_action: Option<ItemAction>,
        target_coords: Option<HashSet<Coord>>,
    },
    SelectingSingleUnitTarget {
        action: ItemAction,
        targets: HashMap<Coord, Entity>,
        selected_target: Option<Coord>,
    },
    ResolvingAction,
    EndingTurn,
}

#[derive(Debug, Clone, Copy)]
pub struct ActionPreview {
    pub name: ShortString,
    pub valid: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ItemAction {
    pub item_id: ItemId,
    pub item_name: ShortString,
    pub item_color: Color,
    pub action: Action,
}
