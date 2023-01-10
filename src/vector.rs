use crate::{arithmetic, complex::Complex, Scalar};
use derive_more::{Deref, DerefMut, Display, Index, IndexMut};
use itertools::Itertools;
use num::pow::Pow;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

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

impl<K: Scalar<K>> Vector<K>
where
    f32: Sum<K> + Sum<<K as Pow<f32>>::Output>,
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

impl<K: Scalar<K>> Mul<K> for Vector<K> {
    type Output = Self;

    fn mul(self, f: K) -> Self::Output {
        let mut res = self;
        res.mul_assign(f);
        res
    }
}

impl<K: Scalar<K>> MulAssign<K> for Vector<K> {
    fn mul_assign(&mut self, rhs: K) {
        self.iter_mut().for_each(|u| {
            *u *= rhs;
        });
    }
}

arithmetic!(Vector, Add);
arithmetic!(Vector, Sub);
arithmetic!(Vector, Mul);

impl<T: Into<Vec<K>>, K: Scalar<K>> From<T> for Vector<K> {
    fn from(v: T) -> Self {
        let vector = v.into();
        Vector {
            size: vector.len(),
            vector,
        }
    }
}

impl<T: Into<Vec<Complex>>> From<T> for Vector<Complex> {
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
