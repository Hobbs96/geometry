extern crate float_cmp;
pub mod vector;

#[cfg(test)]
mod vector_tests {
    use crate::vector::build_point;
    use crate::vector::build_vector;
    use crate::float_cmp::ApproxEq;

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

        assert!(p2.x.approx_eq(2.0, (0.0, 2)));
        assert!(p2.y.approx_eq(2.0, (0.0, 2)));
        assert!(p2.z.approx_eq(2.0, (0.0, 2)));
        assert!(p2.is_point());
    }

    #[test]
    fn vector_plus_vector() {
        let v1 = build_vector(3.0, 2.0, 1.0);
        let v2 = build_vector(1.0, 2.0, 3.0);
        let v3 = v1 + v2;
        assert!(v3.x.approx_eq(4.0, (0.0, 2)));
        assert!(v3.y.approx_eq(4.0, (0.0, 2)));
        assert!(v3.z.approx_eq(4.0, (0.0, 2)));
        assert!(!v3.is_point());
    }
    #[test]
    fn vector_w_is_zero() {
        let origin = build_vector(0.0, 0.0, 0.0);
        assert!(!origin.is_point());
    }

    #[test]
    fn values_set_correctly() {
        let vector = build_vector(-1.7, 3.5, 0.2);
        assert_eq!(vector.x, -1.7);
        assert_eq!(vector.y, 3.5);
        assert_eq!(vector.z, 0.2);
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

        //currently failing because I'm not using a fuzzy comp
        assert!(v1.x.approx_eq(-2.4, (0.0, 2)));
        assert!(v1.y.approx_eq(-3.4, (0.0, 2)));
        assert!(v1.z.approx_eq(0.7, (0.0, 2)));
        assert!(!v1.is_point());

        assert!(v2.x.approx_eq(2.4, (0.0, 2)));
        assert!(v2.y.approx_eq(3.4, (0.0, 2)));
        assert!(v2.z.approx_eq(-0.7, (0.0, 2)));
        assert!(!v2.is_point())
    }

    #[test]
    fn subtract_from_zero_vector() {
       let v1 = build_vector(3.0, 4.0, 5.0);
       let origin = build_vector(0.0, 0.0, 0.0);
       let v2 = origin - v1;

       assert!(v2.x.approx_eq(-3.0, (0.0, 2)));
       assert!(v2.y.approx_eq(-4.0, (0.0, 2)));
       assert!(v2.z.approx_eq(-5.0, (0.0, 2)));
       assert!(!v2.is_point());
    }

    #[test]
    fn negate_vector() {
        let v1 = build_vector(1.0, 2.0, 3.0);
        let v2 = -v1;
        
        assert!(v2.x.approx_eq(-1.0, (0.0, 2)));
        assert!(v2.y.approx_eq(-2.0, (0.0, 2)));
        assert!(v2.z.approx_eq(-3.0, (0.0, 2)));
        assert!(!v2.is_point());
    }

    #[test]
    fn scalar_vector_multiplication() {
        let v1 = build_vector(1.0, 2.0, 3.0);
        let v2 = v1 * 3.0;

        assert!(v2.x.approx_eq(3.0, (0.0, 2)));
        assert!(v2.y.approx_eq(6.0, (0.0, 2)));
        assert!(v2.z.approx_eq(9.0, (0.0, 2)));
        assert!(!v2.is_point());
    }

    #[test]
    fn fractional_scalar_vector_multiplication() {
        let v1 = build_vector(2.0, 1.0, 3.5);
        let v2 = v1 * 0.5;

        assert!(v2.x.approx_eq(1.0, (0.0, 2)));
        assert!(v2.y.approx_eq(0.5, (0.0, 2)));
        assert!(v2.z.approx_eq(1.75, (0.0, 2)));
        assert!(!v2.is_point());
    }

    #[test]
    fn scalar_vector_division() {
        let v1 = build_vector(2.0, 1.0, 3.5);
        let v2 = v1 / 2.0;

        assert!(v2.x.approx_eq(1.0, (0.0, 2)));
        assert!(v2.y.approx_eq(0.5, (0.0, 2)));
        assert!(v2.z.approx_eq(1.75, (0.0, 2)));
        assert!(!v2.is_point());
    }

    #[test]
    fn fractional_scalar_vector_division() {
        let v1 = build_vector(2.0, 1.0, 3.5);
        let v2 = v1 / 0.5;

        assert!(v2.x.approx_eq(4.0, (0.0, 2)));
        assert!(v2.y.approx_eq(2.0, (0.0, 2)));
        assert!(v2.z.approx_eq(7.0, (0.0, 2)));
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

        assert!(v2.x.approx_eq(1.0, (0.0, 2)));
        assert!(v2.y.approx_eq(0.0, (0.0, 2)));
        assert!(v2.z.approx_eq(0.0, (0.0, 2)));
        assert!(v2.magnitude().approx_eq(1.0, (0.0, 2)));

        assert!(v4.magnitude().approx_eq(1.0, (0.0, 2)));
    }

    #[test]
    fn dot_product() {
        let v1 = build_vector(1.0, 2.0, 3.0);
        let v2 = build_vector(2.0, 3.0, 4.0);
        let v1_dot_v2 = v1.dot(v2);

        assert!(v1_dot_v2.approx_eq(20.0, (0.0, 2)));
    }

    #[test]
    #[should_panic]
    fn dot_product_panics_for_point() {
        let p1 = build_point(1.0, 2.0, 3.0);

        assert!(p1.dot(p1).approx_eq(14.0, (0.0, 2)));
    }
}
