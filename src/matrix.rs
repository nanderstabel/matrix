use crate::vector::Vector;
use crate::*;
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut, Index, IndexMut};
use itertools::Itertools;
use num::{pow::Pow, Float, NumCast};
use std::fmt;

#[derive(Clone, Debug, Default, Deref, DerefMut, Index, IndexMut, PartialEq)]
pub struct Matrix<K> {
    #[deref]
    #[deref_mut]
    #[index]
    #[index_mut]
    pub matrix: Vec<Vector<K>>,
    pub n: usize,
    pub m: usize,
}

impl<K: Scalar<K> + Float> Matrix<K>
where
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
    K: num::traits::Pow<f32>
        + std::fmt::Display
        + NumCast
        + std::ops::SubAssign
        + std::ops::MulAssign
        + From<f32>,
{
    pub fn shape(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn mul_vec(&mut self, vec: &Vector<K>) -> Vector<K> {
        self.iter()
            .map(|v| (v.clone() * vec.clone()).vector.into_iter().sum())
            .collect()
    }

    pub fn mul_mat(&mut self, mat: &Matrix<K>) -> Matrix<K> {
        self.iter()
            .map(|v| mat.transpose().iter().map(|u| u.dot(v.clone())).collect())
            .collect()
    }

    pub fn trace(&mut self) -> K {
        (0..self.n).map(|idx| self[idx][idx]).sum()
    }

    pub fn transpose(&self) -> Matrix<K> {
        let len = self[0].len();
        let mut iters: Vec<_> = self.iter().map(|n| n.iter()).collect();

        (0..len)
            .map(|_| iters.iter_mut().map(|n| *n.next().unwrap()).collect())
            .collect()
    }

    fn row_echelon(&mut self) -> Matrix<K> {
        let (nrows, ncols) = self.shape();
        let (mut pivot_row, mut pivot_col) = (0, 0);
        let mut res = self.clone();

        // let mut res: Matrix<K> = self
        //     .into_iter()
        //     .map(|v| v.into_iter().collect::<Vector<K>>())
        //     .collect_vec()
        //     .into();

        while pivot_row < nrows - 1 && pivot_col < ncols - 1 {
            let (_, i_max) = (pivot_row..nrows).fold((f32::MIN.into(), 0), |(max, i_max), i| {
                if res[i][pivot_col].abs() > max {
                    (res[i][pivot_col], i)
                } else {
                    (max, i_max)
                }
            });
            if res[i_max][pivot_col] != K::default() {
                for j in 0..res.m {
                    let tmp = res[i_max][j];
                    res[i_max][j] = res[pivot_row][j];
                    res[pivot_row][j] = tmp;
                }
                for i in (pivot_row + 1)..nrows {
                    let ratio = res[i][pivot_col] / res[pivot_row][pivot_col];
                    res[i][pivot_col] = K::default();
                    for j in (pivot_row + 1)..ncols {
                        let tmp = res[pivot_row][j] * ratio;
                        res[i][j] -= tmp;
                    }
                }
                pivot_col += 1;
            }
            pivot_row += 1;
        }
        res
    }

    pub fn reduced_row_echelon(&mut self) -> Matrix<K> {
        let mut pivot = 0;
        let mut res = self.clone();
        for r in 0..res.m {
            if res.n <= pivot {
                return res;
            }
            let mut i = r;
            while res[i][pivot] == 0.0.into() {
                i += 1;
                if i == res.m {
                    i = r;
                    pivot += 1;
                    if pivot == res.n {
                        return res;
                    }
                }
            }
            let divisor = res[r][pivot];
            if divisor != 0.0.into() {
                for j in 0..res.n {
                    res[r][j] = res[r][j] / divisor;
                }
            }
            for j in 0..res.m {
                if j != r {
                    let hold = res[j][pivot];
                    for k in 0..res.n {
                        res[j][k] = res[j][k] - (hold * res[r][k]);
                    }
                }
            }
            pivot += 1;
        }
        res
    }

    pub fn determinant(&mut self) -> K {
        match self.shape() {
            (2, 2) => self[0][0] * self[1][1] - self[0][1] * self[1][0],
            (3, 3) => {
                self[0][0]
                    * Matrix::from([
                        ([self[1][1], self[1][2]]).into(),
                        ([self[2][1], self[2][2]]).into(),
                    ])
                    .determinant()
                    - self[0][1]
                        * Matrix::from([
                            ([self[1][0], self[1][2]]).into(),
                            ([self[2][0], self[2][2]]).into(),
                        ])
                        .determinant()
                    + self[0][2]
                        * Matrix::from([
                            ([self[1][0], self[1][1]]).into(),
                            ([self[2][0], self[2][1]]).into(),
                        ])
                        .determinant()
            }
            (4, 4) => {
                let res = self.clone().row_echelon();
                (0..res.n).fold(1.0.into(), |determinant, i| determinant * res[i][i])
            }
            _ => panic!("Cannot calculate determinant"),
        }
    }

    fn augmented(&mut self) -> Matrix<K> {
        let mut res = self.clone();
        for j in 0..self.m {
            for i in 0..self.n {
                res[j].push(if j == i { 1.0.into() } else { 0.0.into() });
            }
        }
        res.n *= 2;
        res
    }

    pub fn inverse(&mut self) -> Result<Matrix<K>> {
        if self.determinant() == 0.0.into() {
            return Err(anyhow!("Matrix is singular"));
        }
        let reduced = self.augmented().reduced_row_echelon();
        Ok((0..self.m)
            .map(|j| (0..self.n).map(|i| reduced[j][i + self.n]).collect())
            .collect())
    }

    pub fn rank(&mut self) -> usize {
        (self.clone().reduced_row_echelon())
            .iter()
            .filter(|r| r.iter().filter(|&&n| n != Default::default()).count() > 0)
            .count()
    }
}

impl<K: Scalar<K>> Add for Matrix<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl<K: Scalar<K>> AddAssign for Matrix<K> {
    fn add_assign(&mut self, rhs: Self) {
        self.iter_mut().zip_eq(rhs.iter()).for_each(|(m, n)| {
            *m += n.clone();
        });
    }
}

impl<K: Scalar<K>> Sub for Matrix<K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = self;
        res -= other;
        res
    }
}

