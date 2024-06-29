use image::{Rgba, RgbaImage};

use crate::{
    display::utils::draw_text, model::maths::vec2::Vec2, FIELD_PADDING_X, FIELD_PADDING_Y,
    GUI_HEIGHT, GUI_WIDTH,
};

use super::{settings, uisettings::UISettings, utils::get_line_position};

pub struct Style {
    size: Vec2,
    pub font_size: f32,
    font_color: Rgba<u8>,
    background_color: Rgba<u8>,
    pub width: u32,
    pub height: u32,
    pub fill_width: bool,
    pub visible: bool,
    pub disabled: bool,
    pub bg_color: Option<Rgba<u8>>,
    pub border_radius: u32,
    pub padding_left: u32,
    pub padding_right: u32,
    pub padding_top: u32,
    pub padding_bot: u32,
}

pub struct StyleBuilder {
    style: Style,
    settings: UISettings,
}
pub trait Formattable {
    fn base_style(&self, settings: &UISettings) -> Style {
        Style::default(settings)
    }
}

impl StyleBuilder {
    pub fn default(settings: &UISettings) -> Self {
        Self {
            settings: settings.clone(),
            style: Style::default(settings),
        }
    }
    pub fn from_btn(settings: &UISettings) -> Self {
        StyleBuilder::default(settings)
            .bg_color(Some(Rgba([200, 200, 200, 255])))
            .font_color(Rgba([0, 0, 0, 255]))
            .border_radius(3)
    }
    pub fn padding_left(mut self, padding_left: u32) -> Self {
        self.style.padding_left = padding_left;
        self
    }
    pub fn padding_right(mut self, padding_right: u32) -> Self {
        self.style.padding_right = padding_right;
        self
    }
    pub fn padding_top(mut self, padding_top: u32) -> Self {
        self.style.padding_top = padding_top;
        self
    }
    pub fn padding_bot(mut self, padding_bot: u32) -> Self {
        self.style.padding_bot = padding_bot;
        self
    }
    pub fn fill_width(mut self, fill_width: bool) -> Self {
        self.style.fill_width = fill_width;
        self
    }
    pub fn padding(mut self, padding: u32) -> Self {
        self.padding_bot(padding)
            .padding_top(padding)
            .padding_left(padding)
            .padding_right(padding)
    }
    pub fn visible(mut self, visible: bool) -> Self {
        self.style.visible = visible;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.style.disabled = disabled;
        self
    }
    pub fn border_radius(mut self, border_radius: u32) -> Self {
        self.style.border_radius = border_radius;
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.style.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.style.height = height;
        self
    }
    pub fn bg_color(mut self, bg_color: Option<Rgba<u8>>) -> Self {
        self.style.bg_color = bg_color;
        self
    }
    pub fn font_color(mut self, font_color: Rgba<u8>) -> Self {
        self.style.font_color = font_color;
        self
    }
    pub fn build(self) -> Style {
        self.style
    }
}

impl Style {
    pub fn size(&self) -> &Vec2 {
        &self.size
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn font_color(&self) -> &Rgba<u8> {
        &self.font_color
    }
    pub fn background_color(&self) -> &Rgba<u8> {
        &self.background_color
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }
    pub fn set_font_color(&mut self, font_color: Rgba<u8>) {
        self.font_color = font_color;
    }
    pub fn set_background_color(&mut self, background_color: Rgba<u8>) {
        self.background_color = background_color;
    }

    pub fn default(settings: &UISettings) -> Self {
        Self {
            size: Vec2::new(0., 0.),
            width: 0,
            height: 0,
            font_size: settings.font_size as f32,
            font_color: Rgba([255, 255, 255, 255]),
            background_color: Rgba([89, 89, 89, 255]),
            bg_color: Some(Rgba([89, 89, 89, 255])),
            border_radius: 0,
            padding_left: settings.padding_x,
            padding_right: settings.padding_x,
            padding_top: settings.padding_y,
            padding_bot: settings.padding_y,
            visible: true,
            disabled: false,
            fill_width: false,
        }
    }

    pub fn row(settings: &UISettings) -> Self {
        StyleBuilder::default(settings)
            .padding(0)
            .fill_width(true)
            .bg_color(None)
            .build()
    }

    pub fn field_format(settings: &UISettings) -> Self {
        StyleBuilder::default(settings).fill_width(true).build()
    }

    pub fn editing(settings: &UISettings) -> Self {
        StyleBuilder::default(settings)
            .bg_color(Some(Rgba([255, 255, 255, 255])))
            .font_color(Rgba([0, 0, 0, 255]))
            .build()
    }

    pub fn category(settings: &UISettings) -> Self {
        StyleBuilder::default(settings)
            .fill_width(true)
            .bg_color(Some(Rgba([40, 40, 40, 255])))
            .font_color(Rgba([200, 200, 200, 255]))
            .build()
    }

    pub fn button(settings: &UISettings) -> Self {
        StyleBuilder::from_btn(settings).build()
    }

    pub fn btn_apply(settings: &UISettings) -> Self {
        StyleBuilder::from_btn(settings)
            .bg_color(Some(Rgba([70, 125, 70, 255])))
            .build()
    }

    pub fn btn_cancel(settings: &UISettings) -> Self {
        StyleBuilder::from_btn(settings)
            .bg_color(Some(Rgba([125, 70, 70, 255])))
            .build()
    }

    pub fn get_spacer(&self, text: &str, value: &str) -> String {
        let text_len = text.len();
        let value_len = value.len();
        let char_width = 10;
        let char_num = (*self.size.x() as usize) / char_width;
        let spacer_len = char_num - text_len - value_len - 5;

        " ".repeat(spacer_len)
    }

    pub fn parse_and_draw_text(
        &mut self,
        img: &mut RgbaImage,
        i: u32,
        text: &str,
        value: &str,
    ) -> (Vec2, Vec2) {
        let spacer = self.get_spacer(text, value);
        let pos = get_line_position(i, &self.size);

        if value == "" {
            draw_text(img, &pos, text.to_string(), self);
            return (Vec2::new(0., 0.), Vec2::new(0., 0.));
        }

        self.font_color = self.get_axis_color(text);
        draw_text(img, &pos, text.to_string(), self);

        let offset = (spacer.len() + text.len() + 3) as f64 * 10.0;
        let pos = Vec2::new(pos.x() + offset, *pos.y());
        self.font_color = Rgba([255, 255, 255, 255]);
        draw_text(img, &pos, value.to_string(), self);

        let end_pos = Vec2::new(pos.x() + (value.len() + 1) as f64 * 10.0, *pos.y() + 26.);
        (pos, end_pos)
    }

    pub fn get_axis_color(&self, text: &str) -> Rgba<u8> {
        match text {
            " X:" | " R:" => Rgba([255, 150, 150, 255]),
            " Y:" | " G:" => Rgba([150, 255, 150, 255]),
            " Z:" | " B:" => Rgba([150, 150, 255, 255]),
            _ => Rgba([255, 255, 255, 255]),
        }
    }
}
