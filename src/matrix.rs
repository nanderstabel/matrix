use crate::vector::Vector;
use crate::*;
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut, Index, IndexMut};
use itertools::Itertools;
use num::{pow::Pow, Float, NumCast};
use std::fmt;

#[derive(Clone, Debug, Default, Deref, DerefMut, Index, IndexMut)]
pub struct Matrix<K> {
    pub n: usize,
    pub m: usize,
    #[deref]
    #[deref_mut]
    #[index]
    #[index_mut]
    pub matrix: Vec<Vector<K>>,
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
    //     pub fn get(&self) -> Vec<Vec<K>> {
    //         self.matrix.clone()
    //     }

    pub fn shape(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    //     pub fn is_square(self) -> bool {
    //         self.n == self.m
    //     }

    pub fn mul_vec(&mut self, vec: &Vector<K>) -> Vector<K> {
        Vector::from(
            self.iter()
                .map(|v| (v.clone() * vec.clone()).vector.into_iter().sum::<K>())
                .collect_vec(),
        )
    }

    pub fn mul_mat(&mut self, mat: &Matrix<K>) -> Matrix<K> {
        let mut mat = mat.clone().transpose();
        mat.matrix = (0..self.n)
            .map(|row| {
                (0..mat.m)
                    .map(|column| {
                        let r = self[row].clone();
                        let c = mat[column].clone();
                        r.dot(c)
                    })
                    .collect()
            })
            .collect();
        mat.m = mat.n;
        mat.n = self.n;
        mat
    }

    pub fn trace(&mut self) -> K {
        (0..self.n).map(|idx| self[idx][idx]).sum()
    }

    pub fn transpose(&self) -> Matrix<K> {
        let len = self[0].len();
        let mut iters: Vec<_> = self.iter().map(|n| n.iter()).collect();

        Matrix::from(
            (0..len)
                .map(|_| {
                    iters
                        .iter_mut()
                        .map(|n| *n.next().unwrap())
                        .collect::<Vector<K>>()
                })
                .collect::<Vec<Vector<K>>>()
                .as_slice(),
        )
    }

    fn row_echelon(&mut self) -> Matrix<K> {
        let (nrows, ncols) = self.shape();
        let (mut pivot_row, mut pivot_col) = (0, 0);
        let mut res = self.clone();
        while pivot_row < nrows - 1 && pivot_col < ncols - 1 {
            let mut max: K = <K as From<f32>>::from(f32::MIN);
            let mut i_max = 0;
            for i in pivot_row..nrows {
                if res[i][pivot_col].abs() > max {
                    max = res[i][pivot_col];
                    i_max = i;
                }
            }
            if res[i_max][pivot_col] == 0.0.into() {
                pivot_col += 1;
            } else {
                for j in 0..res.m {
                    let tmp = res[i_max][j];
                    res[i_max][j] = res[pivot_row][j];
                    res[pivot_row][j] = tmp;
                }
                for i in (pivot_row + 1)..nrows {
                    let ratio: K = res[i][pivot_col] / res[pivot_row][pivot_col];
                    res[i][pivot_col] = <K as From<f32>>::from(0.);
                    for j in (pivot_row + 1)..ncols {
                        let tmp = res[pivot_row][j] * ratio;
                        res[i][j] -= tmp;
                    }
                }
                pivot_row += 1;
                pivot_col += 1;
            }
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

    fn dim2_determinant(&self, m: Vec<Vector<K>>) -> K {
        m[0][0] * m[1][1] - m[0][1] * m[1][0]
    }

    fn dim3_determinant(&self, m: Vec<Vector<K>>) -> K {
        m[0][0] * self.dim2_determinant(vec![Vector::from(&m[1][1..]), Vector::from(&m[2][1..])])
            - m[0][1]
                * self.dim2_determinant(vec![
                    Vector::from([m[1][0], m[1][2]]),
                    Vector::from([m[2][0], m[2][2]]),
                ])
            + m[0][2]
                * self.dim2_determinant(vec![Vector::from(&m[1][..2]), Vector::from(&m[2][..2])])
    }

    fn dim4_determinant(&self) -> K {
        let mut determinant = <K as From<f32>>::from(1.);
        let res = self.clone().row_echelon();
        for i in 0..res.n {
            determinant *= res[i][i];
        }
        determinant
    }

    pub fn determinant(&mut self) -> K {
        match self.shape() {
            (2, 2) => self.dim2_determinant(self.matrix.clone()),
            (3, 3) => self.dim3_determinant(self.matrix.clone()),
            _ => self.dim4_determinant(),
        }
    }

    fn augmented(&mut self) -> Matrix<K> {
        let mut res = self.clone();
        for j in 0..self.m {
            for i in 0..(self.n) {
                res[j]
                    .vector
                    .push(if j == i { 1.0.into() } else { 0.0.into() });
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
        let mut res = self.clone();
        for j in 0..res.m {
            for i in 0..res.n {
                res[j][i] = reduced[j][i + res.n];
            }
        }
        Ok(res)
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

impl<K: Scalar<K> + std::convert::From<f32>> MulAssign<f32> for Matrix<K>
where
    Vector<K>: std::ops::MulAssign<f32>,
{
    fn mul_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|u| {
            *u *= rhs;
        });
    }
}

impl<K: Scalar<K>> From<&[Vector<K>]> for Matrix<K> {
    fn from(value: &[Vector<K>]) -> Self {
        let matrix = value.to_vec();
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

impl<K: Scalar<K>, const N: usize, const M: usize> From<[[K; M]; N]> for Matrix<K> {
    fn from(value: [[K; M]; N]) -> Self {
        let matrix: Vec<Vector<K>> = value.into_iter().map(Into::into).collect_vec();
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

impl<K: Scalar<K>> PartialEq for Matrix<K> {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

impl<K: Scalar<K>> Eq for Matrix<K> {}

impl<K: Scalar<K>> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.n == 0 {
            write!(f, "[]")?;
        } else {
            self.iter().for_each(|u| {
                writeln!(f, "{}", u).unwrap();
            })
        }
        Ok(())
    }
}
