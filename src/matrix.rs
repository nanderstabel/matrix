use crate::vector::Vector;
use crate::*;
use anyhow::{anyhow, Result};
use num::{pow::Pow, Float, NumCast};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub n: usize,
    pub m: usize,
    pub matrix: Vec<Vec<K>>,
}

impl<K: Scalar<K> + Float> Matrix<K>
where
    f32: Sum<K> + From<K> + Sum<<K as Pow<f32>>::Output>,
    K: num::traits::Pow<f32>
        + std::fmt::Display
        + NumCast
        + std::ops::SubAssign
        + std::ops::MulAssign,
{
    pub fn get(&self) -> Vec<Vec<K>> {
        self.matrix.clone()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn is_square(self) -> bool {
        self.n == self.m
    }

    pub fn mul_vec(&mut self, vec: &Vector<K>) -> Vector<K> {
        Vector {
            size: self.m,
            vector: self
                .matrix
                .iter()
                .map(|m| Vector::from(m).dot(vec.clone()))
                .collect::<Vec<K>>(),
        }
    }

    pub fn mul_mat(&mut self, mat: &Matrix<K>) -> Matrix<K> {
        let mut mat = mat.clone().transpose();
        mat.matrix = (0..self.n)
            .map(|row| {
                (0..mat.m)
                    .map(|column| {
                        let r = Vector::from(self.matrix[row].clone());
                        let c = Vector::from(mat.matrix[column].clone());
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
        (0..self.n).map(|idx| self.matrix[idx][idx]).sum()
    }

    pub fn transpose(&self) -> Matrix<K> {
        Matrix::from(
            (0..self.n)
                .map(|i| (0..self.m).map(|j| self.matrix[j][i]).collect())
                .collect::<Vec<Vec<K>>>(),
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
                if res.matrix[i][pivot_col].abs() > max {
                    max = res.matrix[i][pivot_col];
                    i_max = i;
                }
            }
            if res.matrix[i_max][pivot_col] == 0. {
                pivot_col += 1;
            } else {
                for j in 0..res.m {
                    let tmp = res.matrix[i_max][j];
                    res.matrix[i_max][j] = res.matrix[pivot_row][j];
                    res.matrix[pivot_row][j] = tmp;
                }
                for i in (pivot_row + 1)..nrows {
                    let ratio: K = res.matrix[i][pivot_col] / res.matrix[pivot_row][pivot_col];
                    res.matrix[i][pivot_col] = <K as From<f32>>::from(0.);
                    for j in (pivot_row + 1)..ncols {
                        let tmp = res.matrix[pivot_row][j] * ratio;
                        res.matrix[i][j] -= tmp;
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
            while res.matrix[i][pivot] == 0. {
                i += 1;
                if i == res.m {
                    i = r;
                    pivot += 1;
                    if pivot == res.n {
                        return res;
                    }
                }
            }
            let divisor = res.matrix[r][pivot];
            if divisor != 0. {
                for j in 0..res.n {
                    res.matrix[r][j] = res.matrix[r][j] / divisor;
                }
            }
            for j in 0..res.m {
                if j != r {
                    let hold = res.matrix[j][pivot];
                    for k in 0..res.n {
                        res.matrix[j][k] = res.matrix[j][k] - (hold * res.matrix[r][k]);
                    }
                }
            }
            pivot += 1;
        }
        res
    }

    fn dim2_determinant(&self, m: Vec<Vec<K>>) -> K {
        m[0][0] * m[1][1] - m[0][1] * m[1][0]
    }

    fn dim3_determinant(&self, m: Vec<Vec<K>>) -> K {
        m[0][0] * self.dim2_determinant(vec![m[1][1..].to_vec(), m[2][1..].to_vec()])
            - m[0][1] * self.dim2_determinant(vec![vec![m[1][0], m[1][2]], vec![m[2][0], m[2][2]]])
            + m[0][2] * self.dim2_determinant(vec![m[1][..2].to_vec(), m[2][..2].to_vec()])
    }

    fn dim4_determinant(&self) -> K {
        let mut determinant = <K as From<f32>>::from(1.);
        let res = self.clone().row_echelon();
        for i in 0..res.n {
            determinant *= res.matrix[i][i];
        }
        determinant
    }

    pub fn determinant(&mut self) -> K {
        match self.shape() {
            (2, 2) => self.dim2_determinant(self.get()),
            (3, 3) => self.dim3_determinant(self.get()),
            _ => self.dim4_determinant(),
        }
    }

    fn augmented(&mut self) -> Matrix<K> {
        let mut res = self.clone();
        for j in 0..self.m {
            for i in 0..(self.n) {
                res.matrix[j].push(if j == i {
                    <K as From<f32>>::from(1.)
                } else {
                    <K as From<f32>>::from(0.)
                });
            }
        }
        res.n *= 2;
        println!("{}", res);
        res
    }

    pub fn inverse(&mut self) -> Result<Matrix<K>> {
        if self.determinant() == 0.0 {
            return Err(anyhow!("Matrix is singular"));
        }
        let reduced = self.augmented().reduced_row_echelon();
        let mut res = self.clone();
        for j in 0..res.m {
            for i in 0..res.n {
                res.matrix[j][i] = reduced.matrix[j][i + res.n];
            }
        }
        println!("{}", res);
        Ok(res)
    }

    pub fn rank(&mut self) -> usize {
        (self.clone().reduced_row_echelon())
            .matrix
            .iter()
            .filter(|r| r.iter().filter(|&&n| n != 0.0 && n != -0.0).count() > 0)
            .count()
    }
}

impl<K: Scalar<K>> VectorSpace<Matrix<K>, K> for Matrix<K> {
    fn _add(&mut self, m: &Matrix<K>) {
        self.matrix = self
            .matrix
            .iter()
            .zip(m.matrix.iter())
            .map(|(m, ma)| m.iter().zip(ma.iter()).map(|(&i, &j)| i + j).collect())
            .collect()
    }

    fn _sub(&mut self, m: &Matrix<K>) {
        self.matrix = self
            .matrix
            .iter()
            .zip(m.matrix.iter())
            .map(|(m, ma)| m.iter().zip(ma.iter()).map(|(&i, &j)| i - j).collect())
            .collect()
    }

    fn scl(&mut self, a: K) {
        self.matrix = self
            .matrix
            .iter()
            .map(|r| r.iter().map(|&v| v * a).collect())
            .collect()
    }

    fn inv_scl(&mut self, a: K) {
        self.matrix = self
            .matrix
            .iter()
            .map(|r| r.iter().map(|&v| v / a).collect())
            .collect()
    }
}

impl<K: Scalar<K>> Add for Matrix<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = self;
        res._add(&other);
        res
    }
}

impl<K: Scalar<K>> Sub for Matrix<K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = self;
        res._sub(&other);
        res
    }
}

impl Mul<f32> for Matrix<f32> {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        let mut res = self;
        res.scl(f);
        res
    }
}

impl Div<f32> for Matrix<f32> {
    type Output = Self;

    fn div(self, f: f32) -> Self {
        let mut res = self;
        res.inv_scl(f);
        res
    }
}

impl<K: Scalar<K>> Matrix<K> {
    pub fn from<T, Row>(matrix: T) -> Self
    where
        T: AsRef<[Row]>,
        Row: Clone + AsRef<[K]>,
    {
        let matrix: Vec<Vec<K>> = matrix
            .as_ref()
            .to_vec()
            .iter()
            .map(|x| x.as_ref().to_vec())
            .collect();
        Matrix {
            n: matrix[0].len(),
            m: if matrix[0].is_empty() {
                0
            } else {
                matrix.len()
            },
            matrix,
        }
    }
}

impl<K: Clone + PartialEq> PartialEq for Matrix<K> {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

impl<K: Clone + PartialEq> Eq for Matrix<K> {}

impl<K: fmt::Display + Clone> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.n == 0 {
            write!(f, "[]")?;
        } else {
            for j in 0..(self.m) {
                write!(f, "[")?;
                for i in 0..(self.n - 1) {
                    write!(f, "{}, ", self.matrix[j][i])?;
                }
                writeln!(f, "{}]", self.matrix[j][self.n - 1])?;
            }
        }
        Ok(())
    }
}
