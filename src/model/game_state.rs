use crate::util::*;

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Start,
    StartingTurn { time: f32 },
    SelectingMove { moves_left: u16 },
    ExecutingMove { next_coord: Coord, moves_left: u16 },
    EndingTurn,
}
