use matrix::{vector::Vector, Scalar, VectorSpace};
use num::pow::Pow;
use std::iter::Sum;

fn cross_product<K: Scalar<K>>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
    Vector::from([
        (u.get()[1] * v.get()[2]) - (u.get()[2] * v.get()[1]),
        (u.get()[2] * v.get()[0]) - (u.get()[0] * v.get()[2]),
        (u.get()[0] * v.get()[1]) - (u.get()[1] * v.get()[0]),
    ])
}

fn main() {
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);

    println!("{}", u.dot(v.clone()));
    println!("{}", u.clone().norm());
    println!("{}", v.clone().norm());
    println!("{}", u.clone().norm() * v.clone().norm());
    println!("{}", cross_product(&u, &v));
}

#[cfg(test)]
mod cosine {
    use super::*;

    #[test]
    fn vector_cosine() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        assert_eq!(cross_product(&u, &v), Vector::from([0., 1., 0.]));

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(cross_product(&u, &v), Vector::from([-3., 6., -3.]));

        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        assert_eq!(cross_product(&u, &v), Vector::from([17., -58., -16.]));
    }
}
