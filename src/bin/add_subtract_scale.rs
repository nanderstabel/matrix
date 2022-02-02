use rsb::matrix::Matrix;
use rsb::vector::Vector;

#[allow(dead_code)]
fn main() {
    add_subtract_scale(0, 0);
}

#[cfg(test)]
mod add_subtract_scale {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(add_subtract_scale(0, 0), 0);
    }
}
