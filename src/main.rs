use macroquad::prelude::*;

mod model;
mod render;
mod util;

use model::*;
use render::*;
use util::*;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let map = map::Map::new();
    let mut entities = Entities::new();

    let light_id = entities.next_id();

    entities.lights.insert(
        light_id,
        Light {
            id: light_id,
            radius: 5,
        },
    );

    entities.positions.insert(
        light_id,
        Position {
            coord: Coord { x: 1, y: 1 },
        },
    );

    let unit_id = entities.next_id();

    entities.lights.insert(
        unit_id,
        Light {
            id: unit_id,
            radius: 3,
        },
    );

    entities.positions.insert(
        unit_id,
        Position {
            coord: Coord { x: 14, y: 3 },
        },
    );

    entities.units.insert(
        unit_id,
        Unit {
            glyph: Glyph {
                symbol: 'A',
                color: WHITE,
            },
            vision: 2,
        },
    );

    let mut light_grid = LightGrid::new(&map, &entities);

    let mut time = 0.0;

    loop {
        time += get_frame_time();
        let base = 0.5;
        let flicker = algorithm::perlin_noise_1d(time, 0.5, 1.0, 42);
        let torch_light = base + flicker * 0.5;

        update_unit_position(&map, &mut light_grid, &mut entities, unit_id);
        clear_background(BLACK);
        draw_visible_grid(&map, &entities, &light_grid, unit_id, torch_light);
        draw_text("0.0.1", 8.0, 16.0, 16.0, WHITE);
        next_frame().await;
    }
}

fn update_unit_position(
    map: &Map,
    light_grid: &mut LightGrid,
    entities: &mut Entities,
    entity_id: EntityID,
) -> Option<()> {
    let position = entities.positions.get_mut(&entity_id)?;
    if let Some(direction) = input::pressed_direction() {
        let new_coord = position.coord.shift(direction);
        if map.walkable(new_coord) {
            position.coord = new_coord;
        }
        *light_grid = LightGrid::new(map, entities);
    }

    Some(())
}

fn draw_visible_grid(
    map: &Map,
    entities: &Entities,
    light_grid: &LightGrid,
    entity_id: EntityID,
    torch_light: f32,
) -> Option<()> {
    let position = entities.positions.get(&entity_id)?;
    let unit = entities.units.get(&entity_id)?;
    let torch_color = Color {
        r: 1.0 * torch_light,
        g: 0.65 * torch_light,
        b: 0.0,
        a: 1.0,
    };

    for x in 0..Map::WIDTH {
        for y in 0..Map::HEIGHT {
            let coord = Coord { x, y };

            if !map.check_line_of_sight(position.coord, coord) {
                continue;
            }

            let distance = light_grid.distance_from_light(coord);
            if distance <= unit.vision {
                let tile = map.tile(coord);

                if let Some(bg_color) = tile.background {
                    draw_grid::draw_square(coord, mix_color(bg_color, torch_color, 0.5));
                }

                if let Some(unit) = entities.unit_at(coord) {
                    let glyph = Glyph {
                        symbol: unit.glyph.symbol,
                        color: mix_color(unit.glyph.color, torch_color, 0.5),
                    };
                    draw_grid::draw_glyph(coord, glyph);
                } else {
                    let glyph = Glyph {
                        symbol: tile.glyph.symbol,
                        color: mix_color(tile.glyph.color, torch_color, 0.5),
                    };
                    draw_grid::draw_glyph(coord, glyph);
                }

                if distance > 0 {
                    draw_grid::draw_square(coord, BLACK.with_alpha(0.5));
                }
            }
        }
    }

    Some(())
}
