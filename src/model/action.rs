use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    pub range: Range,
    pub effect_templates: Vec<EffectTemplate>,
}
