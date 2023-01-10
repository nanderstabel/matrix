use crate::*;
use derive_more::{Deref, DerefMut, Display, Index, IndexMut};
use itertools::Itertools;
use num::{pow::Pow, Float};
use std::{fmt, iter::Sum, ops::AddAssign, ops::MulAssign, ops::SubAssign};

#[derive(Clone, Debug, Default, Deref, DerefMut, Index, IndexMut, PartialEq, Display)]
#[display(fmt = "{:?}", vector)]
pub struct Vector<K> {
    #[deref]
    #[deref_mut]
    #[index]
    #[index_mut]
    pub vector: Vec<K>,
    size: usize,
}

impl<K: Scalar<K> + Float> Vector<K>
where
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
    K: num::traits::Pow<f32> + std::convert::From<f32>,
{
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dot(&self, v: Vector<K>) -> K {
        self.iter().zip_eq(v.iter()).map(|(u, v)| *u * *v).sum()
    }

    pub fn norm_1(&mut self) -> f32 {
        self.iter().map(|i| i.abs()).sum::<f32>()
    }

    pub fn norm(&mut self) -> f32 {
        self.iter().map(|i| i.pow(2.)).sum::<f32>().sqrt()
    }

    pub fn norm_inf(&mut self) -> f32 {
        self.iter()
            .min_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap())
            .unwrap()
            .abs()
            .into()
    }
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
        if self.size == 0 {
            *self = rhs;
            return;
        }
        self.iter_mut().zip_eq(rhs.iter()).for_each(|(u, v)| {
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
        self.iter_mut().zip_eq(rhs.iter()).for_each(|(u, v)| {
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
        self.iter_mut().for_each(|u| {
            *u *= rhs;
        });
    }
}

impl<K: Scalar<K> + std::convert::From<K>> Mul<Vector<K>> for Vector<K> {
    type Output = Self;

    fn mul(self, f: Vector<K>) -> Self::Output {
        let mut res = self;
        res.mul_assign(f);
        res
    }
}

impl<K: Scalar<K> + std::convert::From<K>> MulAssign<Vector<K>> for Vector<K> {
    fn mul_assign(&mut self, rhs: Vector<K>) {
        self.iter_mut().zip_eq(rhs.iter()).for_each(|(u, v)| {
            *u *= *v;
        });
    }
}

impl<T: Into<Vec<K>>, K: Scalar<K>> From<T> for Vector<K> {
    fn from(v: T) -> Self {
        let vector = v.into();
        Vector {
            size: vector.len(),
            vector,
        }
    }
}

impl<K: Scalar<K>> FromIterator<K> for Vector<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<K>>().into()
    }
}
