use matrix::vector::Vector;
use std::ops::{Add, Mul, Sub};

fn main() {
    let m1 = Vector::from([2., 1.]);
    let m2 = Vector::from([20., 10.]);

    println!("{}", m2);
}

#[cfg(test)]
mod norm {
    use super::*;

    #[test]
    fn vector_norm() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!((u.norm_1(), u.norm(), u.norm_inf()), (0., 0., 0.));

        let u = Vector::from([1., 2., 3.]);
        assert_eq!((u.norm_1(), u.norm(), u.norm_inf()), (6., 3.74165738, 3.));

        let u = Vector::from([-1., -2.]);
        assert_eq!((u.norm_1(), u.norm(), u.norm_inf()), (3., 2.236067977, 2.));
    }
}
