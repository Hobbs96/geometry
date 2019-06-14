use core::ops::Add;
use core::ops::Sub;
use core::ops::Neg;
use core::ops::Mul;
use core::ops::Div;
use crate::float_cmp::ApproxEq;

#[derive(Copy, Clone)]
pub struct Vector {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn build_vector(x: f64, y: f64, z: f64) -> Vector {
    Vector {
        w: 0.0,
        x,
        y,
        z
    }
}

pub fn build_point(x: f64, y: f64, z: f64) -> Vector {
    Vector {
        w: 1.0,
        x,
        y,
        z
    }
}

impl Vector {
    pub fn is_point(&self) -> bool {
        if self.w.approx_eq(1.0, (0.0, 2)) {
            true
        }
        else {
            false
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            w: self.w * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            w: self.w / scalar,
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}