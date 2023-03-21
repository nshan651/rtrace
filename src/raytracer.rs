use crate::vec3::*;
use crate::ray::*;
use crate::sphere::*;
use crate::config::Config;

fn hit_world(
    world: &Vec<Sphere>,
    r: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}

pub fn ray_color(
    r: &Ray,
    scene: &Config,
) -> Color {
    let hit = hit_world(&scene.objects, r, 0.0, f64::MAX);
    
    match hit {
        Some(hit_record) => {
            0.5 * hit_record.normal + Color::new(1.0, 1.0, 1.0)
        }
        None => {
            let unit_direction: Vec3 = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
        }
    }
}

/*
pub fn ray_color(
    r: &Ray,
) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let norm: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5*Color::new(norm.x()+1.0, norm.y()+1.0, norm.z()+1.0);
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}
*/
