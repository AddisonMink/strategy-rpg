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

    let mut game = Game::new(Map::new());
    game.add_unit(make_unit);
    game.add_point_light(make_point_light);
    game.light_grid = LightGrid::new(&game);
    game.next_turn();
    let mut time = 0.0;

    loop {
        time += get_frame_time();
        let base = 0.5;
        let flicker = algorithm::perlin_noise_1d(time, 0.5, 1.0, 42);
        let torch_light = base + flicker * 0.5;

        update_unit_position(&mut game);
        clear_background(BLACK);
        draw_visible_grid(&game, torch_light);
        draw_text("0.0.1", 8.0, 16.0, 16.0, WHITE);
        next_frame().await;
    }
}

fn make_unit(id: UnitId) -> Unit {
    Unit {
        id,
        coord: Coord { x: 14, y: 1 },
        glyph: Glyph {
            symbol: '@',
            color: WHITE,
        },
        vision: 2,
        movement: 3,
        light: Some(Light {
            radius: 3,
            color: ORANGE,
        }),
    }
}

fn make_point_light(id: PointLightId) -> PointLight {
    PointLight {
        id,
        coord: Coord { x: 1, y: 1 },
        light: Light {
            radius: 5,
            color: BLUE,
        },
    }
}

fn update_unit_position(game: &mut Game) -> Option<()> {
    let coord = game.active_unit()?.coord;
    if let Some(direction) = input::pressed_direction() {
        let new_coord = coord.shift(direction);
        if game.map.walkable(new_coord) {
            game.active_unit_mut()?.coord = new_coord;
        }
        game.light_grid = LightGrid::new(game);
    }

    Some(())
}

fn draw_visible_grid(game: &Game, flicker: f32) -> Option<()> {
    let unit = game.active_unit()?;

    for x in 0..Map::WIDTH {
        for y in 0..Map::HEIGHT {
            let coord = Coord { x, y };

            if !game.map.check_line_of_sight(unit.coord, coord) {
                continue;
            }

            let light_color = game.light_grid.color_at(coord).with_alpha(flicker);
            let distance = game.light_grid.distance_from_light(coord);

            if distance <= unit.vision {
                let tile = game.map.tile(coord);

                if let Some(bg_color) = tile.background {
                    draw_grid::draw_square(coord, mix_color(bg_color, light_color, 0.5));
                }

                if let Some(unit) = game.unit_at(coord) {
                    let glyph = Glyph {
                        symbol: unit.glyph.symbol,
                        color: mix_color(unit.glyph.color, light_color, 0.5),
                    };
                    draw_grid::draw_glyph(coord, glyph);
                } else {
                    let glyph = Glyph {
                        symbol: tile.glyph.symbol,
                        color: mix_color(tile.glyph.color, light_color, 0.5),
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
