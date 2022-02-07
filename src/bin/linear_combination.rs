use matrix::{vector::Vector, Scalar, VectorSpace};

fn linear_combination<'a, V, K>(u: &[V], coefs: &'a [K]) -> V
where
    V: VectorSpace<V, K> + From<&'a [K]> + std::fmt::Display + Clone,
    K: 'a + Scalar<K>,
{
    let mut iter = u.iter();
    let mut coefs = coefs.iter();
    if let Some(v) = iter.next() {
        let mut res = v.clone();
        res.scl(*coefs.next().unwrap());
        while let Some(v) = iter.next() {
            let mut add = v.clone();
            add.scl(*coefs.next().unwrap());
            res._add(&add);
        }
        return res;
    }
    todo!();
}

fn main() {
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    println!(
        "{}",
        linear_combination::<Vector<f32>, f32>(&[v1, v2], &[10., -2.])
    );
}

#[cfg(test)]
mod linear_combination {
    use super::*;

    #[test]
    fn vector_linear_combination() {
        // let e1 = Vector::from([1., 0., 0.]);
        // let e2 = Vector::from([0., 1., 0.]);
        // let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        assert_eq!(
            linear_combination(&[v1, v2], &[10., -2.]),
            Vector::from([10., 0., 230.])
        );
    }
}
