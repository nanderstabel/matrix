use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    n: usize,
    m: usize,
    matrix: Vec<Vec<K>>,
}

impl<K> Matrix<K> {
    pub fn shape(self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn is_square(self) -> bool {
        self.n == self.m
    }

    pub fn mul_vec(&mut self, vec: Vector<K>) -> Vector<K> {}

    // fn mul_mat(&mut self, mat: Matrix<K>) -> Matrix<K>;
}

impl<K: Scalar<K>> VectorSpace<Matrix<K>, K> for Matrix<K> {
    fn get(&self) -> Vec<K> {
        todo!();
    }

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

#[cfg(test)]
mod matrix {
    use super::*;

    #[test]
    fn shape() {
        assert_eq!(Matrix::from([[1, 2], [3, 4]]).shape(), (2, 2));
        assert_eq!(Matrix::<i32>::from(&[[]]).shape(), (0, 0));
        assert_eq!(Matrix::from([[1]]).shape(), (1, 1));
    }

    #[test]
    fn is_square() {
        assert_eq!(Matrix::<i32>::from(&[&[]]).is_square(), true);
        assert_eq!(Matrix::from(&[&[1]]).is_square(), true);
        assert_eq!(Matrix::from(&[&[1, 2], &[3, 4]]).is_square(), true);

        assert_eq!(Matrix::from(&[&[0, 1]]).is_square(), false);
        assert_eq!(
            Matrix::from(&[&[1, 2], &[3, 4], &[5, 6]]).is_square(),
            false
        );
    }
}
