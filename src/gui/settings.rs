use rusttype::Font;

use crate::{
    model::{
        materials::color::Color,
        maths::vec3::Vec3,
        objects::light::{AmbientLight, ParallelLight},
    },
    ANTIALIASING, BASE_FONT_SIZE, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT, GUI_WIDTH,
    INDENT_PADDING, MARGIN, MAX_DEPTH, MAX_ITERATIONS, VIEW_MODE,
};

use super::{
    elements::{
        ui::UI,
        uielement::{Category, ElemType, Property, UIElement, Value},
        Displayable,
    },
    uisettings::UISettings,
};

#[derive(Debug)]
pub enum ViewMode {
    Simple(Color, ParallelLight),
    Norm,
    HighDef,
    BVH,
}

#[derive(Debug)]
pub struct Settings {
    pub reflections: bool,
    pub indirect: bool,
    pub iterations: usize,
    pub depth: usize,
    pub anti_alisaing: f64,
    pub view_mode: ViewMode,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            view_mode: VIEW_MODE,
            reflections: true,
            indirect: true,
            iterations: MAX_ITERATIONS,
            depth: MAX_DEPTH,
            anti_alisaing: ANTIALIASING,
        }
    }
}

impl Displayable for Settings {
    fn get_fields(&self, reference: &String, settings: &UISettings) -> Vec<UIElement> {
        let mut category = Category::default();
        category.elems.push(UIElement::new(
            "Iterations",
            "iterations",
            ElemType::Property(Property::new(
                Value::Unsigned(self.iterations as u32),
                Box::new(|value, scene, ui| {
                    if let Value::Unsigned(value) = value {
                        scene.settings_mut().iterations = value as usize;
                        scene.set_dirty(true);
                    }
                }),
                Box::new(|_| Ok(())),
                settings,
            )),
            settings,
        ));
        category.elems.push(UIElement::new(
            "Reflections",
            "chk_reflect",
            ElemType::Property(Property::new(
                Value::Bool(self.reflections),
                Box::new(|value, scene, ui| {
                    if let Value::Bool(value) = value {
                        scene.settings_mut().reflections = value;
                        scene.set_dirty(true);
                    }
                }),
                Box::new(|_| Ok(())),
                settings,
            )),
            settings,
        ));

        vec![UIElement::new(
            "Render settings",
            "settings",
            ElemType::Category(category),
            settings,
        )]
    }
}
