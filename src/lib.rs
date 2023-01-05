pub mod matrix;
pub mod vector;

use num::traits::float::Float;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};

pub trait Scalar<K>:
    From<K>
    + PartialEq<K>
    + PartialEq
    + Copy
    + std::fmt::Debug
    + Clone
    + Float
    + Sum<K>
    + AddAssign<K>
    + SubAssign<K>
    + MulAssign<K>
{
}
impl<
        K: From<f32>
            + PartialEq<f32>
            + PartialEq
            + Copy
            + std::fmt::Debug
            + Clone
            + Float
            + Sum<K>
            + AddAssign<K>
            + SubAssign<K>
            + MulAssign<K>,
    > Scalar<K> for K
{
}

pub trait VectorSpace<V, K> {
    // fn _add(&mut self, v: &V);
    // fn _sub(&mut self, v: &V);
    // fn scl(&mut self, a: K);
    // fn inv_scl(&mut self, a: K);
}
