use std::ops::{Add, Mul, Sub};

pub mod matrix;
pub mod vector;

pub trait Generic<K>: Clone + Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K> {}
impl<K: Clone + Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K>> Generic<K> for K {}
