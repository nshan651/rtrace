use crate::vec3::*;
use crate::ray::Ray;

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc: Vec3 = r.origin - center;
    let a = Vec3::stat_dot(r.direction, r.direction);
    let b = 2.0 * Vec3::stat_dot(oc, r.direction);
    let c = Vec3::stat_dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    discriminant > 0.0
}

pub fn ray_color(r: &Ray) -> Color {
    if (hit_sphere(Point3(0,0,-1), 0.5, r))
        Color::new(1, 0, 0);
    let unit_dir: Vec3 = 
}
