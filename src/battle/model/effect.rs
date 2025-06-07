use super::*;

#[derive(Debug, Clone, Copy)]
pub enum Effect {
    Noop,
    Damage { min: u16, max: u16, target: UnitId },
    QueueAnimation { animation: Animation },
}
