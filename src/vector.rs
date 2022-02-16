use crate::*;
use num::{pow::Pow, Float};
use std::{fmt, iter::Sum};

#[derive(Clone, Debug)]
pub struct Vector<K> {
    pub size: usize,
    pub vector: Vec<K>,
}

impl<K: Scalar<K> + Float> Vector<K>
where
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
    K: num::traits::Pow<f32>,
{
    pub fn get(&self) -> Vec<K> {
        self.vector.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dot(&self, v: Vector<K>) -> K {
        self.vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u * ve)
            .sum()
    }

    pub fn norm_1(&mut self) -> f32 {
        self.vector
            .clone()
            .into_iter()
            .map(|i| i.abs())
            .sum::<f32>()
            .into()
    }

    pub fn norm(&mut self) -> f32 {
        self.vector
            .clone()
            .into_iter()
            .map(|i| i.pow(2.))
            .sum::<f32>()
            .sqrt()
            .into()
    }

    pub fn norm_inf(&mut self) -> f32 {
        self.vector
            .clone()
            .into_iter()
            .min_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap())
            .unwrap()
            .abs()
            .into()
    }
}

impl<K: Scalar<K>> VectorSpace<Vector<K>, K> for Vector<K> {
    fn _add(&mut self, v: &Vector<K>) {
        self.vector = self
            .vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u + ve)
            .collect()
    }

    fn _sub(&mut self, v: &Vector<K>) {
        self.vector = self
            .vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u - ve)
            .collect()
    }

    fn scl(&mut self, a: K) {
        self.vector = self.vector.iter().map(|&v| v * a).collect()
    }

    fn inv_scl(&mut self, a: K) {
        self.vector = self.vector.iter().map(|&v| v / a).collect()
    }
}

impl<K: Scalar<K>> Add for Vector<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = self.clone();
        res._add(&other);
        res
    }
}

impl<K: Scalar<K>> Sub for Vector<K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = self.clone();
        res._sub(&other);
        res
    }
}

impl Mul<f32> for Vector<f32> {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        let mut res = self.clone();
        res.scl(f);
        res
    }
}

impl<K: Scalar<K>> Div<K> for Vector<K> {
    type Output = Self;

    fn div(self, f: K) -> Self {
        let mut res = self.clone();
        res.scl(f);
        res
    }
}

impl<K, T> From<T> for Vector<K>
where
    K: Scalar<K>,
    T: AsRef<[K]>,
{
    fn from(v: T) -> Self {
        let v = v.as_ref().to_vec();
        Vector {
            size: v.len(),
            vector: v,
        }
    }
}

impl<K: Clone + PartialEq> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        self.vector == other.vector
    }
}

impl<K: Clone + PartialEq> Eq for Vector<K> {}

impl<K: std::fmt::Debug> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.vector)
    }
}

// #[cfg(test)]
// mod vector {
//     use super::*;

//     #[test]
//     fn size() {
//         assert_eq!(Vector::<i32>::from([]).size(), 0);
//         assert_eq!(Vector::from([1]).size(), 1);
//         assert_eq!(Vector::from([3, 4]).size(), 2);
//     }
// }
