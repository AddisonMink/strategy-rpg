use super::asset;

use macroquad::{
    color::WHITE,
    math::{Rect, vec2},
    texture::{self, DrawTextureParams, draw_texture_ex},
};

const NINE_SLICE_TILE_SIZE: f32 = 16.0 / 3.0;
const SCALE: f32 = 1.0;

pub fn draw_nine_slice(x: f32, y: f32, width: f32, height: f32) {
    let Some(texture) = asset::NINESLICE_TEXTURE.get() else {
        return; // Texture not loaded, skip drawing
    };

    let tile = NINE_SLICE_TILE_SIZE * SCALE;
    let center_w = width - tile * 2.0;
    let center_h = height - tile * 2.0;

    // Source rects (in texture space)
    let src = |col: f32, row: f32| {
        Rect::new(
            col * NINE_SLICE_TILE_SIZE,
            row * NINE_SLICE_TILE_SIZE,
            NINE_SLICE_TILE_SIZE,
            NINE_SLICE_TILE_SIZE,
        )
    };

    // Corners
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            source: Some(src(0.0, 0.0)),
            dest_size: Some(vec2(tile, tile)),
            ..Default::default()
        },
    ); // top-left
    draw_texture_ex(
        texture,
        x + tile + center_w,
        y,
        WHITE,
        DrawTextureParams {
            source: Some(src(2.0, 0.0)),
            dest_size: Some(vec2(tile, tile)),
            ..Default::default()
        },
    ); // top-right
    draw_texture_ex(
        texture,
        x,
        y + tile + center_h,
        WHITE,
        DrawTextureParams {
            source: Some(src(0.0, 2.0)),
            dest_size: Some(vec2(tile, tile)),
            ..Default::default()
        },
    ); // bottom-left
    draw_texture_ex(
        texture,
        x + tile + center_w,
        y + tile + center_h,
        WHITE,
        DrawTextureParams {
            source: Some(src(2.0, 2.0)),
            dest_size: Some(vec2(tile, tile)),
            ..Default::default()
        },
    ); // bottom-right

    // Edges
    // Top
    draw_texture_ex(
        texture,
        x + tile,
        y,
        WHITE,
        DrawTextureParams {
            source: Some(src(1.0, 0.0)),
            dest_size: Some(vec2(center_w, tile)),
            ..Default::default()
        },
    );
    // Bottom
    draw_texture_ex(
        texture,
        x + tile,
        y + tile + center_h,
        WHITE,
        DrawTextureParams {
            source: Some(src(1.0, 2.0)),
            dest_size: Some(vec2(center_w, tile)),
            ..Default::default()
        },
    );
    // Left
    draw_texture_ex(
        texture,
        x,
        y + tile,
        WHITE,
        DrawTextureParams {
            source: Some(src(0.0, 1.0)),
            dest_size: Some(vec2(tile, center_h)),
            ..Default::default()
        },
    );
    // Right
    draw_texture_ex(
        texture,
        x + tile + center_w,
        y + tile,
        WHITE,
        DrawTextureParams {
            source: Some(src(2.0, 1.0)),
            dest_size: Some(vec2(tile, center_h)),
            ..Default::default()
        },
    );

    // Center
    draw_texture_ex(
        texture,
        x + tile,
        y + tile,
        WHITE,
        DrawTextureParams {
            source: Some(src(1.0, 1.0)),
            dest_size: Some(vec2(center_w, center_h)),
            ..Default::default()
        },
    );
}
