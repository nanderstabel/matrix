use matrix::vector::Vector;
use std::ops::{Add, Mul, Sub};

fn main() {
    let m1 = Vector::from([2., 1.]);
    let m2 = Vector::from([20., 10.]);

    println!("{}", m2);
}

#[cfg(test)]
mod dot_product {
    use super::*;

    #[test]
    fn vector_dot_product() {
        let mut u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        u.dot(v);
        assert_eq!(u, 0.0);

        let mut u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        u.dot(v);
        assert_eq!(u, 2.0);

        let mut u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        u.dot(v);
        assert_eq!(u, 9.0);
    }
}
