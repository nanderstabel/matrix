use itertools::Itertools;
use matrix::{vector::Vector, Scalar};
use num::pow::Pow;
use std::iter::Sum;

use safe_arch::fused_mul_add_m256;
use safe_arch::m256;

fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Scalar<K> + Pow<f32> + std::fmt::Display + From<f32>,
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
{
    let res = u
        .iter()
        .zip_eq(coefs.iter())
        .map(|(e, c)| {
            let mut e_arr = [0.0f32; 8];
            let mut c_arr = [0.0f32; 8];
            e_arr
                .iter_mut()
                .zip_eq(c_arr.iter_mut())
                .zip(e.iter())
                .for_each(|((e_b, c_b), e)| {
                    *e_b = (*e).into();
                    *c_b = (*c).into();
                });

            (m256::from(e_arr), m256::from(c_arr))
        })
        .fold(Default::default(), |c, (a, b)| fused_mul_add_m256(a, b, c));

    Vector::from_m256(res, u[0].size)
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
