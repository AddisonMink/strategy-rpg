use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Coord {
    pub fn manhattan_distance(&self, other: Coord) -> u16 {
        (self.x as i32 - other.x as i32).abs() as u16
            + (self.y as i32 - other.y as i32).abs() as u16
    }

    pub fn shift(&self, direction: Direction) -> Coord {
        match direction {
            Direction::Up => Coord {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coord {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}
