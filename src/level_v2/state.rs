use crate::{engine::Panel, util::*};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct SelectingMove {
    pub valid_moves: HashSet<Coord>,
    pub path: Option<VecDeque<Coord>>,
    pub cancel_button: Button,
    pub unit_description_opt: Option<Panel>,
    pub tile_description_opt: Option<Panel>,
}

pub enum State {
    Starting,
    SelectingMove(SelectingMove),
    ResolvingMove,
    EndingTurn,
}
