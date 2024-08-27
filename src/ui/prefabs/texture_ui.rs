use std::sync::{Arc, RwLock};

use crate::{model::{materials::texture::{Texture, TextureType}, maths::vec3::Vec3, scene::Scene}, ui::{uielement::{Category, UIElement}, uisettings::UISettings, utils::{misc::{ElemType, Property, Value}, ui_utils::get_parent_ref}}};

use super::{file_ui::get_file_box, vector_ui::{get_vector_from_vector_ui, get_vector_ui}};

pub fn get_texture_ui(name: &str, texture: &Texture, submit: Box<impl Fn(Texture, &Arc<RwLock<Scene>>) + 'static>, settings: &UISettings) -> Vec<UIElement> {
    let mut elements_vec = vec![];
    let mut category = UIElement::new(name, name, ElemType::Category(Category::default()), settings);
    

    let as_file;
    let mut as_vec = Vec3::new(0., 0., 0.);
    let mut as_text = "".to_string();
    let texture_type = match texture {
        Texture::Value(value , ttype) => {
            as_file = false;
            as_vec = value.clone();
            ttype.clone()
        },
        Texture::Texture(value, ttype) => {
            as_file = true;
            as_text = value.clone();
            ttype.clone()
        }
    };
    {
        let texture_type = texture_type.clone();
        let as_file = as_file;
        let mut chk_file = UIElement::new("As file", "chk_file", ElemType::Property(
            Property::new(Value::Bool(as_file), 
                Box::new(move |elem, value, scene, ui| {
                    if let Some(elem) = elem {
                        if let Value::Bool(as_file) = value {
                            let parent_ref = get_parent_ref(elem.reference.clone());
                            if as_file {
                                let file_element_reference = parent_ref + ".as_file";
                                let file_element = ui.get_property_mut(&file_element_reference);
                                if let Some(property) = file_element {
                                    if let Value::Text(file) = &property.value {
                                        submit(Texture::Texture(file.clone(), texture_type.clone()), scene);
                                    }
                                }
                            } else {
                                let value_element_reference = parent_ref + ".as_value";
                                let value_element = ui.get_element_mut(value_element_reference);
                                if let Some(elem) = value_element {
                                    if let ElemType::Property(property) = &elem.elem_type {
                                        submit(Texture::from_value(&property.value), scene);
                                    } else {
                                        submit(Texture::from_vector(&"".to_string(), get_vector_from_vector_ui(elem, false)), scene);
                                    }
                                }
                            }
                        }
                    }
                })
                , Box::new(|_| Ok(())), settings)
        ), settings);
        chk_file.on_click = Some(Box::new(move |elem, _, ui| {
            let elem = elem.unwrap();
            if let ElemType::Property(property) = &elem.elem_type {
                if let Value::Bool(is_file) = property.value {
                    let parent_ref = get_parent_ref(elem.reference.clone());
                        let file_element_reference = parent_ref.clone() + ".as_file";
                        let file_element = ui.get_element_mut(file_element_reference).unwrap();
                        file_element.style.visible = is_file;
                        let value_element_reference = parent_ref + ".as_value";
                        let value_element = ui.get_element_mut(value_element_reference).unwrap();
                        value_element.style.visible = !is_file;
                }
            }
        }));
        category.add_element(chk_file);
    }
    let mut elem = match texture_type {
        TextureType::Float => {
            UIElement::new("Value", "as_value", ElemType::Property(Property::new(Value::Float(as_vec.to_value()), Box::new(|_, _, _, _| ()), Box::new(|_| Ok(())), settings)), settings)
        }
        TextureType::Vector => {
            get_vector_ui(as_vec, "Value", "as_value", settings, Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), false)
        }
        TextureType::Color => {
            get_vector_ui(as_vec, "Value", "as_value", settings, Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), true)
        }
        _ => panic!("There should not be a non float/vector/color texture")
    };
    elem.style.visible = !as_file;
    category.add_element(elem);
    let as_text = as_text;
    let name = name.to_string();
    let settings = settings.clone();
    let mut elem = UIElement::new("File", "as_file", ElemType::Property(Property::new(Value::Text(as_text.clone()), Box::new(|_, _, _, _| ()), Box::new(|_| Ok(())), &settings)), &settings);
    elem.on_click = Some(Box::new(move |elem, scene, ui| {
        if let Some(elem) = elem {
            let reference = elem.reference.clone();
            let file_box = get_file_box("./textures/".to_string(), name.clone(), Box::new(move |_, value, scene, ui| {
                let elem = ui.get_property_mut(&reference.clone()).unwrap();
                elem.value = value;
            }), &settings.clone(), as_text.clone());
            let box_reference = file_box.reference.clone();
            ui.add_box(file_box);
            ui.set_active_box(box_reference);
        }
    }));
    elem.style.visible = as_file;
    elem.style_mut().disabled = true;
    category.add_element(elem);


    elements_vec.push(category);
    elements_vec
}