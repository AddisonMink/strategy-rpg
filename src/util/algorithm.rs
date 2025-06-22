use super::{Direction, coord::Coord};
use std::collections::{HashSet, VecDeque};

pub fn flood_fill<F>(from: Coord, range: u16, accept: F) -> HashSet<Coord>
where
    F: Fn(Coord) -> bool,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(from);
    visited.insert(from);

    while let Some(coord) = queue.pop_front() {
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let next_coord = coord.shift(dir);
            if next_coord.manhattan_distance(from) <= range
                && !visited.contains(&next_coord)
                && accept(next_coord)
            {
                queue.push_back(next_coord);
                visited.insert(next_coord);
            }
        }
    }

    visited
}

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

pub fn perlin_noise_1d(x: f32, period: f32, amplitude: f32, seed: u64) -> f32 {
    let gradients = [-2.0, -1.0, 0.0, 1.0, 2.0];

    let hash = |vertex: usize| {
        let h = vertex as u64 ^ seed;
        (h.wrapping_mul(6364136223846793005).wrapping_add(1) >> 32) as usize % gradients.len()
    };

    let gradient = |vertex: usize| gradients[hash(vertex)];

    let smooth = |x: f32| 6.0 * x.powi(5) - 15.0 * x.powi(4) + 10.0 * x.powi(3);

    let mu = (x % period) / period;
    let smooth_mu = smooth(mu);
    let v0 = (x / period) as usize;
    let v1 = v0 + 1;
    let y0 = gradient(v0) * mu * amplitude;
    let y1 = gradient(v1) * (-1.0 + mu) * amplitude;
    let result = y0 * (1.0 - smooth_mu) + y1 * smooth_mu;
    (result + 2.0 * amplitude) * 0.25 / amplitude
}

pub fn breadth_first_search<F, G>(from: Coord, accept: F, goal: G) -> VecDeque<Coord>
where
    F: Fn(Coord) -> bool,
    G: Fn(Coord) -> bool,
{
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    let mut came_from = std::collections::HashMap::new();

    queue.push_back(from);
    visited.insert(from);

    while let Some(coord) = queue.pop_front() {
        if goal(coord) {
            // Reconstruct path
            let mut path = VecDeque::new();
            let mut current = coord;
            path.push_front(current);
            while let Some(&prev) = came_from.get(&current) {
                current = prev;
                path.push_front(current);
            }
            // remove origin
            path.pop_front();
            return path;
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let next_coord = coord.shift(dir);

            if accept(next_coord) && !visited.contains(&next_coord) {
                queue.push_back(next_coord);
                visited.insert(next_coord);
                came_from.insert(next_coord, coord);
            }
        }
    }

    VecDeque::new() // No path found
}
