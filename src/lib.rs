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
        assert_eq!(origin.w, 1.0);
    }

    #[test]
    fn vector_plus_point() {
        let p1 = build_point(1.0, 1.0, 1.0);
        let v1 = build_vector(1.0, 1.0, 1.0);
        let p2 = p1 + v1;

        assert!(p2.x.approx_eq(2.0, (0.0, 2)));
        assert!(p2.y.approx_eq(2.0, (0.0, 2)));
        assert!(p2.z.approx_eq(2.0, (0.0, 2)));
        assert!(p2.w.approx_eq(1.0, (0.0, 2)));
    }

    #[test]
    fn vector_plus_vector() {
        let v1 = build_vector(3.0, 2.0, 1.0);
        let v2 = build_vector(1.0, 2.0, 3.0);
        let v3 = v1 + v2;
        assert!(v3.x.approx_eq(4.0, (0.0, 2)));
        assert!(v3.y.approx_eq(4.0, (0.0, 2)));
        assert!(v3.z.approx_eq(4.0, (0.0, 2)));
        assert!(v3.w.approx_eq(0.0, (0.0, 2)));
    }
    #[test]
    fn vector_w_is_zero() {
        let origin = build_vector(0.0, 0.0, 0.0);
        assert_eq!(origin.w, 0.0);
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
        let p4 = p2 - p1;

        //currently failing because I'm not using a fuzzy comp
        assert!(v1.x.approx_eq(-2.4, (0.0, 2)));
        assert!(v1.y.approx_eq(-3.4, (0.0, 2)));
        assert!(v1.z.approx_eq(0.7, (0.0, 2)));
        assert!(v1.w.approx_eq(0.0, (0.0, 2)));
        assert!(!v1.is_point());
    }
}
