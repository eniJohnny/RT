extern crate image;
use display::display_scene;
use gui::settings::ViewMode;
use image::flat::View;
use parsing::get_scene;
use winit::event::VirtualKeyCode;

pub mod display;
pub mod gui;
pub mod model;
pub mod parsing;
pub mod picker;
pub mod render;

const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const MAX_THREADS: usize = 1;
const BASE_SIMPLIFICATION: usize = 32;
const MAX_DEPTH: usize = 10;
const ANTIALIASING: f64 = 0.001;
const MAX_ITERATIONS: usize = 1;
const MAX_EMISSIVE: f64 = 100.;

/********* Default UISettings *********/
const MARGIN: usize = 3;
const GUI_WIDTH: u32 = 600;
const GUI_HEIGHT: u32 = 600;
const VIEW_MODE: ViewMode = ViewMode::HighDef;
const FIELD_PADDING_X: u32 = 10;
const FIELD_PADDING_Y: u32 = 3;
const INDENT_PADDING: u32 = 10;
const BASE_FONT_SIZE: u32 = 16;
const BUTTON_FONT_SIZE: u32 = 36;

const SCENE_FOLDER: &str = "scenes";
const PICKER_LINE_HEIGHT: f64 = 30.0;
// const SCENE: &str = "scenes/sphere.json";
const FPS: u64 = 20;

const RGB_KEYS: [&str; 3] = ["colr", "colg", "colb"];
const CAM_MOVE_KEYS: [VirtualKeyCode; 10] = [
    VirtualKeyCode::W,
    VirtualKeyCode::A,
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::Up,
    VirtualKeyCode::Left,
    VirtualKeyCode::Down,
    VirtualKeyCode::Right,
    VirtualKeyCode::Space,
    VirtualKeyCode::LShift,
];

pub fn run() {
    let path = String::from("scenes/testing.json");
    if path != "" {
        let mut scene = get_scene(&path);
        scene.add_skysphere_texture("skysphere.jpg");
        display_scene(scene);
    }
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}
