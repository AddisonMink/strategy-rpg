use crate::constants::PADDING;
use crate::engine_v2::*;

#[derive(Debug, Clone)]
pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
    color: Color,
}

impl Button {
    pub fn new(
        label: String,
        x: f32,
        y: f32,
        min_width: f32,
        min_height: f32,
        color: Color,
    ) -> Self {
        let (text_width, text_height) = text_size(&label);
        let default_width = text_width + PADDING * 2.0;
        let default_height = text_height + PADDING * 2.0;
        let width = default_width.max(min_width);
        let height = default_height.max(min_height);

        Self {
            label,
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn builder(label: impl Into<String>) -> ButtonBuilder {
        ButtonBuilder::new(label)
    }

    pub fn next_y(&self) -> f32 {
        self.y + self.height + PADDING
    }

    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn is_selected(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_pos();

        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }

    pub fn is_clicked(&self) -> bool {
        self.is_selected() && mouse_clicked()
    }

    pub fn draw(&self) {
        let (text_width, text_height) = text_size(&self.label);
        let text_x = self.x + (self.width - text_width) / 2.0;
        let text_y = self.y + (self.height - text_height) / 2.0;

        let color = if self.is_selected() {
            self.color
        } else {
            mix_color(self.color, BLACK, 0.5)
        };

        draw_nine_slice(self.x, self.y, self.width, self.height);
        draw_text(text_x, text_y, &self.label, color);
    }
}

pub struct ButtonBuilder {
    x: f32,
    y: f32,
    min_width: f32,
    min_height: f32,
    label: String,
    color: Color,
}

impl ButtonBuilder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            label: label.into(),
            color: WHITE,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        self.min_width = width;
        self.min_height = height;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn build(self) -> Button {
        Button::new(
            self.label,
            self.x,
            self.y,
            self.min_width,
            self.min_height,
            self.color,
        )
    }
}
