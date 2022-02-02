use std::fmt;

pub struct Matrix<'a, K> {
    pub matrix: &'a [&'a [K]],
}

impl<'a, K> Matrix<'a, K> {
    fn shape(self) -> (usize, usize) {
        self.shape().clone()
    }
    // fn add(&mut self, v: &Matrix<K>);
    // fn sub(&mut self, v: &Matrix<K>);
    // fn scl(&mut self, a: K);
}

impl<'a, K> From<&'a [&'a [K]]> for Matrix<'a, K> {
    fn from(matrix: &'a [&'a [K]]) -> Matrix<K> {
        Matrix { matrix: matrix }
    }
}

impl<'a, K: fmt::Display> fmt::Display for Matrix<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for j in 0..(self.matrix.len()) {
            write!(f, "[");
            for i in 0..(self.matrix[0].len() - 1) {
                write!(f, "{}, ", self.matrix[j][i]);
            }
            write!(f, "{}]\n", self.matrix[j][self.matrix[0].len() - 1]);
        }
        Ok(())
    }
}
