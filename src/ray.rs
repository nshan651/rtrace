use crate::vec3::{Vec3, Point3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    // fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool;
}
