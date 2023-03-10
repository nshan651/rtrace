use crate::vec3::*;
use crate::ray::Ray;
use crate::sphere::hit_sphere;

pub fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let norm: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5*Color::new(norm.x()+1.0, norm.y()+1.0, norm.z()+1.0);
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}
