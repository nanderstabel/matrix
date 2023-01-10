use matrix::{vector::Vector, Scalar};
use num::pow::Pow;
use std::iter::Sum;

fn angle_cos<K: Scalar<K>>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
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
    use float_cmp::*;

    #[test]
    fn vector_cosine() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert_approx_eq!(f32, angle_cos(&u, &v), 1., epsilon = f32::EPSILON);

        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert_approx_eq!(f32, angle_cos(&u, &v), 0., epsilon = f32::EPSILON);

        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert_approx_eq!(f32, angle_cos(&u, &v), -1., epsilon = f32::EPSILON);

        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert_approx_eq!(f32, angle_cos(&u, &v), 1., epsilon = f32::EPSILON);

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_approx_eq!(f32, angle_cos(&u, &v), 0.9746318, epsilon = f32::EPSILON);
    }
}
