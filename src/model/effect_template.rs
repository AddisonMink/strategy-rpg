#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectTemplate {
    Damage { min: u16, max: u16 },
}
