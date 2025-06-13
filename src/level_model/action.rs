use crate::engine::*;

#[derive(Debug, Clone)]
pub struct Action {
    pub name: ShortString,
    pub range: Range,
    pub effects: ShortList<EffectTemplate>,
}

impl Action {
    pub const ATTACK: Action = Action {
        name: ShortString::new("Attack"),
        range: Range::SingleUnit { min: 1, max: 1 },
        effects: ShortList::new(&[
            EffectTemplate::AttackAnimation,
            EffectTemplate::Damage { min: 1, max: 3 },
        ]),
    };
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
}
