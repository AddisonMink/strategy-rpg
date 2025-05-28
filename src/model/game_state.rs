use crate::util::*;

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Start,
    SelectingMove { moves_left: u16 },
    ExecutingMove { next_coord: Coord, moves_left: u16 },
    EndingTurn,
}
