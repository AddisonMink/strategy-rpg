// General Constants
pub const PADDING: f32 = 10.0;
pub const TEXT_SIZE: u16 = 16;
pub const MAP_TEXT_SIZE: u16 = 24;
pub const TILE_SIZE: f32 = 32.0;

// Grid Constants
pub const GRID_COLUMNS: u16 = 10;
pub const GRID_ROWS: u16 = 10;
pub const GRID_ORIGIN: (f32, f32) = (PADDING * 2.0, PADDING * 2.0);
pub const GRID_WIDTH: f32 = GRID_COLUMNS as f32 * TILE_SIZE;
pub const GRID_HEIGHT: f32 = GRID_ROWS as f32 * TILE_SIZE;
pub const GRID_PANE_ORIGIN: (f32, f32) = (PADDING, PADDING);
pub const GRID_PANE_WIDTH: f32 = GRID_WIDTH + PADDING * 2.0;
pub const GRID_PANE_HEIGHT: f32 = GRID_HEIGHT + PADDING * 2.0;

// UI Constants
pub const UI_WIDTH: f32 = 300.0;

pub const UI_ORIGIN: (f32, f32) = (
    GRID_PANE_ORIGIN.0 + GRID_PANE_WIDTH + PADDING,
    GRID_PANE_ORIGIN.1,
);
