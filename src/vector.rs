use core::ops::Add;

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
        if self.w != 0.0 {
            true
        }
        else {
            false
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
/* TODO: need to impl for add, and should use the float-cmp crate (on github) */
//also need to add an is_point() method