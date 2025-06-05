use crate::battle::model::UnitId;

#[derive(Debug, Clone, Copy)]
pub enum Effect {
    Damage { min: u16, max: u16, target: UnitId },
}
