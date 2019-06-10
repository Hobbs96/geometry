pub mod point;
pub mod vector;

#[cfg(test)]
mod point_tests {
    use crate::point::build_point;

    #[test]
    fn w_is_one() {
        let origin = build_point(0.0, 0.0, 0.0);
        assert_eq!(origin.w, 1.0);
    }

    #[test]
    fn values_set_correctly() {
        let point = build_point(7.1, 6.2, -8.3);
        assert_eq!(point.x, 7.1);
        assert_eq!(point.y, 6.2);
        assert_eq!(point.z, -8.3);
    }  

   #[test]
   fn points_compare_equal() {
       let p1 = build_point(1.0, 1.0, 1.0);
       let p2 = build_point(1.0, 1.0, 1.0);

        assert_eq!(p1, p2);
   } 
}

#[cfg(test)]
mod vector_tests {
    use crate::vector::build_vector;

    #[test]
    fn w_is_zero() {
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