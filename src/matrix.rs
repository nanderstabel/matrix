use crate::{arithmetic, vector::Vector, Scalar};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut, Index, IndexMut};
use itertools::Itertools;
use num::pow::Pow;
use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Matrix struct that consists of a `Vec` of [`Vector`]'s and it's shape.
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

impl<K: Scalar<K>> Matrix<K>
where
    f32: Sum<K> + Sum<<K as Pow<f32>>::Output>,
{
    pub fn shape(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    /// Multiplies this [`Matrix`] by the given [`Vector`].
    /// ```
    /// let mut u = Matrix::from([[2., 0.], [0., 2.]]);
    /// let v = Vector::from([4., 2.]);
    /// assert_eq!(u.mul_vec(&v), Vector::from([8., 4.]));
    /// ```
    pub fn mul_vec(&mut self, vec: &Vector<K>) -> Vector<K> {
        self.iter()
            .map(|v| (v.clone() * vec.clone()).vector.into_iter().sum())
            .collect()
    }

    /// Multiplies this [`Matrix`] by the given `Matrix`.
    /// ```
    /// let mut u = Matrix::from([[3., -5.], [6., 8.]]);
    /// let v = Matrix::from([[2., 1.], [4., 2.]]);
    /// assert_eq!(u.mul_mat(&v), Matrix::from([[-14., -7.], [44., 22.]]));
    /// ```
    pub fn mul_mat(&mut self, mat: &Matrix<K>) -> Matrix<K> {
        self.iter()
            .map(|v| mat.transpose().iter().map(|u| u.dot(v.clone())).collect())
            .collect()
    }

    /// Computes the trace of the current [`Matrix`].
    /// ```
    /// let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    /// assert_eq!(u.trace(), 9.0);
    /// ```
    pub fn trace(&mut self) -> K {
        (0..self.n).map(|idx| self[idx][idx]).sum()
    }

    /// Computes and returns the transpose matrix of the current [`Matrix`].
    /// ```
    /// let u = Matrix::from([[1., 2., 3.], [3., 4., 5.], [6., 7., 8.]]);
    /// assert_eq!(
    ///     u.transpose(),
    ///     Matrix::from([[1., 3., 6.], [2., 4., 7.], [3., 5., 8.]])
    /// );
    /// ```
    pub fn transpose(&self) -> Matrix<K> {
        let mut iters: Vec<_> = self.iter().map(|n| n.iter()).collect();

        (0..self[0].len())
            .map(|_| iters.iter_mut().map(|n| *n.next().unwrap()).collect())
            .collect()
    }

    fn row_echelon(&mut self) -> Matrix<K> {
        let (nrows, ncols) = self.shape();
        let (mut pivot_row, mut pivot_col) = (0, 0);
        let mut res = self.clone();

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

    /// Computes the reduced row-echelon form of the current [`Matrix`].
    /// ```
    /// let mut u = Matrix::from([
    ///     [8., 5., -2., 4., 28.],
    ///     [4., 2.5, 20., 4., -4.],
    ///     [8., 5., 1., 4., 17.],
    /// ]);
    /// assert_eq!(
    ///     u.reduced_row_echelon(),
    ///     Matrix::from([
    ///         [1., 0.625, 0., 0., -12.166668],
    ///         [0., 0., 1., 0., -3.6666667],
    ///         [-0., -0., -0., 1., 29.5],
    ///     ])
    /// );
    /// ```
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

    /// Calculates the determinant of a [`Matrix`]. Currently only for matrices up to shape 4x4.
    /// ```
    /// let mut u = Matrix::from([
    ///     [8., 5., -2., 4.],
    ///     [4., 2.5, 20., 4.],
    ///     [8., 5., 1., 4.],
    ///     [28., -4., 17., 1.],
    /// ]);
    /// assert_eq!(u.determinant(), 1032.0);
    /// ```
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

    /// Calculates the inverse of the [`Matrix`].
    /// ```
    /// let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    /// if let Ok(inverse) = u.inverse() {
    ///     assert_eq!(
    ///         inverse,
    ///         Matrix::from([
    ///             [0.649425287, 0.097701149, -0.655172414],
    ///             [-0.781609195, -0.126436782, 0.965517241],
    ///             [0.143678161, 0.0747126454, -0.206896552]
    ///         ])
    ///     );
    /// }
    /// ```
    pub fn inverse(&mut self) -> Result<Matrix<K>> {
        if self.determinant() == 0.0.into() {
            return Err(anyhow!("Matrix is singular"));
        }
        let reduced = self.augmented().reduced_row_echelon();
        Ok((0..self.m)
            .map(|j| (0..self.n).map(|i| reduced[j][i + self.n]).collect())
            .collect())
    }

    /// Cmputes the rank of the current [`Matrix`].
    /// ```
    /// let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
    /// assert_eq!(u.rank(), 3);
    /// ```
    pub fn rank(&mut self) -> usize {
        (self.clone().reduced_row_echelon())
            .iter()
            .filter(|r| r.iter().any(|n| *n != Default::default()))
            .count()
    }
}

arithmetic!(Matrix, Add);
arithmetic!(Matrix, Sub);

impl Mul<f32> for Matrix<f32> {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        let mut res = self;
        res *= f;
        res
    }
}

impl<K: Scalar<K>> MulAssign<f32> for Matrix<K>
where
    Vector<K>: MulAssign<f32>,
{
    fn mul_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|u| {
            *u *= rhs;
        });
    }
}

impl<K: Scalar<K>> From<&[Vector<K>]> for Matrix<K> {
    fn from(value: &[Vector<K>]) -> Self {
        value.into_iter().map(Clone::clone).collect_vec().into()
    }
}

impl<K: Scalar<K>, const N: usize, const M: usize> From<[[K; M]; N]> for Matrix<K> {
    fn from(value: [[K; M]; N]) -> Self {
        value.into_iter().map(Into::into).collect_vec().into()
    }
}

impl<K: Scalar<K>> From<Vec<Vector<K>>> for Matrix<K> {
    fn from(value: Vec<Vector<K>>) -> Self {
        let matrix: Vec<Vector<K>> = value.into_iter().collect_vec();
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

impl<K: Scalar<K> + std::fmt::Debug> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.n == 0 {
            write!(f, "[]")
        } else {
            self.iter()
                .fold(Ok(()), |result, v| result.and_then(|_| writeln!(f, "{v}")))
        }
    }
}
