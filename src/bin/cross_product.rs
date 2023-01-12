use matrix::{matrix::Matrix, vector::Vector, Scalar};
use num::pow::Pow;
use std::iter::Sum;

/// Calculate the cross product of two [`Vector`]'s.
fn cross_product<K: Scalar<K>>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    f32: Sum<K> + Sum<<K as Pow<f32>>::Output>,
{
    Vector::from([
        (u[1] * v[2]) - (v[1] * u[2]),
        (u[2] * v[0]) - (v[2] * u[0]),
        (u[0] * v[1]) - (v[0] * u[1]),
    ])
}

fn main() {
    let a = Vector::from([1., 1., 1.]);
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);

    let mut matrix = Matrix::from(vec![a, u, v]).transpose();

    // dbg!(u.dot(v));

    dbg!(matrix.determinant());

    // println!("{}", cross_product(&u, &v));
}

#[cfg(test)]
mod cosine {
    use super::*;

    #[test]
    fn vector_cross_product() {
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
