use crate::constants::{PADDING, TEXT_SIZE};
use crate::engine_v2::*;

const LINE_HEIGHT: f32 = TEXT_SIZE as f32;
const METER_WIDTH: f32 = 100.0;
const METER_PADDING: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct Panel {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    title: Option<(String, Color)>,
    lines: Vec<Line>,
}

impl Panel {
    fn new(
        x: f32,
        y: f32,
        min_width: f32,
        min_height: f32,
        title: Option<(String, Color)>,
        lines: Vec<Line>,
    ) -> Self {
        let mut content_width: f32 = 0.0;
        let mut content_height: f32 = PADDING;

        if let Some((title_text, _)) = &title {
            let title_width = text_size(title_text).0 + PADDING * 2.0;
            content_width = content_width.max(title_width);
            content_height += TEXT_SIZE as f32 / 2.0;
        }

        for line in &lines {
            match line {
                Line::TextLine(text) => {
                    content_width = content_width.max(text.width + PADDING * 2.0);
                }
                Line::MeterLine(meter) => {
                    content_width = content_width.max(meter.width + PADDING * 2.0);
                }
            }
            content_height += LINE_HEIGHT + PADDING;
        }

        let width = min_width.max(content_width);
        let height = min_height.max(content_height);

        Self {
            x,
            y,
            width,
            height,
            title,
            lines,
        }
    }

    pub fn builder() -> PanelBuilder {
        PanelBuilder::new()
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn selected_index(&self) -> Option<usize> {
        let (mouse_x, mouse_y) = mouse_pos();
        if !self.in_bounds(mouse_x, mouse_y) {
            return None;
        }

        let y = self.starting_y();

        let selected_line = self.lines.iter().enumerate().find_map(|(i, line)| {
            let line_y = y + i as f32 * (LINE_HEIGHT + PADDING);
            (mouse_y >= line_y && mouse_y <= line_y + LINE_HEIGHT).then_some(line)
        });

        selected_line.and_then(|line| {
            if let Line::TextLine(text) = line {
                text.select_index
            } else {
                None
            }
        })
    }

    pub fn draw(&self) {
        draw_nine_slice(self.x, self.y, self.width, self.height);

        if let Some((title, color)) = &self.title {
            let (text_width, text_height) = text_size(title);
            let text_x = self.x + PADDING;
            let text_y = self.y - text_height / 2.0;
            draw_rectangle(text_x, text_y, text_width, text_height, BLACK);
            draw_text(text_x, text_y, title, *color);
        }

        let selected_index = self.selected_index();
        let mut y = self.starting_y();

        for line in &self.lines {
            match line {
                Line::TextLine(text) => self.draw_text(y, text, selected_index),
                Line::MeterLine(meter) => self.draw_meter(y, meter),
            }
            y += LINE_HEIGHT + PADDING;
        }
    }

    fn draw_text(&self, y: f32, text: &Text, selected_index: Option<usize>) {
        let color = if text.select_index.is_none() || text.select_index == selected_index {
            text.color
        } else {
            mix_color(text.color, BLACK, 0.5)
        };

        draw_text(self.x + PADDING, y, &text.text, color);
    }

    fn draw_meter(&self, y: f32, meter: &Meter) {
        let label_x = self.x + PADDING;
        let value_x = self.x + PADDING + meter.label_width + PADDING;

        let total_padding = (meter.max_value - 1).max(0) as f32 * METER_PADDING;
        let pip_width = (METER_WIDTH - total_padding) as f32 / meter.max_value as f32;
        let pip_height = LINE_HEIGHT * 0.75;

        if let Some((label_text, label_color)) = &meter.label {
            draw_text(label_x, y, label_text, *label_color);
        }

        for i in 0..meter.max_value as usize {
            let pip_x = value_x + i as f32 * (pip_width + METER_PADDING);

            let color = if i < (meter.value - meter.diff_value) as usize {
                meter.color
            } else if i < meter.value as usize {
                mix_color(meter.color, WHITE, 0.5)
            } else {
                mix_color(meter.color, BLACK, 0.5)
            };

            draw_rectangle(pip_x, y, pip_width, pip_height, color);
        }
    }

    fn starting_y(&self) -> f32 {
        if self.title.is_some() {
            self.y + TEXT_SIZE as f32 / 2.0 + PADDING
        } else {
            self.y + PADDING
        }
    }

    fn in_bounds(&self, mouse_x: f32, mouse_y: f32) -> bool {
        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }
}

#[derive(Debug, Clone)]
struct Text {
    text: String,
    color: Color,
    width: f32,
    select_index: Option<usize>,
}

#[derive(Debug, Clone)]
struct Meter {
    label: Option<(String, Color)>,
    value: u16,
    diff_value: u16,
    max_value: u16,
    color: Color,
    label_width: f32,
    width: f32,
}

#[derive(Debug, Clone)]
enum Line {
    TextLine(Text),
    MeterLine(Meter),
}

pub struct PanelBuilder {
    x: f32,
    y: f32,
    min_width: f32,
    min_height: f32,
    title: Option<(String, Color)>,
    lines: Vec<Line>,
    next_select_index: usize,
}

impl PanelBuilder {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            title: None,
            lines: Vec::new(),
            next_select_index: 0,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.min_width = width;
        self.min_height = height;
        self
    }

    pub fn title(mut self, title: impl Into<String>, color: Color) -> Self {
        self.title = Some((title.into(), color));
        self
    }

    pub fn text(mut self, text: impl Into<String>, color: Color) -> Self {
        let str: String = text.into();
        let (width, _) = text_size(&str);

        self.lines.push(Line::TextLine(Text {
            text: str,
            color,
            width,
            select_index: None,
        }));
        self
    }

    pub fn selectable_text(mut self, text: impl Into<String>, color: Color) -> Self {
        let str: String = text.into();
        let (width, _) = text_size(&str);

        self.lines.push(Line::TextLine(Text {
            text: str,
            color,
            width,
            select_index: Some(self.next_select_index),
        }));
        self.next_select_index += 1;
        self
    }

    pub fn meter(mut self, value: u16, diff_value: u16, max_value: u16, color: Color) -> Self {
        let label = None;
        let label_width = 0.0;

        self.lines.push(Line::MeterLine(Meter {
            label,
            value,
            diff_value,
            max_value,
            color,
            label_width,
            width: METER_WIDTH,
        }));
        self
    }

    pub fn labeled_meter(
        mut self,
        label: impl Into<String>,
        value: u16,
        diff_value: u16,
        max_value: u16,
        color: Color,
    ) -> Self {
        let label_text: String = label.into();
        let (label_width, _) = text_size(&label_text);

        self.lines.push(Line::MeterLine(Meter {
            label: Some((label_text, color)),
            value,
            diff_value,
            max_value,
            color,
            label_width,
            width: METER_WIDTH + label_width + PADDING,
        }));
        self
    }

    pub fn build(self) -> Panel {
        Panel::new(
            self.x,
            self.y,
            self.min_width,
            self.min_height,
            self.title,
            self.lines,
        )
    }
}
