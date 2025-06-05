use super::*;
use crate::engine::*;
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
        actions: Vec<Action>,
        selected_index: Option<usize>,
        panel: Panel,
    },
    SelectingSingleUnitTarget {
        action: Action,
        targets: HashSet<UnitId>,
        selected_target: Option<UnitId>,
    },
    ExecutingEffects {
        effects: VecDeque<Effect>,
        animations: VecDeque<Animation>,
    },
}
