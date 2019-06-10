pub mod vector;

#[cfg(test)]
mod vector_tests {
    use crate::vector::build_vector;
    use crate::vector::build_point;

    #[test]
    fn point_w_is_one() {
        let origin = build_point(0.0, 0.0, 0.0);
        assert_eq!(origin.w, 1.0);
    }

    #[test]
   fn points_compare_equal() {
       let p1 = build_point(1.0, 1.0, 1.0);
       let p2 = build_point(1.0, 1.0, 1.0);

        assert_eq!(p1, p2);
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
}