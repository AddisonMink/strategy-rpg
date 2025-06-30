use super::*;

pub const SWORD_DATA: ItemData = ItemData {
    id: ItemId(0),
    name: ShortString::new("Sword"),
    color: Color::new(0.8, 0.8, 0.8, 1.0),
    charges_max: 2,
    actions: ShortList::new(&[Action {
        name: ShortString::new("Slash"),
        cost: 1,
        range: ActionRange::Enemy {
            min_range: 1,
            max_range: 1,
        },
        effects: ShortList::new(&[
            ActionEffect::Attack,
            ActionEffect::Damage { min: 1, max: 3 },
        ]),
    }]),
};

pub const TORCH_DATA: ItemData = ItemData {
    id: ItemId(1),
    name: ShortString::new("Torch"),
    color: ORANGE,
    charges_max: 2,
    actions: ShortList::new(&[
        Action {
            name: ShortString::new("Ignite"),
            cost: 1,
            range: ActionRange::SelfRange,
            effects: ShortList::new(&[ActionEffect::Light {
                light: Light {
                    radius: 3,
                    color: ORANGE,
                },
            }]),
        },
        Action {
            name: ShortString::new("Red Seed"),
            cost: 1,
            range: ActionRange::Enemy {
                min_range: 2,
                max_range: 5,
            },
            effects: ShortList::new(&[
                ActionEffect::Projectile,
                ActionEffect::Light {
                    light: Light {
                        radius: 2,
                        color: ORANGE,
                    },
                },
                ActionEffect::Damage { min: 1, max: 3 },
            ]),
        },
        // TODO: Red Flower
    ]),
};
