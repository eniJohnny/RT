use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use super::HasShape;

pub struct Plane {
    pos: Vec3,
    dir: Vec3,
}

impl HasShape for Plane {
    fn distance(&self) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, vector: &Ray) -> Option<Hit> {
        unimplemented!()
    }
    fn projection(&self, hit: &Hit) -> Option<(i32, i32)> {
        unimplemented!()
    }
}

impl Plane {
    // Accessors
    pub fn get_pos(&self) -> Vec3 { Vec3::new(self.pos.x().to_owned(), self.pos.y().to_owned(), self.pos.z().to_owned()) }
    pub fn get_dir(&self) -> Vec3 { Vec3::new(self.dir.x().to_owned(), self.dir.y().to_owned(), self.dir.z().to_owned()) }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3) -> Plane{
        self::Plane { pos, dir }
    }

}