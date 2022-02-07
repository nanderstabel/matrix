use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Vector<K> {
    size: usize,
    vector: Vec<K>,
}

impl<K: Scalar<K>> Vector<K> {
    pub fn size(self) -> usize {
        self.size
    }

    pub fn dot(&self, v: Vector<K>) -> K {
        self.vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u * ve)
            .sum()
    }
}

impl<K: Scalar<K>> VectorSpace<Vector<K>, K> for Vector<K> {
    fn get(&mut self) -> Vec<K> {
        self.vector.clone()
    }

    fn _add(&mut self, v: &Vector<K>) {
        self.vector = self
            .vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u + ve)
            .collect()
    }

    fn _sub(&mut self, v: &Vector<K>) {
        self.vector = self
            .vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u - ve)
            .collect()
    }

    fn scl(&mut self, a: K) {
        self.vector = self.vector.iter().map(|&v| v * a).collect()
    }
}

impl<K: Scalar<K>> Add for Vector<K> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = self.clone();
        res._add(&other);
        res
    }
}

impl<K: Scalar<K>> Sub for Vector<K> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = self.clone();
        res._sub(&other);
        res
    }
}

impl Mul<f32> for Vector<f32> {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        let mut res = self.clone();
        res.scl(f);
        res
    }
}

impl<K, T> From<T> for Vector<K>
where
    K: Scalar<K>,
    T: AsRef<[K]>,
{
    fn from(v: T) -> Self {
        let v = v.as_ref().to_vec();
        Vector {
            size: v.len(),
            vector: v,
        }
    }
}

impl<K: Clone + PartialEq> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        self.vector == other.vector
    }
}

impl<K: Clone + PartialEq> Eq for Vector<K> {}

impl<K: std::fmt::Debug> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.vector)
    }
}

#[cfg(test)]
mod vector {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(Vector::<i32>::from(&[]).size(), 0);
        assert_eq!(Vector::from(&[1]).size(), 1);
        assert_eq!(Vector::from(&[3, 4]).size(), 2);
    }
}
