use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

use super::Shape;

pub struct Sphere {
    pos: Vec3,
    dir: Vec3,
    radius: f64
}

impl Shape for Sphere {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    
    fn intersect(&self, vector: &Ray) -> Option<Hit> {
        unimplemented!()
    }

    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
}

impl Sphere {
    // Accessors
    pub fn get_pos(&self) -> Vec3 { Vec3::new(self.pos.x().to_owned(), self.pos.y().to_owned(), self.pos.z().to_owned()) }
    pub fn get_dir(&self) -> Vec3 { Vec3::new(self.dir.x().to_owned(), self.dir.y().to_owned(), self.dir.z().to_owned()) }
    pub fn get_radius(&self) -> f64 { self.radius.to_owned() }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64) -> Sphere{
        self::Sphere { pos, dir, radius }
    }

}