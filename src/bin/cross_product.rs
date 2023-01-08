use matrix::{vector::Vector, Scalar};
use num::pow::Pow;
use std::iter::Sum;

fn cross_product<K: Scalar<K>>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    f32: Sum<K>,
    f32: From<K>,
    f32: Sum<<K as Pow<f32>>::Output>,
    K: Pow<f32>,
{
    Vector::from([
        (u[1] * v[2]) - (u[2] * v[1]),
        (u[2] * v[0]) - (u[0] * v[2]),
        (u[0] * v[1]) - (u[1] * v[0]),
    ])
}

fn main() {
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);

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
