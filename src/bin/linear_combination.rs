use matrix::{vector::Vector, Element, VectorSpace};

fn linear_combination<'a, V, K>(u: &[V], coefs: &'a [K]) -> V
where
    V: VectorSpace<V, K> + From<&'a [K]> + std::fmt::Display,
    K: 'a + Element<K>,
{
    // for v in u {
    //     let new = v.iter().copied();
    //     new.scl(coefs[0]);
    //     println!("{}", new);
    // }

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
