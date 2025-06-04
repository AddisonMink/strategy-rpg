#[derive(Debug, Clone, Copy)]
pub enum Range {
    SingleUnit { min: u16, max: u16 },
}
