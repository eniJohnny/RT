use std::cell::{Ref, RefCell};

use image::RgbaImage;

use crate::{
    gui::{gui::Property, textformat::TextFormat, uisettings::UISettings, Gui},
    model::{materials::color::Color, scene::Scene},
    SCREEN_WIDTH,
};

use super::{
    ui::UI, uieditbar::UIEditBar, uielement::{ElemType, UIElement}, Position
};

pub struct UIBox {
    pub pos: (u32, u32),
    pub collapsed: bool,
    pub visible: bool,
    pub borders: Option<(Color, usize)>,
    pub background_color: Option<Color>,
    pub zindex: usize,
    pub margin: u32,
    pub elems: Vec<UIElement>,
    pub reference: String,
    pub edit_bar: Option<UIEditBar>,
}

impl UIBox {
    pub fn default(gui: &UI, reference: String) -> Self {
        UIBox {
            pos: (SCREEN_WIDTH as u32 - gui.settings().gui_width, 0),
            background_color: Some(Color::new(0.1, 0.1, 0.1)),
            borders: None,
            visible: true,
            collapsed: false,
            zindex: 1,
            margin: gui.settings().margin,
            elems: vec![],
            reference,
            edit_bar: None,
        }
    }

    pub fn add_elements(&mut self, mut elems: Vec<UIElement>) {
        self.elems.append(&mut elems);
    }

    pub fn set_edit_bar(&mut self, settings: &UISettings) {
        self.edit_bar = Some(UIEditBar::new(self.reference.clone(), settings))
    }

    pub fn validate_properties(&self, scene: &mut Scene, ui: &mut UI) -> Result<(), String> {
        for elem in &self.elems {
            elem.validate_properties()?;
        }
        Ok(())
    }

    pub fn height(&self, margin: u32) -> u32 {
        let mut top_height = 0;
        let mut bot_height = 0;
        let mut inline_height = 0;

        for elem in &self.elems {
            if !elem.visible {
                continue;
            }
            match elem.pos {
                Position::Relative(_, y) =>{
                    match y {
                        _ if y > 0 => {
                            let needed_height = y as u32 + elem.get_size(&elem.text, &elem.format).1 + margin;
                            if needed_height > top_height {
                                top_height = needed_height;
                            }
                        },
                        _ if y < 0 => {
                            let needed_height = -y as u32 + margin;
                            if needed_height > bot_height {
                                bot_height = needed_height;
                            }
                        },
                        _ => ()
                    }
                },
                Position::Inline => {
                    inline_height += elem.get_size(&elem.text, &elem.format).1 + margin;
                }
            }
        }
        if let Some(edit_bar) = &self.edit_bar {
            for elem in vec![&edit_bar.btn_apply, &edit_bar.btn_cancel, &edit_bar.txt_message] {
                if !elem.visible {
                    continue;
                }
                match elem.pos {
                    Position::Relative(_, y) =>{
                        match y {
                            _ if y > 0 => {
                                let needed_height = y as u32 + elem.get_size(&elem.text, &elem.format).1 + margin;
                                if needed_height > top_height {
                                    top_height = needed_height;
                                }
                            },
                            _ if y < 0 => {
                                let needed_height = -y as u32 + margin;
                                if needed_height > bot_height {
                                    bot_height = needed_height;
                                }
                            },
                            _ => ()
                        }
                    },
                    Position::Inline => {
                        inline_height += elem.get_size(&elem.text, &elem.format).1 + margin;
                    }
                }
            }
        }
        inline_height + top_height + bot_height
    }

    pub fn show(&self, ui: &mut UI) {
        ui.set_active_box(self.reference.clone());
    }
}
