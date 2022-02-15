use matrix::{vector::Vector, Scalar};
use num::pow::Pow;
use std::iter::Sum;

fn angle_cos<K: Scalar<K>>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
    K: num::traits::Pow<f32>,
{
    f32::from(u.dot(v.clone())) / (u.clone().norm() * v.clone().norm())
}

fn main() {
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);

    println!("{}", angle_cos(&u, &v));
}

#[cfg(test)]
mod cosine {
    use super::*;

    #[test]
    fn vector_cosine() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert_eq!(angle_cos(&u, &v), 1.);

        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert_eq!(angle_cos(&u, &v), 0.);

        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert_eq!(angle_cos(&u, &v), -1.0000001);

        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert_eq!(angle_cos(&u, &v), 1.);

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(angle_cos(&u, &v), 0.9746318);
    }
}
