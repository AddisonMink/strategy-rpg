#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    SingleUnit { min_range: u16, max_range: u16 },
}
