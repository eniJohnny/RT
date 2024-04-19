use image::{Rgba, RgbaImage};
pub mod draw;

use crate::{display::draw_text, model::{maths::{hit, vec2::Vec2, vec3::Vec3}, scene::{self, Scene}, shapes::{sphere::{self, Sphere}, Shape}, Element}, render::raycasting::cast_ray, GUI_HEIGHT, GUI_WIDTH, SCREEN_WIDTH, SCREEN_WIDTH_U32};

pub fn get_line_position (i: u32, size: &Vec2) -> Vec2 {
    let x = SCREEN_WIDTH as f64 - size.x();
    let y = i as f64 * 26.;

    Vec2::new(x, y)
}

pub fn hide_gui (img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, scene: &scene::Scene) {
    let width = GUI_WIDTH;
    let x_start = img.width() - width;
    let height = GUI_HEIGHT;

    let rays = scene.camera().rays();

    for x in x_start..img.width() {
        for y in 0..height {
            img.put_pixel(x, y, cast_ray(scene, &rays[x as usize][y as usize]).to_rgba());
        }
    }
}


pub fn gui_clicked(pos: (f64, f64), gui: &Gui) -> bool {

    if gui == &Gui::new() {
        return false;
    }

    let x = pos.0 as u32;
    let y = pos.1 as u32;

    if x >= SCREEN_WIDTH_U32 - GUI_WIDTH && x <= SCREEN_WIDTH_U32 {
        if y <= GUI_HEIGHT {
            return true;
        }
    }

    false
}

pub fn hitbox_contains (hitbox: &(Vec2, Vec2), pos: (f64, f64)) -> bool {
    let x = pos.0 as u32;
    let y = pos.1 as u32;

    if x >= *hitbox.0.x() as u32 && x <= *hitbox.1.x() as u32 {
        if y >= *hitbox.0.y() as u32 && y <= *hitbox.1.y() as u32 {
            return true;
        }
    }

    false
}

pub struct TextFormat {
    size: Vec2,
    font_size: f32,
    font_color: Rgba<u8>,
    background_color: Rgba<u8>,
}

impl Default for TextFormat {
    fn default() -> Self {
        TextFormat {
            size: Vec2::new(0., 0.),
            font_size: 24.,
            font_color: Rgba([255, 255, 255, 255]),
            background_color: Rgba([50, 50, 50, 255]),
        }
    }
}

impl TextFormat {
    pub fn size(&self) -> &Vec2 { &self.size }
    pub fn font_size(&self) -> f32 { self.font_size }
    pub fn font_color(&self) -> &Rgba<u8> { &self.font_color }
    pub fn background_color(&self) -> &Rgba<u8> { &self.background_color }

    pub fn new(size: Vec2, font_size: f32, font_color: Rgba<u8>, background_color: Rgba<u8>) -> Self {
        Self {
            size,
            font_size,
            font_color,
            background_color,
        }
    }

    pub fn get_spacer(&self, text: &str, value: &str) -> String {
        let text_len = text.len();
        let value_len = value.len();
        let char_width = 10;
        let char_num = (*self.size.x() as usize) / char_width;
        let spacer_len = char_num - text_len - value_len - 5;

        " ".repeat(spacer_len)
    }

    pub fn parse_and_draw_text(&mut self, img: &mut RgbaImage, i: u32, text: &str, value: &str) -> (Vec2, Vec2){
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

#[derive(Debug, PartialEq)]
pub struct Gui {
    keys: Vec<String>,
    values: Vec<String>,
    hitboxes: Vec<(Vec2, Vec2)>,
    apply_hitbox: (Vec2, Vec2),
    cancel_hitbox: (Vec2, Vec2),
    element_index: usize,
    updating: bool,
    updating_index: usize,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            hitboxes: Vec::new(),
            apply_hitbox: (Vec2::new(1250., 540.), Vec2::new(1350., 580.)),
            cancel_hitbox: (Vec2::new(1440., 540.), Vec2::new(1550., 580.)),
            element_index: 0,
            updating: false,
            updating_index: 0,
        }
    }

    pub fn keys(&self) -> &Vec<String> { &self.keys }
    pub fn values(&self) -> &Vec<String> { &self.values }
    pub fn hitboxes(&self) -> &Vec<(Vec2, Vec2)> { &self.hitboxes }
    pub fn apply_hitbox(&self) -> &(Vec2, Vec2) { &self.apply_hitbox }
    pub fn cancel_hitbox(&self) -> &(Vec2, Vec2) { &self.cancel_hitbox }
    pub fn element_index(&self) -> usize { self.element_index }
    pub fn updating(&self) -> bool { self.updating }
    pub fn updating_index(&self) -> usize { self.updating_index }

    pub fn set_element_index(&mut self, index: usize) {
        self.element_index = index;
    }
    pub fn set_updating(&mut self, updating: bool) {
        self.updating = updating;
    }
    pub fn set_updating_index(&mut self, index: usize) {
        self.updating_index = index;
    }
    pub fn set_updates(&mut self, index: usize, value: &String, hitbox: &(Vec2, Vec2)) {
        self.values[index] = value.to_string();
        self.hitboxes[index] = hitbox.clone();
    }
}
