pub struct Point {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn build_point(x: f64, y: f64, z: f64) -> Point {
    Point {
        w: 1.0,
        x,
        y,
        z
    }
}