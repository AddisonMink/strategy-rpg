use coord::Coord;
use entity::*;
use glyph::Glyph;
use light_grid::LightGrid;
use macroquad::prelude::*;
use map::Map;

mod algorithm;
mod asset;
mod coord;
mod direction;
mod draw_grid;
mod entity;
mod glyph;
mod input;
mod light_grid;
mod map;
mod tile;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let map = map::Map::new();
    let mut entities = Entities::new();

    let light_id = entities.next_id();
    entities.add_light(light_id, 5);
    entities.add_position(light_id, Coord { x: 1, y: 1 });

    let unit_id = entities.next_id();
    entities.add_light(unit_id, 3);
    entities.add_position(unit_id, Coord { x: 14, y: 1 });

    entities.add_unit(
        unit_id,
        Glyph {
            symbol: '@',
            color: WHITE,
        },
        2,
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
        next_frame().await;
    }
}

fn update_unit_position(
    map: &Map,
    light_grid: &mut LightGrid,
    entities: &mut Entities,
    entity_id: EntityID,
) -> Option<()> {
    let position = entities.position_mut(entity_id)?;
    if let Some(direction) = input::pressed_direction() {
        let new_coord = position.coord.shift(direction);
        if map.walkable(new_coord) {
            position.coord = new_coord;
        }
        *light_grid = LightGrid::new(map, entities);
    }

    Some(())
}

fn draw_light_grid(light_grid: &LightGrid) {
    for x in 0..Map::WIDTH {
        for y in 0..Map::HEIGHT {
            let coord = Coord { x, y };
            let distance = light_grid.distance_from_light(coord);
            if distance < 10 {
                draw_grid::draw_string(coord, &distance.to_string(), WHITE);
            }
        }
    }
}

fn draw_visible_grid(
    map: &Map,
    entities: &Entities,
    light_grid: &LightGrid,
    entity_id: EntityID,
    torch_light: f32,
) -> Option<()> {
    let position = entities.position(entity_id)?;
    let unit = entities.unit(entity_id)?;
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

fn draw_full_grid(map: &Map, entities: &Entities, light_grid: &LightGrid) {
    for x in 0..Map::WIDTH {
        for y in 0..Map::HEIGHT {
            let coord = Coord { x, y };
            if light_grid.distance_from_light(coord) == 0 {
                let tile = map.tile(coord);
                if let Some(bg_color) = tile.background {
                    draw_grid::draw_square(coord, bg_color);
                }
                if let Some(unit) = entities.unit_at(coord) {
                    draw_grid::draw_glyph(coord, unit.glyph);
                } else {
                    draw_grid::draw_glyph(coord, tile.glyph);
                }
            }
        }
    }
}

fn mix_color(primary: Color, secondary: Color, ratio: f32) -> Color {
    let r = primary.r * (1.0 - ratio) + secondary.r * ratio;
    let g = primary.g * (1.0 - ratio) + secondary.g * ratio;
    let b = primary.b * (1.0 - ratio) + secondary.b * ratio;
    let a = primary.a * (1.0 - ratio) + secondary.a * ratio;
    Color { r, g, b, a }
}
