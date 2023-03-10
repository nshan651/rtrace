use serde::{Deserialize, Serialize};
use std::ops::{Neg, Add, Sub, Div, Mul};
use std::fmt;
use std::f64;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Vec3 {
    e: [f64; 3],
}

// Aliases
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 { e: [e1, e2, e3] }
    }
    /* Accessors */
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3::new(
            self.e[0] / self.length(), self.e[1] / self.length(), self.e[2] / self.length()
        )
    }

    /* Dot product */
    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.e[0] * rhs.e[0] +
        self.e[1] * rhs.e[1] +
        self.e[2] * rhs.e[2]
    }

    /* Cross product */
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new( 
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
        )
    }
}

/* Implement formatting */
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

/* Vector algrebra */
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ]
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ]
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs,
            ]
        }
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / rhs.e[0],
                self.e[1] / rhs.e[1],
                self.e[2] / rhs.e[2],
            ]
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / rhs,
                self.e[1] / rhs,
                self.e[2] / rhs,
            ]
        }
    }
}

/* Implementations for f64 */
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self * rhs.e[0],
                self * rhs.e[1],
                self * rhs.e[2],
            ]
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self / rhs.e[0],
                self / rhs.e[1],
                self / rhs.e[2],
            ]
        }
    }
}
