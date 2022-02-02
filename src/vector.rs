struct Vector::<K> {
    size: usize
}

impl<K> Vector<K> {
    fn add(&mut self, v: &Vector<K>);
    fn sub(&mut self, v: &Vector<K>);
    fn scl(&mut self, a: K);
    }