use crate::engine::*;
use crate::level_model::*;

pub const SWORD: Item = Item {
    id: ItemId(1),
    name: ShortString::new("Sword"),
    color: LIGHTGRAY,
    uses_max: 5,
    uses: 5,
    actions: ShortList::new(&[SLASH]),
};

pub const TORCH: Item = Item {
    id: ItemId(2),
    name: ShortString::new("Torch"),
    color: ORANGE,
    uses_max: 1,
    uses: 1,
    actions: ShortList::new(&[LIGHT]),
};

const SLASH: Action = Action {
    name: ShortString::new("Slash"),
    range: Range::SingleUnit { min: 1, max: 1 },
    cost: 1,
    effects: ShortList::new(&[
        EffectTemplate::AttackAnimation,
        EffectTemplate::Damage { min: 1, max: 4 },
    ]),
};

const LIGHT: Action = Action {
    name: ShortString::new("Light"),
    range: Range::SelfRange,
    cost: 1,
    effects: ShortList::new(&[EffectTemplate::AddLightToEntity {
        color: ORANGE,
        radius: 3,
    }]),
};
