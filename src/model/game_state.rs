use std::collections::VecDeque;

use super::*;
use crate::util::*;

#[derive(Debug, Clone)]
pub enum GameState {
    Start,
    StartingTurn {
        time: f32,
    },
    SelectingMove {
        moves_left: u16,
    },
    NpcSelectingMove,
    ExecutingMove {
        next_coord: Coord,
        moves_left: u16,
    },
    NpcExecutingMove {
        path: VecDeque<Coord>,
        time: f32,
    },
    SelectingAction {
        actions: Vec<Action>,
        selected_index: usize,
    },
    SelectingSingleUnitTarget {
        action: Action,
        targets: Vec<UnitId>,
        selected_index: usize,
    },
    NpcSelectingAction,
    ExecutingEffects {
        effects: VecDeque<Effect>,
    },
    EndingTurn,
}