impl<K: Scalar<K>> SubAssign for Matrix<K> {
    fn sub_assign(&mut self, rhs: Self) {
        self.iter_mut().zip_eq(rhs.iter()).for_each(|(m, n)| {
            *m -= n.clone();
        });
    }
}

impl Mul<f32> for Matrix<f32> {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        let mut res = self;
        res *= f;
        res
    }
}

impl<K: Scalar<K> + From<f32>> MulAssign<f32> for Matrix<K>
where
    Vector<K>: MulAssign<f32>,
{
    fn mul_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|u| {
            *u *= rhs;
        });
    }
}

// impl<K: Scalar<K>> From<&[Vector<K>]> for Matrix<K> {
//     fn from(value: &[Vector<K>]) -> Self {
//         value.into_iter().map(Clone::clone).collect_vec().into()
//     }
// }

// impl<K: Scalar<K>, const N: usize, const M: usize> From<[[K; M]; N]> for Matrix<K> {
//     fn from(value: [[K; M]; N]) -> Self {
//         value.into_iter().map(Into::into).collect_vec().into()
//     }
// }

// impl<K: Scalar<K>> From<Vec<Vector<K>>> for Matrix<K> {
//     fn from(value: Vec<Vector<K>>) -> Self {
//         let matrix: Vec<Vector<K>> = value.into_iter().collect_vec();
//         Matrix {
//             n: matrix[0].vector.len(),
//             m: if matrix[0].vector.is_empty() {
//                 0
//             } else {
//                 matrix.len()
//             },
//             matrix,
//         }
//     }
// }

impl<T: Into<Vec<Vector<K>>>, K: Scalar<K>> From<T> for Matrix<K>
where
    Vector<K>: From<Vec<K>>,
{
    fn from(value: T) -> Self {
        let matrix: Vec<Vector<K>> = value.into();
        Matrix {
            n: matrix[0].vector.len(),
            m: if matrix[0].vector.is_empty() {
                0
            } else {
                matrix.len()
            },
            matrix,
        }
    }
}

impl<K: Scalar<K>> FromIterator<Vector<K>> for Matrix<K> {
    fn from_iter<T: IntoIterator<Item = Vector<K>>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<Vector<K>>>().into()
    }
}

impl<K: Scalar<K>> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.n == 0 {
            write!(f, "[]")
        } else {
            self.iter()
                .fold(Ok(()), |result, v| result.and_then(|_| writeln!(f, "{v}")))
        }
    }
}
