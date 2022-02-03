use crate::*;
use std::fmt;

#[derive(Debug)]
pub struct Matrix<K: Clone> {
    n: usize,
    m: usize,
    matrix: Vec<Vec<K>>,
}

impl<K: Generic<K>> Matrix<K> {
    pub fn from<T, Row>(matrix: T) -> Self
    where
        T: AsRef<[Row]>,
        Row: Clone + Copy + AsRef<[K]>,
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

    pub fn shape(self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn is_square(self) -> bool {
        self.n == self.m
    }

    pub fn add(&mut self, m: &Matrix<K>) {
        self.matrix = self
            .matrix
            .iter()
            .zip(m.matrix.iter())
            .map(|(m, ma)| m.iter().zip(ma.iter()).map(|(&i, &j)| i + j).collect())
            .collect()
    }

    pub fn sub(&mut self, m: &Matrix<K>) {
        self.matrix = self
            .matrix
            .iter()
            .zip(m.matrix.iter())
            .map(|(m, ma)| m.iter().zip(ma.iter()).map(|(&i, &j)| i - j).collect())
            .collect()
    }

    pub fn scl(&mut self, a: K) {
        self.matrix = self
            .matrix
            .iter()
            .map(|r| r.iter().map(|&v| v * a).collect())
            .collect()
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
