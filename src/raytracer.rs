use std::io::{Write, stderr, stdout, BufWriter};
use crate::vec3::*;
use crate::ray::Ray;

pub fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
   
    let c1: Color = Color::new(1.0, 1.0, 1.0) * (1.0 - t);
    let c2: Color = Color::new(0.5, 0.7, 1.0) * t;
    
    c1 + c2
}
