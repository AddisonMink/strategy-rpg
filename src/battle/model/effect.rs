use crate::battle::model::UnitId;

#[derive(Debug, Clone, Copy)]
pub enum Effect {
    Damage { min: u16, max: u16, target: UnitId },
}

impl Effect {
    pub fn damage(min: u16, max: u16, target: UnitId) -> Self {
        Self::Damage { min, max, target }
    }
}
