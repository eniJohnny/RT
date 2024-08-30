use std::sync::{Arc, RwLock};

use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::scene::Scene;
use crate::model::shapes::plane::Plane;
use crate::model::shapes::triangle::Triangle;
use crate::model::Element;
use crate::ui::prefabs::shape_ui::ShapeUI;
use crate::ui::prefabs::vector_ui::get_vector_ui;
use crate::ui::ui::UI;
use crate::ui::uielement::{Category, UIElement};
use crate::ui::utils::misc::{ElemType, Property, Value};
use super::Shape;

#[derive(Debug)]
pub struct Rectangle {
    pos: Vec3,
    length: f64,
    width: f64,
    dir_l : Vec3,
    dir_w : Vec3,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    plane: Plane
}

impl Shape for Rectangle {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {

        let mut intersection: f64;
        match self.plane.intersect(r) {
            Some(intersections) => {
                intersection = intersections[0];
            },
            _ => {
                return None;
            }
        }

        let p = intersection * r.get_dir() + r.get_pos();

        if Triangle::inside_triangle(&p, &self.d, &self.b, &self.c, &self.plane.dir()) || Triangle::inside_triangle(&p, &self.a, &self.b, &self.c, &self.plane.dir()) {
            return Some(Vec::from([intersection]));
        }
        None
    }

	fn outer_intersect(&self, r: &Ray, displaced_factor: f64) -> Option<Vec<f64>> {
		self.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
		self.intersect(ray)
	}

    fn projection(&self, hit: &Hit) -> Projection {
        Projection::default()
    }
    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        self.plane.norm(hit, ray_dir)
    }
    fn pos(&self) -> &Vec3 { &self.pos }
    fn as_rectangle(&self) -> Option<&Rectangle> { Some(self) }

    fn get_ui(&self, element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Rectangle", "rectangle", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(rectangle) = element.shape().as_rectangle() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(rectangle.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.pos.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.pos.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.pos.set_z(value);
                        }
                    }
                }),
                true));
            category.add_element(get_vector_ui(rectangle.dir_l.clone(), "Direction 1", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, scene, ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.dir_l.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.dir_l.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.dir_l.set_z(value);
                        }
                    }
                }),
                true));
                category.add_element(get_vector_ui(rectangle.dir_l.clone(), "Direction 2", "dir2", &ui.uisettings_mut(),
                Box::new(move |_, value, scene, ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.dir_w.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.dir_w.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                        if let Value::Float(value) = value {
                            rectangle.dir_w.set_z(value);
                        }
                    }
                }),
                true));
            category.add_element(UIElement::new(
                "Width",
                "width", 
                ElemType::Property(Property::new(
                    Value::Float(rectangle.width), 
                    Box::new(move |_, value, scene, _| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                            if let Value::Float(value) = value {
                                rectangle.set_width(value);
                            }
                        }
                    }),
                    Box::new(|_| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            category.add_element(UIElement::new(
                "Length",
                "length", 
                ElemType::Property(Property::new(
                    Value::Float(rectangle.length), 
                    Box::new(move |_, value, scene, _| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(rectangle) = elem.shape_mut().as_rectangle_mut() {
                            if let Value::Float(value) = value {
                                rectangle.set_length(value);
                            }
                        }
                    }),
                    Box::new(|_| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }

        category
    }
}

impl Rectangle {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn length(&self) -> &f64 { &self.length }
    pub fn width(&self) -> &f64 { &self.width }
    pub fn dir_l(&self) -> &Vec3 { &self.dir_l }
    pub fn dir_w(&self) -> &Vec3 { &self.dir_w }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_length(&mut self, length: f64) { self.length = length }
    pub fn set_width(&mut self, width: f64) { self.width = width }
    pub fn set_dir_l(&mut self, dir_l: Vec3) { self.dir_l = dir_l }
    pub fn set_dir_w(&mut self, dir_w: Vec3) { self.dir_w = dir_w }

    // Constructor
    pub fn new(pos: Vec3, length: f64, width: f64, dir_l: Vec3, dir_w: Vec3) -> Rectangle {
        let l_gap = (&dir_l.clone().normalize() * length) / 2.;
        let w_gap = (&dir_w.clone().normalize() * width) / 2.;
        let a = pos.clone() + &l_gap + &w_gap;
        let b= pos.clone() - &l_gap + &w_gap;
        let c= pos.clone() + &l_gap - &w_gap;
        let d= pos.clone() - &l_gap - &w_gap;
        let plane = Plane::new(a.clone(), dir_l.clone().cross(&dir_w).normalize());

        Rectangle { pos, length, width, dir_l, dir_w, a, b, c, d, plane }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::maths::ray::Ray;

    #[test]
    fn test_rectangle_intersect() {
        let r = Rectangle::new(Vec3::new(0., 0., 1.), 2., 2., Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.));
        let ray = Ray::new(Vec3::new(-0.1, 0., 0.), Vec3::new(0., 0., 1.), 5);
        let intersections = r.intersect(&ray);
        assert_eq!(intersections, Some(vec![1.]));
    }
}