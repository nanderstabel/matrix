use std::ops::Deref;

use itertools::Itertools;
use matrix::{matrix::Matrix, vector::Vector, Scalar, VectorSpace};
use num::pow::Pow;
use num::traits::ops::mul_add;
use num::traits::MulAdd;
use std::iter::Sum;

use safe_arch::fused_mul_add_m256;
use safe_arch::m256;

fn linear_combination<V, K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    V: VectorSpace<V, K> + std::fmt::Display + Clone + Deref,
    K: Scalar<K> + Pow<f32> + std::fmt::Display,
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
{
    Matrix::from(u)
        .transpose()
        .matrix
        .iter()
        .map(|v| {
            let a = m256::from_array([1., 2., 3., 0., 0., 0., 0., 0.]);
            let b = m256::from_array([10., 10., 10., 0., 0., 0., 0., 0.]);

            let c = m256::from_array([0., 10., -100., 0., 0., 0., 0., 0.]);
            let d = m256::from_array([-2., -2., -2., 0., 0., 0., 0., 0.]);

            let e = fused_mul_add_m256(c, d, [0., 0., 0., 0., 0., 0., 0., 0.].into());
            let f = fused_mul_add_m256(a, b, e);

            dbg!(&f);

            v.iter()
                .zip_eq(coefs.iter())
                .map(|(e, c)| *e * *c)
                .sum::<K>()
        })
        .collect()
}

fn main() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    let mut test = [1, 2, 3, 4, 5, 6, 7, 8];

    dbg!(&test);

    assert_eq!(
        linear_combination::<Vector<f32>, f32>(&[e1, e2, e3], &[10., -2., 0.5]),
        Vector::from([10., -2., 0.5])
    );

    assert_eq!(
        linear_combination::<Vector<f32>, f32>(&[v1, v2], &[10., -2.]),
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
            linear_combination::<Vector<f32>, f32>(&[e1, e2, e3], &[10., -2., 0.5]),
            Vector::from([10., -2., 0.5])
        );

        assert_eq!(
            linear_combination::<Vector<f32>, f32>(&[v1, v2], &[10., -2.]),
            Vector::from([10., 0., 230.])
        );
    }
}
