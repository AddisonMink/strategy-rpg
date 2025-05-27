#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Coord {
    pub fn manhattan_distance(&self, other: &Coord) -> u16 {
        (self.x as i32 - other.x as i32).abs() as u16
            + (self.y as i32 - other.y as i32).abs() as u16
    }
}
