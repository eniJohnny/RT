use std::f64::consts::PI;

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::scene::Scene;
use crate::model::Element;
use crate::render::raycasting::get_closest_hit_from_t;

#[derive(Debug)]
pub struct Sphere {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
}

impl Shape for Sphere {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        // intersection rayon/sphere
        let dist = &self.pos - r.get_pos();
        let dot_product = r.get_dir().dot(&dist);
        let discriminant =
            &dot_product * &dot_product - &dist.dot(&dist) + &self.radius * &self.radius;
        if discriminant < 0.0 {
            return None;
        }
        let intersection1 = &dot_product - &discriminant.sqrt();
        let intersection2 = &dot_product + &discriminant.sqrt();
        if intersection1 > 0.0 {
            return Some(Vec::from([intersection1, intersection2]));
        }
        None
    }

	fn outer_intersect(&self, r: &Ray, factor: f64, displaced_factor: f64) -> Option<Vec<f64>> {
		let mut outer_sphere = self.clone();
		outer_sphere.set_radius(outer_sphere.radius() + outer_sphere.radius() * displaced_factor * factor);
		outer_sphere.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
		let displaced_factor = 0.1;
		let total_displacement = self.radius * displaced_factor;
		let step = 0.1;

		let mut current_step = 1.;
		let mut t = self.outer_intersect(ray, current_step, displaced_factor);
		if let Some(mut hit) = get_closest_hit_from_t(scene, ray, &t, element) {
			let mut sphere_to_hit = hit.pos() - self.pos();
			let mut hit_distance = sphere_to_hit.length() - self.radius;
			let mut hit_ratio = hit_distance / total_displacement;

			let mut displaced_ratio = hit.map_texture(element.material().displacement(), scene.textures()).to_value();
			let mut old_hit_ratio = hit_ratio + 1.;
			let mut old_displaced_ratio = displaced_ratio;
			while displaced_ratio < hit_ratio && old_hit_ratio > hit_ratio {
				let mut displaced_dist = (hit_ratio - displaced_ratio) * total_displacement;
				hit = Hit::new(
					element,
					displaced_dist,
					hit.pos() + ray.get_dir() * displaced_dist,
					ray.get_dir(),
					scene.textures()
				);
				sphere_to_hit = hit.pos() - self.pos();
				hit_distance = sphere_to_hit.length() - self.radius;
				hit_ratio = hit_distance / total_displacement;

				current_step -= step;
				if hit_ratio < current_step {
					let mut difference = current_step - hit_ratio;
					while difference > 0.01 {
						let difference_dist = difference * total_displacement;
						hit = Hit::new(element,
							hit.dist() - difference_dist,
							hit.pos() - difference_dist * ray.get_dir(),
							ray.get_dir(),
							scene.textures()
						);
						sphere_to_hit = hit.pos() - self.pos();
						hit_distance = sphere_to_hit.length() - self.radius;
						hit_ratio = hit_distance / total_displacement;
						difference = current_step - hit_ratio;
					}
				}
				old_displaced_ratio = displaced_ratio;
				displaced_ratio = hit.map_texture(element.material().displacement(), scene.textures()).to_value();
				old_hit_ratio = hit_ratio;
			}
			return self.outer_intersect(ray, (displaced_ratio + old_displaced_ratio) / 2., displaced_factor)
		}
		t
	}

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();

        let constant_axis: Vec3;
        if *hit.norm() == Vec3::new(0., 0., 1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }
        projection.v = ((self.dir.dot(&hit.norm()) + 1.) / 2.);
        projection.i = hit.norm().cross(&constant_axis).normalize();
        projection.j = hit.norm().cross(&projection.i).normalize();
        projection.k = hit.norm().clone();
        let constant_axis: Vec3;
        if self.dir == Vec3::new(0., 0., 1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }
        projection.v = ((self.dir.dot(&hit.norm()) + 1.) / 2.).clamp(0., 1.);
        projection.i = self.dir.cross(&constant_axis).normalize();
        projection.j = self.dir.cross(&projection.i).normalize();
        projection.k = hit.norm().clone();
        let i_component: f64 = hit.norm().dot(&projection.i);
        let j_component: f64 = hit.norm().dot(&projection.j);
        let k_component: f64 = hit.norm().dot(&self.dir);
        projection.u = (f64::atan2(i_component, j_component) + PI) / (2. * PI);
        projection.v = f64::acos(k_component) / PI;
        projection.i = hit.norm().cross(&self.dir).normalize();
        projection.j = hit.norm().cross(&projection.i).normalize();
        projection
    }

    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        (hit_position - self.pos()).normalize()
    }

    fn as_sphere(&self) -> Option<&Sphere> {
        Some(self)
    }

    fn as_plane(&self) -> Option<&super::plane::Plane> {
        None
    }

    fn as_cylinder(&self) -> Option<&super::cylinder::Cylinder> {
        None
    }

    fn as_cone(&self) -> Option<&super::cone::Cone> {
        None
    }
    fn pos(&self) -> &Vec3 {
        &self.pos
    }
}

impl Sphere {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64) -> Sphere {
        self::Sphere { pos, dir, radius }
    }

    // Methods
    pub fn clone(&self) -> Sphere {
        let pos = Vec3::new(*self.pos.x(), *self.pos.y(), *self.pos.z());
        let dir = Vec3::new(*self.dir.x(), *self.dir.y(), *self.dir.z());
        self::Sphere {
            pos: pos,
            dir: dir,
            radius: self.radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::maths::ray::Ray;
    use crate::model::maths::vec3::Vec3;
    use crate::model::shapes::sphere::Sphere;
    use crate::model::shapes::Shape;

    #[test]
    fn test_intersect() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(-5., 0., 0.), Vec3::new(1., 0., 0.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([4., 6.])));
    }

    #[test]
    fn test_intersect2() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 2.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([1., 3.])));
    }

    #[test]
    fn test_intersect3() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 2.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), 5);
        assert_eq!(s1.intersect(&r1), None);
    }
}
