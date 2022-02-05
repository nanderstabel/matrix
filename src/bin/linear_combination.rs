use matrix::{vector::Vector, Scalar, VectorSpace};
use safe_arch::m256;

use safe_arch::fused_mul_add_m256;
use safe_arch::*;

fn linear_combination<'a, V, K>(u: &[V], coefs: &'a [K]) -> V
where
    V: VectorSpace<V, K> + From<&'a [K]> + std::fmt::Display,
    K: 'a + Scalar<K>,
{
    let a = m256::from([1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let b = m256::from([0.0, 10.0, -100.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let c = m256::from([0.0, 10.0, -2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

    // use safe_arch::*;
    let result = fused_mul_add_m256(a, b, c).to_array();

    println!("{:?}", result);

    let mut v = V::from(coefs);
    v.scl(coefs[0]);
    v
}

fn main() {
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    println!(
        "{}",
        linear_combination::<Vector<f32>, f32>(&[v1, v2], &[10., -2.])
    );
}
