use super::*;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Effect {
    Damage { min: u16, max: u16, target: UnitId },
    Kill { target: UnitId },
}
