pub mod matrix;
pub mod vector;

use num::{pow::Pow, Float, NumCast};
use std::{
    iter::Sum,
    ops::{AddAssign, MulAssign, SubAssign},
};

pub trait Scalar<K>:
    Sum<K>
    + AddAssign<K>
    + SubAssign<K>
    + MulAssign<K>
    + Default
    + NumCast
    + From<f32>
    + Into<f32>
    + Pow<f32>
    + Float
{
}

impl<
        K: Sum<K>
            + AddAssign<K>
            + SubAssign<K>
            + MulAssign<K>
            + Default
            + NumCast
            + From<f32>
            + Into<f32>
            + Pow<f32>
            + Float,
    > Scalar<K> for K
{
}

/// This macro enables the implementation of Add, AddAssign, Sub, SubAssign, Mul and MulAssign for Vector and Matrix.
#[macro_export]
macro_rules! arithmetic {
    ($struct:tt, Add) => {
        arithmetic!($struct, Add, +=);
    };
    ($struct:tt, Sub) => {
        arithmetic!($struct, Sub, -=);
    };
    ($struct:tt, Mul) => {
        arithmetic!($struct, Mul, *=);
    };
    ($struct:ty, $trait:ty, $op:tt) => {
        paste::item! {
        impl<K: Scalar<K>> [<$trait Assign>] for $struct<K> {
            fn [<$trait:lower _assign>](&mut self, rhs: Self) {
                self.iter_mut().zip_eq(rhs.iter()).for_each(|(u, v)| {
                    *u $op v.clone();
                });
            }
        }
        impl<K: Scalar<K>> $trait for $struct<K> {
            type Output = Self;

            fn [<$trait:lower>](self, f: Self) -> Self::Output {
                let mut res = self;
                res.[<$trait:lower _assign>](f);
                res
            }
        }}
    }
}
