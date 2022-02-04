pub mod matrix;
pub mod vector;

use std::ops::{Add, Mul, Sub};

pub trait Element<K>: Clone + Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> {}
impl<K: Clone + Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K>> Element<K> for K {}

pub trait VectorSpace<V, K> {
    fn add(&mut self, v: &V);
    fn sub(&mut self, v: &V);
    fn scl(&mut self, a: K);
}
