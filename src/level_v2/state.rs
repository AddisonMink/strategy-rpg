use super::Action;
use super::*;
use crate::{level_model::Item, util::*};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy)]
pub enum LevelResult {
    Restart,
}

#[derive(Debug, Clone)]
pub struct SelectingMove {
    pub valid_moves: HashSet<Coord>,
    pub path: Option<VecDeque<Coord>>,
    pub cancel_button: Button,
    pub action_preview: Panel,
    pub unit_description_opt: Option<Panel>,
    pub tile_description_opt: Option<Panel>,
    pub action_description_opt: Option<Panel>,
}

#[derive(Debug, Clone)]
pub struct SelectingAction {
    pub actions: Vec<ItemAction>,
    pub cancel_button: Button,
    pub action_list: Panel,
    pub unit_description_opt: Option<Panel>,
    pub tile_description_opt: Option<Panel>,
    pub action_description_opt: Option<Panel>,
}

#[derive(Debug, Clone)]
pub struct SelectingEnemyTarget {
    pub action: ItemAction,
    pub targets: HashMap<Coord, UnitId>,
    pub selected_target: Option<Coord>,
    pub cancel_button: Button,
    pub action_description: Panel,
    pub unit_description_opt: Option<Panel>,
    pub tile_description_opt: Option<Panel>,
}

#[derive(Debug, Clone)]
pub enum State {
    Starting,
    SelectingMove(SelectingMove),
    ResolvingMove,
    SelectingAction(SelectingAction),
    SelectingEnemyTarget(SelectingEnemyTarget),
    ResolvingAction,
    EndingTurn,
    Failure,
    Success,
    Ending(LevelResult),
}
