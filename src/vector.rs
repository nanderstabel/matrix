use crate::{arithmetic, Scalar};
use derive_more::{Deref, DerefMut, Display, Index, IndexMut};
use itertools::Itertools;
use num::pow::Pow;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Vector struct that consists of a `Vec` of type `K` and it's shape. At this moment the implementation only works for `f32`. However, in the future implementation for complex numbers will be added as well, hence the generic type `K`.
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

    /// Dot product of [`Vector`].
    /// ```
    /// let u = Vector::from([-1., 6.]);
    /// let v = Vector::from([3., 2.]);
    /// assert_eq!(u.dot(v), 9.0);
    /// ```
    pub fn dot(&self, v: Vector<K>) -> K {
        self.iter().zip_eq(v.iter()).map(|(u, v)| *u * *v).sum()
    }

    /// Also called the taxicab norm or Manhattan norm.
    pub fn norm_1(&mut self) -> f32 {
        self.iter().map(|i| i.abs()).sum::<f32>()
    }

    /// Also called the Euclidean norm.
    pub fn norm(&mut self) -> f32 {
        self.iter().map(|i| i.pow(2.)).sum::<f32>().sqrt()
    }

    /// Also called the supremum norm.
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

impl<K: Scalar<K>> FromIterator<K> for Vector<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<K>>().into()
    }
}
