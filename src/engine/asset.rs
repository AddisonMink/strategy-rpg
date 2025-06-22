use macroquad::prelude::*;
use once_cell::sync::OnceCell;

pub static MAP_FONT: OnceCell<Font> = OnceCell::new();
pub static UI_FONT: OnceCell<Font> = OnceCell::new();
pub static NINESLICE_TEXTURE: OnceCell<Texture2D> = OnceCell::new();

pub async fn load_assets() {
    let map_font: Font = load_ttf_font("assets/fonts/EFRogue.ttf").await.unwrap();
    MAP_FONT.set(map_font).unwrap();

    let ui_font: Font = load_ttf_font("assets/fonts/kongtext.ttf").await.unwrap();
    UI_FONT.set(ui_font).unwrap();

    let nineslice_texture: Texture2D = load_texture("assets/textures/nineslice.png").await.unwrap();
    nineslice_texture.set_filter(FilterMode::Nearest);
    NINESLICE_TEXTURE.set(nineslice_texture).unwrap();
    trace!("Assets loaded successfully");
}
