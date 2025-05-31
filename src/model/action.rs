use super::{EffectTemplate, Range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    pub range: Range,
    pub effect_templates: Vec<EffectTemplate>,
}
