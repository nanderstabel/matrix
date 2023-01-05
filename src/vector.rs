use crate::*;
use itertools::Itertools;
use num::{pow::Pow, traits::MulAdd, Float};
use std::ops::Deref;
use std::{array::IntoIter, fmt, iter::Sum, ops::AddAssign, ops::MulAssign, ops::SubAssign};

use safe_arch::fused_mul_add_m256;
use safe_arch::m256;

#[derive(Clone, Debug)]
pub struct Vector<K> {
    pub size: usize,
    pub vector: m256,
}

impl<K: Scalar<K>> Deref for Vector<K> {
    type Target = Vec<K>;

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
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
    }

    pub fn norm(&mut self) -> f32 {
        self.vector
            .clone()
            .into_iter()
            .map(|i| i.pow(2.))
            .sum::<f32>()
            .sqrt()
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
    // fn _sub(&mut self, v: &Vector<K>) {
    //     self.vector = self
    //         .vector
    //         .iter()
    //         .zip(v.vector.iter())
    //         .map(|(&u, &ve)| u - ve)
    //         .collect()
    // }

    // fn scl(&mut self, a: K) {
    //     self.vector = self.vector.iter().map(|&v| v * a).collect()
    // }

    // fn inv_scl(&mut self, a: K) {
    //     self.vector = self.vector.iter().map(|&v| v / a).collect()
    // }
}

impl<K: Scalar<K>> Add for Vector<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut res = self;
        res += other;
        res
    }
}

impl<K: Scalar<K>> AddAssign for Vector<K> {
    fn add_assign(&mut self, rhs: Self) {
        self.vector
            .iter_mut()
            .zip_eq(rhs.vector.iter())
            .for_each(|(u, v)| {
                *u += *v;
            });
    }
}

impl<K: Scalar<K>> Sub for Vector<K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut res = self;
        res -= other;
        res
    }
}

impl<K: Scalar<K>> SubAssign for Vector<K> {
    fn sub_assign(&mut self, rhs: Self) {
        self.vector
            .iter_mut()
            .zip_eq(rhs.vector.iter())
            .for_each(|(u, v)| {
                *u -= *v;
            });
    }
}

impl<K: Scalar<K> + std::convert::From<K>> Mul<K> for Vector<K> {
    type Output = Self;

    fn mul(self, f: K) -> Self::Output {
        let mut res = self;
        res.mul_assign(f);
        res
    }
}

impl<K: Scalar<K> + std::convert::From<K>> MulAssign<K> for Vector<K> {
    fn mul_assign(&mut self, rhs: K) {
        self.vector.iter_mut().for_each(|u| {
            *u *= rhs.into();
        });
    }
}

// impl<K: Scalar<K>> Div<K> for Vector<K> {
//     type Output = Self;

//     fn div(self, f: K) -> Self {
//         let mut res = self;
//         res.scl(f);
//         res
//     }
// }

impl<K: Scalar<K>, const N: usize> From<[K; N]> for Vector<K> {
    fn from(v: [K; N]) -> Self {
        let size = v.len();
        let mut arr = [0.f32; 8];
        (0..size).for_each(|e| {
            arr[e] = v[e].into();
        });
        Vector {
            size,
            vector: m256::from_array(arr),
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

impl<K: Scalar<K>> IntoIterator for Vector<K> {
    type Item = K;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vector.into_iter()
    }
}

impl<K: Scalar<K>> FromIterator<K> for Vector<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let vector: Vec<K> = iter.into_iter().collect();
        Vector {
            size: vector.len(),
            vector,
        }
    }
}
