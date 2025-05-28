use crate::util::*;
use macroquad::prelude::*;

const PADDING: f32 = 10.0;
const TITLE_FONT_SIZE: u16 = 16;
const METER_WIDTH: f32 = 100.0;
const METER_SPACING: f32 = 2.0;
const NINE_SLICE_TILE_SIZE: f32 = 16.0 / 3.0;

enum Content {
    Text {
        text: String,
        color: Color,
    },
    Meter {
        label: String,
        label_color: Color,
        value: u16,
        max: u16,
        meter_color: Color,
        width: f32,
    },
}

pub struct Panel {
    title: String,
    title_color: Color,
    width: f32,
    height: f32,
    lines: Vec<Content>,
}

pub struct PanelBuilder {
    title: String,
    title_color: Color,
    lines: Vec<Content>,
}

impl PanelBuilder {
    pub fn new(title: impl Into<String>, title_color: Color) -> Self {
        Self {
            title: title.into(),
            title_color,
            lines: Vec::new(),
        }
    }

    pub fn line(mut self, text: impl Into<String>, color: Color) -> Self {
        self.lines.push(Content::Text {
            text: text.into(),
            color,
        });
        self
    }

    pub fn meter(
        mut self,
        label: impl Into<String>,
        label_color: Color,
        value: u16,
        max: u16,
        meter_color: Color,
    ) -> Self {
        self.lines.push(Content::Meter {
            label: label.into(),
            label_color,
            value,
            max,
            meter_color,
            width: METER_WIDTH,
        });
        self
    }

    pub fn short_meter(
        mut self,
        label: impl Into<String>,
        label_color: Color,
        value: u16,
        max: u16,
        meter_color: Color,
    ) -> Self {
        self.lines.push(Content::Meter {
            label: label.into(),
            label_color,
            value,
            max,
            meter_color,
            width: METER_WIDTH / 2.0,
        });
        self
    }

    pub fn build(self) -> Panel {
        let title_size = text_size(&self.title);
        let inter_line_padding = (self.lines.len() as f32 - 1.0).max(0.0) * PADDING + PADDING / 2.0;

        let mut height = 0.0;
        let mut total_width: f32 = title_size.width;

        for content in &self.lines {
            match content {
                Content::Text { text, .. } => {
                    let line_size = text_size(text);
                    height += line_size.height;
                    total_width = total_width.max(line_size.width);
                }
                Content::Meter { label, width, .. } => {
                    let label_size = text_size(label);
                    height += label_size.height;
                    total_width = total_width.max(label_size.width + *width + PADDING);
                }
            }
        }
        total_width += PADDING * 2.0;
        height += inter_line_padding + PADDING * 2.0;
        Panel {
            title: self.title,
            title_color: self.title_color,
            width: total_width,
            height,
            lines: self.lines,
        }
    }
}

impl Panel {
    pub fn empty(title: impl Into<String>, title_color: Color, width: f32, height: f32) -> Self {
        Self {
            title: title.into(),
            title_color,
            width,
            height,
            lines: Vec::new(),
        }
    }

    pub fn builder(title: impl Into<String>, title_color: Color) -> PanelBuilder {
        PanelBuilder::new(title, title_color)
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn draw(&self, x: f32, y: f32) {
        let title_size = text_size(&self.title);
        let title_x = x + PADDING;
        let title_y = y - title_size.height / 2.0;

        draw_nine_slice(x, y, self.width, self.height, 1.0);
        draw_rectangle(title_x, title_y, title_size.width, title_size.height, BLACK);
        draw_text_line(title_x, title_y, &self.title, self.title_color);

        let mut current_y = y + PADDING * 1.5;

        for content in &self.lines {
            match content {
                Content::Text { text, color } => {
                    let line_size = text_size(text);
                    draw_text_line(x + PADDING, current_y, text, *color);
                    current_y += line_size.height + PADDING;
                }
                Content::Meter {
                    label,
                    label_color,
                    value,
                    max,
                    meter_color,
                    width,
                } => {
                    let label_size = text_size(&label);
                    let meter_x = x + label_size.width + PADDING;
                    draw_text_line(x + PADDING, current_y, label, *label_color);
                    draw_meter(meter_x, current_y, *width, *value, *max, *meter_color);
                    current_y += label_size.height + PADDING;
                }
            }
        }
    }
}

fn text_size(text: &str) -> TextDimensions {
    measure_text(text, asset::UI_FONT.get(), TITLE_FONT_SIZE, 1.0)
}

fn draw_text_line(x: f32, y: f32, text: &str, color: Color) {
    let text_size = text_size(text);
    draw_text_ex(
        text,
        x,
        y + text_size.offset_y,
        TextParams {
            font: asset::UI_FONT.get(),
            font_size: TITLE_FONT_SIZE,
            color,
            ..Default::default()
        },
    );
}

fn draw_meter(x: f32, y: f32, width: f32, value: u16, max: u16, color: Color) {
    let total_spacing = (max - 1) as f32 * METER_SPACING;
    let pip_width = (width - total_spacing) / max as f32;
    let meter_height = TITLE_FONT_SIZE as f32 * 0.80;

    let mut pip_x = x;
    for i in 0..max {
        let color = if i < value {
            color
        } else {
            color.with_alpha(0.5)
        };
        draw_rectangle(pip_x, y, pip_width, meter_height, color);
        pip_x += pip_width + METER_SPACING;
    }
}

fn draw_nine_slice(x: f32, y: f32, width: f32, height: f32, scale: f32) {
    let texture = asset::NINESLICE_TEXTURE.get().unwrap();
    let tile = NINE_SLICE_TILE_SIZE * scale;
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
