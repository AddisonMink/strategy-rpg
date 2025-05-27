use crate::coord::Coord;

pub fn check_bresenhem_line<F>(from: Coord, to: Coord, accept: F) -> bool
where
    F: Fn(Coord) -> bool,
{
    let dx = (to.x as i32 - from.x as i32).abs();
    let dy = (to.y as i32 - from.y as i32).abs();
    let sx = if from.x < to.x { 1 } else { -1 };
    let sy = if from.y < to.y { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = from.x as i32;
    let mut y = from.y as i32;

    loop {
        let coord = Coord {
            x: x as u16,
            y: y as u16,
        };

        if coord == to {
            return true;
        } else if !accept(coord) {
            return false;
        }

        let err2 = err * 2;
        if err2 > -dy {
            err -= dy;
            x += sx;
        }
        if err2 < dx {
            err += dx;
            y += sy;
        }
    }
}
