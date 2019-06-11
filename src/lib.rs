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
   fn vector_plus_point() {
       let p1 = build_point(1.0, 1.0, 1.0);
       let v1 = build_vector(1.0, 1.0, 1.0);
       let p2 = p1 + v1;

       assert_eq!(p2.x, 2.0);
       assert_eq!(p2.y, 2.0);
       assert_eq!(p2.z, 2.0);
       assert_eq!(p2.w, 1.0);
   }

   #[test]
   fn vector_plus_vector() {
       let v1 = build_vector(3.0, 2.0, 1.0);
       let v2 = build_vector(1.0, 2.0, 3.0);
       let v3 = v1 + v2;

       assert_eq!(v3.x, 4.0);
       assert_eq!(v3.y, 4.0);
       assert_eq!(v3.z, 4.0);
       assert_eq!(v3.w, 0.0);
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
}