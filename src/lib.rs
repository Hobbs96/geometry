mod point;

#[cfg(test)]
mod point_tests {
    use crate::point::Point;
    #[test]
    fn w_is_one() {
        let point = Point {
            w: 1.0,
            x: 1.0,
            y: 1.0,
            z: 1.0
        };
        assert_eq!(point.w, 1.0);
    }
}
