use matrix::{vector::Vector, Element, VectorSpace};

fn linear_combination<'a, V, K>(u: &[V], coefs: &'a [K]) -> V
where
    V: VectorSpace<V, K> + From<&'a [K]>,
    K: 'a + Element<K>,
{
    let mut v = V::from(coefs);
    v.scl(coefs[0]);
    v
}

fn main() {
    let e1 = Vector::from([42., 1., 2., 3.]);
    let _e2 = Vector::from([42., 1., 2., 3.]);
    let _e3 = Vector::from([42., 1., 2., 3.]);

    println!("{}", e1);
    println!(
        "{}",
        linear_combination::<Vector<f32>, f32>(&[e1, _e2, _e3], &[10., -2., 0.5])
    );
}
