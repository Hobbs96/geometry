use crate::float_cmp::ApproxEq;
use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Neg;
use core::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn build_vector(x: f64, y: f64, z: f64) -> Vector {
    Vector { w: 0.0, x, y, z }
}

pub fn build_point(x: f64, y: f64, z: f64) -> Vector {
    Vector { w: 1.0, x, y, z }
}

impl Vector {
    pub fn is_point(&self) -> bool {
        if self.w.approx_eq(1.0, (0.0, 2)) {
            true
        } else {
            false
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            w: self.w / magnitude,
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        if self.is_point() {
            panic!("dot, the dot-product method, should not be called on a point");
        }
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        if self.is_point() {
            panic!("cross, the cross-product method, should not be called on a point");
        }
        Self {
            w: self.w,
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.w.approx_eq(other.w, (0.0, 2))
            && self.x.approx_eq(other.x, (0.0, 2))
            && self.y.approx_eq(other.y, (0.0, 2))
            && self.z.approx_eq(other.z, (0.0, 2))
    }
}

impl Eq for Vector {}

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
            z: self.z + other.z,
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
            z: -self.z,
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
            z: self.z * scalar,
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
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod vector_tests {
    use crate::float_cmp::ApproxEq;
    use crate::vector::build_point;
    use crate::vector::build_vector;

    #[test]
    fn point_w_is_one() {
        let origin = build_point(0.0, 0.0, 0.0);
        assert!(origin.is_point());
    }

    #[test]
    fn vector_plus_point() {
        let p1 = build_point(1.0, 1.0, 1.0);
        let v1 = build_vector(1.0, 1.0, 1.0);
        let p2 = p1 + v1;

        assert_eq!(p2, build_point(2.0, 2.0, 2.0));
        assert!(p2.is_point());
    }

    #[test]
    fn vector_plus_vector() {
        let v1 = build_vector(3.0, 2.0, 1.0);
        let v2 = build_vector(1.0, 2.0, 3.0);
        let v3 = v1 + v2;
        assert_eq!(v3, build_vector(4.0, 4.0, 4.0));
        assert!(!v3.is_point());
    }
    #[test]
    fn vector_w_is_zero() {
        let origin = build_vector(0.0, 0.0, 0.0);
        assert!(!origin.is_point());
    }

    #[test]
    fn values_set_correctly() {
        let v1 = build_vector(-1.7, 3.5, 0.2);
        assert_eq!(v1, build_vector(-1.7, 3.5, 0.2));
    }

    #[test]
    fn points_are_points() {
        let origin = build_point(0.0, 0.0, 0.0);

        assert!(origin.is_point());
    }

    #[test]
    fn point_minus_point() {
        let p1 = build_point(1.0, 2.9, 3.4);
        let p2 = build_point(3.4, 6.3, 2.7);
        let v1 = p1 - p2;
        let v2 = p2 - p1;

        assert_eq!(v1, build_vector(-2.4, -3.4, 0.7));
        assert!(!v1.is_point());

        assert_eq!(v2, build_vector(2.4, 3.4, -0.7));
        assert!(!v2.is_point())
    }

    #[test]
    fn subtract_from_zero_vector() {
        let v1 = build_vector(3.0, 4.0, 5.0);
        let origin = build_vector(0.0, 0.0, 0.0);
        let v2 = origin - v1;

        assert_eq!(v2, build_vector(-3.0, -4.0, -5.0));
        assert!(!v2.is_point());
    }

    #[test]
    fn negate_vector() {
        let v1 = build_vector(1.0, 2.0, 3.0);
        let v2 = -v1;

        assert_eq!(v2, build_vector(-1.0, -2.0, -3.0));
        assert!(!v2.is_point());
    }

    #[test]
    fn scalar_vector_multiplication() {
        let v1 = build_vector(1.0, 2.0, 3.0);
        let v2 = v1 * 3.0;

        assert_eq!(v2, build_vector(3.0, 6.0, 9.0));
        assert!(!v2.is_point());
    }

    #[test]
    fn fractional_scalar_vector_multiplication() {
        let v1 = build_vector(2.0, 1.0, 3.5);
        let v2 = v1 * 0.5;

        assert_eq!(v2, build_vector(1.0, 0.5, 1.75));
        assert!(!v2.is_point());
    }

    #[test]
    fn scalar_vector_division() {
        let v1 = build_vector(2.0, 1.0, 3.5);
        let v2 = v1 / 2.0;

        assert_eq!(v2, build_vector(1.0, 0.5, 1.75));
        assert!(!v2.is_point());
    }

    #[test]
    fn fractional_scalar_vector_division() {
        let v1 = build_vector(2.0, 1.0, 3.5);
        let v2 = v1 / 0.5;

        assert_eq!(v2, build_vector(4.0, 2.0, 7.0));
        assert!(!v2.is_point());
    }

    #[test]
    fn vector_magnitude() {
        let v1 = build_vector(0.0, 0.0, 1.0);
        let v2 = build_vector(0.0, 1.0, 0.0);
        let v3 = build_vector(1.0, 0.0, 0.0);
        let v4 = build_vector(3.0, 4.0, 5.0);
        let v5 = build_vector(-3.0, -4.0, -5.0);

        assert!(v1.magnitude().approx_eq(1.0, (0.0, 2)));
        assert!(v2.magnitude().approx_eq(1.0, (0.0, 2)));
        assert!(v3.magnitude().approx_eq(1.0, (0.0, 2)));
        assert!(v4.magnitude().approx_eq(50.0_f64.sqrt(), (0.0, 2)));
        assert!(v5.magnitude().approx_eq(50.0_f64.sqrt(), (0.0, 2)));
    }

    #[test]
    fn vector_normalization() {
        let v1 = build_vector(4.0, 0.0, 0.0);
        let v2 = v1.normalized();
        let v3 = build_vector(1.0, 2.0, 3.0);
        let v4 = v3.normalized();

        assert_eq!(v2, build_vector(1.0, 0.0, 0.0));
        assert!(v2.magnitude().approx_eq(1.0, (0.0, 2)));

        assert!(v4.magnitude().approx_eq(1.0, (0.0, 2)));
    }

    #[test]
    fn dot_product() {
        let v1 = build_vector(1.0, 2.0, 3.0);
        let v2 = build_vector(2.0, 3.0, 4.0);
        let v1_dot_v2 = v1.dot(v2);
        let v2_dot_v1 = v2.dot(v1);

        assert!(v1_dot_v2.approx_eq(20.0, (0.0, 2)));
        assert!(v2_dot_v1.approx_eq(20.0, (0.0, 2)));
    }

    #[test]
    #[should_panic]
    fn dot_product_panics_for_point() {
        let p1 = build_point(1.0, 2.0, 3.0);

        assert!(p1.dot(p1).approx_eq(14.0, (0.0, 2)));
    }

    #[test]
    fn cross_product() {
        let v1 = build_vector(1.0, 2.0, 3.0);
        let v2 = build_vector(2.0, 3.0, 4.0);
        let v3 = v1.cross(v2);
        let v4 = v2.cross(v1);
        let v5 = (-v1).cross(v2);
        let v6 = (-v2).cross(v1);

        assert_eq!(v3, build_vector(-1.0, 2.0, -1.0));
        assert_eq!(v4, build_vector(1.0, -2.0, 1.0));
        assert_eq!(v5, build_vector(1.0, -2.0, 1.0));
        assert_eq!(v6, build_vector(-1.0, 2.0, -1.0));
    }
}
