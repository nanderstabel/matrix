use matrix::{matrix::Matrix, vector::Vector, Scalar};
use num::pow::Pow;
use std::iter::Sum;

fn linear_combination<K: Scalar<K>>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    f32: Sum<K> + Sum<<K as Pow<f32>>::Output>,
{
    Matrix::from(u).transpose().mul_vec(&coefs.into())
}

fn main() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    assert_eq!(
        linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]),
        Vector::from([10., -2., 0.5])
    );

    assert_eq!(
        linear_combination::<f32>(&[v1, v2], &[10., -2.]),
        Vector::from([10., 0., 230.])
    );
}

#[cfg(test)]
mod linear_combination {
    use super::*;

    #[test]
    fn test_vector_linear_combination() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        assert_eq!(
            linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]),
            Vector::from([10., -2., 0.5])
        );

        assert_eq!(
            linear_combination::<f32>(&[v1, v2], &[10., -2.]),
            Vector::from([10., 0., 230.])
        );
    }
}
