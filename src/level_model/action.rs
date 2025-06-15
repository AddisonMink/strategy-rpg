use crate::{engine::*, level_model::Entity};

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub name: ShortString,
    pub range: Range,
    pub cost: u16,
    pub effects: ShortList<EffectTemplate>,
}

#[derive(Debug, Clone, Copy)]
pub enum Range {
    SelfRange,
    SingleUnit { min: u16, max: u16 },
}

#[derive(Debug, Clone, Copy)]
pub enum EffectTemplate {
    AttackAnimation,
    Damage { min: u16, max: u16 },
    AddLightToEntity { color: Color, radius: u16 },
}
