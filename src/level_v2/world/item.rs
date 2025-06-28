use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct ItemData {
    pub id: ItemId,
    pub name: ShortString,
    pub color: Color,
    pub charges_max: u16,
    pub actions: ShortList<Action>,
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    data: ItemData,
    pub charges: u16,
}

impl Item {
    pub fn new(data: ItemData) -> Self {
        Self {
            data,
            charges: data.charges_max,
        }
    }

    pub fn data(&self) -> &ItemData {
        &self.data
    }

    pub fn actions(&self) -> Vec<ItemAction> {
        self.data
            .actions
            .iter()
            .map(|action| ItemAction {
                item_name: self.data.name,
                item_id: self.data.id,
                item_color: self.data.color,
                item_charges: self.charges,
                item_charges_max: self.data.charges_max,
                action: *action,
            })
            .filter(|a| a.action.cost <= a.item_charges)
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemAction {
    pub item_name: ShortString,
    pub item_id: ItemId,
    pub item_color: Color,
    pub item_charges: u16,
    pub item_charges_max: u16,
    pub action: Action,
}

impl ItemAction {
    pub fn compile_self_action(
        &self,
        world: &World,
        unit: &Unit,
        target_id: UnitId,
    ) -> VecDeque<Effect> {
        let mut effects = VecDeque::new();

        for effect in self.action.effects.iter() {
            match effect {
                ActionEffect::Light { light } => {
                    let effect = Effect::AddUnitLight {
                        id: unit.id(),
                        light: *light,
                    };
                    effects.push_back(effect);
                }
                _ => {}
            }
        }

        effects.push_back(Effect::ConsumeCharge {
            id: unit.id(),
            item_id: self.item_id,
            amount: self.action.cost,
        });

        effects
    }

    pub fn compile_single_enemy_action(
        &self,
        world: &World,
        unit: &Unit,
        enemy_id: UnitId,
    ) -> VecDeque<Effect> {
        let mut effects = VecDeque::new();

        for effect in self.action.effects.iter() {
            match effect {
                ActionEffect::Attack => {
                    if let Some(enemy) = world.unit(enemy_id) {
                        let dir = unit.coord.direction_to(enemy.coord).unwrap();
                        let animation = Animation::attack(unit.id(), dir);
                        let effect = Effect::Animate { animation };
                        effects.push_back(effect);
                    }
                }
                ActionEffect::Damage { min, max } => {
                    let effect = Effect::Damage {
                        id: enemy_id,
                        min: *min,
                        max: *max,
                    };
                    effects.push_back(effect);
                }
                ActionEffect::Light { light } => {
                    let effect = Effect::AddUnitLight {
                        id: unit.id(),
                        light: *light,
                    };
                    effects.push_back(effect);
                }
            }
        }

        effects.push_back(Effect::ConsumeCharge {
            id: unit.id(),
            item_id: self.item_id,
            amount: self.action.cost,
        });

        effects
    }
}
