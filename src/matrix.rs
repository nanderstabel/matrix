use crate::vector::Vector;
use crate::*;
use num::{pow::Pow, Float};
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
    K: num::traits::Pow<f32> + std::fmt::Display,
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

    pub fn row_echelon(&mut self) -> Matrix<K> {
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
            for j in 0..res.n {
                let tmp = res.matrix[r][j];
                res.matrix[r][j] = res.matrix[r][j];
                res.matrix[r][j] = tmp;
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
        todo!();
        let mut clone = self.clone();
        clone.row_echelon();
        clone.matrix[0][0] * clone.matrix[1][1] * clone.matrix[2][2] * clone.matrix[3][3]
    }

    pub fn determinant(&mut self) -> K {
        match self.shape() {
            (2, 2) => self.dim2_determinant(self.get()),
            (3, 3) => self.dim3_determinant(self.get()),
            (4, 4) => self.dim4_determinant(),
            _ => panic!(),
        }
    }

    // pub fn inverse(&mut self) -> Result<Matrix<K>> {
    pub fn inverse(&mut self) -> Matrix<K> {
        todo!();
    }

    pub fn rank(&mut self) -> usize {
        (self.clone().row_echelon())
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
        let mut res = self.clone();
        res._add(&other);
        res
    }
}

impl<K: Scalar<K>> Sub for Matrix<K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = self.clone();
        res._sub(&other);
        res
    }
}

impl Mul<f32> for Matrix<f32> {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        let mut res = self.clone();
        res.scl(f);
        res
    }
}

impl Div<f32> for Matrix<f32> {
    type Output = Self;

    fn div(self, f: f32) -> Self {
        let mut res = self.clone();
        res.inv_scl(f);
        res
    }
}

// impl<K: Clone, T, Row> From<T> for Matrix<K>
// where
//     K: Scalar<K>,
//     T: AsRef<[Row]>,
//     Row: Clone + AsRef<[K]>,
// {
//     fn from(m: T) -> Self {
//         let m: Vec<Vec<K>> = m
//             .as_ref()
//             .to_vec()
//             .iter()
//             .map(|x| x.as_ref().to_vec())
//             .collect();
//         Matrix {
//             n: m[0].len(),
//             m: if m[0].len() == 0 { 0 } else { m.len() },
//             matrix: m,
//         }
//     }
// }

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
            m: if matrix[0].len() == 0 {
                0
            } else {
                matrix.len()
            },
            matrix: matrix,
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
                write!(f, "{}]\n", self.matrix[j][self.n - 1])?;
            }
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod matrix {
//     use super::*;

//     #[test]
//     fn shape() {
//         assert_eq!(Matrix::from([[1, 2], [3, 4]]).shape(), (2, 2));
//         assert_eq!(Matrix::<i32>::from(&[[]]).shape(), (0, 0));
//         assert_eq!(Matrix::from([[1]]).shape(), (1, 1));
//     }

//     #[test]
//     fn is_square() {
//         assert_eq!(Matrix::<i32>::from(&[&[]]).is_square(), true);
//         assert_eq!(Matrix::from(&[&[1]]).is_square(), true);
//         assert_eq!(Matrix::from(&[&[1, 2], &[3, 4]]).is_square(), true);

//         assert_eq!(Matrix::from(&[&[0, 1]]).is_square(), false);
//         assert_eq!(
//             Matrix::from(&[&[1, 2], &[3, 4], &[5, 6]]).is_square(),
//             false
//         );
//     }
// }
