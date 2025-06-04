#[derive(Debug, Clone, Copy)]
pub enum EffectTemplate {
    Damage { min: u16, max: u16 },
}
