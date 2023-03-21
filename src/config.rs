use serde::{Deserialize, Serialize};
// use crate::vec3::Vec3;
use crate::sphere::Sphere;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub objects: Vec<Sphere>,
}
