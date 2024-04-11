use crate::gui::draw_gui;
use crate::{model::{materials::Color, maths::{hit::Hit, ray::Ray}, Scene}, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::events::{Model, event, model};
use nannou::prelude::*;

// pub mod camera;
pub mod lighting;

pub fn display_scene(scene: &Scene) {
    nannou::app(model).view(view).event(event).run();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_gui(&draw);

    draw.to_frame(app, &frame).unwrap();
}

// pub fn render_scene(scene: &Scene) {
//     let camera = scene.camera();
//     let rays = camera.get_rays();
//     let mut image: Vec<Vec<Color>> = vec![];

//     for x in [0, SCREEN_WIDTH] {
//         let mut line: Vec<Color> = vec![];
//         for y in [0, SCREEN_HEIGHT] {
//             line.push(cast_ray(scene, &rays[x][y]))
//         }
//         image.push(line)
//     }
// }

pub fn show_scene(scene: &Scene) {
    nannou::app(model).view(view).event(event).run();
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => unimplemented!(),
        None => Color {r: 0, g: 0, b: 0}
    }
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;
    for element in scene.elements().iter() {
        if let Some(hit) = element.shape().intersect(ray) {
            if let Some(closest_hit) = &closest{
                if hit.dist() < closest_hit.dist() {
                    closest = Some(hit);
                }
            }
        }
    }
    closest
}
