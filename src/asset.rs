use macroquad::prelude::*;
use once_cell::sync::OnceCell;

pub static MAP_FONT: OnceCell<Font> = OnceCell::new();

pub async fn load_assets() {
    let map_font: Font = load_ttf_font("assets/fonts/EFRogue.ttf").await.unwrap();
    MAP_FONT.set(map_font).unwrap();
}
