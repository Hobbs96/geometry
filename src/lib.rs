pub mod point;

#[cfg(test)]
mod point_tests {
    use crate::point::build_point;

    #[test]
    fn w_is_one() {
        let origin = build_point(0.0, 0.0, 0.0);
        assert_eq!(origin.w, 1.0);
    }
}
