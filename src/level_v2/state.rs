use super::Action;
use crate::util::*;
use std::collections::{HashSet, VecDeque};

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
    pub actions: Vec<Action>,
    pub cancel_button: Button,
    pub action_list: Panel,
    pub unit_description_opt: Option<Panel>,
    pub tile_description_opt: Option<Panel>,
    pub action_description_opt: Option<Panel>,
}

#[derive(Debug, Clone)]
pub enum State {
    Starting,
    SelectingMove(SelectingMove),
    ResolvingMove,
    SelectingAction(SelectingAction),
    EndingTurn,
}
