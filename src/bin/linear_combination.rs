use matrix::vector::{Scalar, Vector};
use matrix::Generic;

fn linear_combination<V: Scalar<K> + std::fmt::Debug, K: Generic<K>>(
    u: &[V],
    coefs: &[K],
) -> Vector<f32> {
    for i in u.iter() {
        println!("{:?}", i);
    }
    <Vector<_> as Scalar<_>>::from([1.1f32])
}

#[allow(dead_code)]
fn main() {
    let e1 = <Vector<_> as Scalar<f32>>::from([1., 0., 0.]);
    let e2 = <Vector<_> as Scalar<f32>>::from([0., 1., 0.]);
    let e3 = <Vector<_> as Scalar<f32>>::from([0., 0., 1.]);
    let v1 = <Vector<_> as Scalar<f32>>::from([1., 2., 3.]);
    let v2 = <Vector<_> as Scalar<f32>>::from([0., 10., -100.]);

    println!(
        "{}",
        linear_combination::<Vector<f32>, f32>(&[e1, e2, e3], &[10., -2., 0.5])
    );
}

#[cfg(test)]
mod linear_combination {
    use super::*;
}
