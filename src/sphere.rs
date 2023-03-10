use crate::vec3::*;
use crate::ray::{Ray, Hittable};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let oc: Vec3 = r.origin - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }


    }
}

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    
    if discriminant < 0.0 {
        return -1.0
    }
    else {
        return (-half_b - discriminant.sqrt()) / a
    }
}
