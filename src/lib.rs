pub mod matrix;
pub mod vector;

// use num::traits::float::Float;
use num::Float;
use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

pub trait Scalar<K>: Clone + Copy + Float + Sum<K> {}
impl<K: Clone + Copy + Float + Sum<K>> Scalar<K> for K {}

pub trait VectorSpace<V, K> {
    fn _add(&mut self, v: &V);
    fn _sub(&mut self, v: &V);
    fn scl(&mut self, a: K);
}
