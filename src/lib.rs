pub mod matrix;
pub mod vector;

use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

pub trait Scalar<K>:
    Clone + Copy + Mul<f32, Output = K> + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Sum<K>
{
}
impl<
        K: Clone
            + Copy
            + Mul<f32, Output = K>
            + Add<Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Sum<K>,
    > Scalar<K> for K
{
}

pub trait VectorSpace<V, K> {
    fn get(&mut self) -> Vec<K>;
    fn _add(&mut self, v: &V);
    fn _sub(&mut self, v: &V);
    fn scl(&mut self, a: K);
}
