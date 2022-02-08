pub mod matrix;
pub mod vector;

use num::traits::float::Float;
use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

pub trait Scalar<K>: From<f32> + PartialEq<f32> + Clone + Copy + Float + Sum<K> {}
impl<K: From<f32> + PartialEq<f32> + Clone + Copy + Float + Sum<K>> Scalar<K> for K {}

pub trait VectorSpace<V, K> {
    fn _add(&mut self, v: &V);
    fn _sub(&mut self, v: &V);
    fn scl(&mut self, a: K);
    fn inv_scl(&mut self, a: K);
}
