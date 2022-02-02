struct Matrix::<K> {
    shape: (usize, usize)
}

impl<K> Matrix<K> {
    fn add(&mut self, v: &Matrix<K>);
    fn sub(&mut self, v: &Matrix<K>);
    fn scl(&mut self, a: K);
    }