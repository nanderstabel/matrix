use matrix::matrix::Matrix;
use matrix::vector::Vector;

#[allow(dead_code)]
fn main() {
    let m = Matrix::from(&[&[1, 2], &[3, 4]]);
    println!("{}", m);
}

#[cfg(test)]
mod add_subtract_scale {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(add_subtract_scale(0, 0), 0);
    }
}
