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