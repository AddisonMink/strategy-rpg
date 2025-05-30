use std::collections::VecDeque;

use crate::util::*;

#[derive(Debug, Clone)]
pub enum GameState {
    Start,
    StartingTurn { time: f32 },
    SelectingMove { moves_left: u16 },
    NpcSelectingMove,
    ExecutingMove { next_coord: Coord, moves_left: u16 },
    NpcExecutingMove { path: VecDeque<Coord>, time: f32 },
    EndingTurn,
}
