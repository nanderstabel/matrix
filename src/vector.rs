use std::{
    fmt,
    ops::{Add, Mul, Sub},
};

#[derive(Debug)]
pub struct Vector<K: Clone> {
    size: usize,
    vector: Vec<K>,
}

impl<K: Clone + Copy + Add<Output = K> + Sub<Output = K> + Mul<Output = K>> Vector<K> {
    pub fn from<T: AsRef<[K]>>(vector: T) -> Self {
        Vector {
            size: vector.as_ref().len(),
            vector: vector.as_ref().to_vec(),
        }
    }

    pub fn size(self) -> usize {
        self.size
    }

    pub fn add(&mut self, v: &Vector<K>) {
        self.vector = self
            .vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u + ve)
            .collect()
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        self.vector = self
            .vector
            .iter()
            .zip(v.vector.iter())
            .map(|(&u, &ve)| u - ve)
            .collect()
    }

    pub fn scl(&mut self, a: K) {
        self.vector = self.vector.iter().map(|&v| v * a).collect()
    }
}

impl<K: Clone + PartialEq> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        self.vector == other.vector
    }
}

impl<K: Clone + PartialEq> Eq for Vector<K> {}

impl<K: fmt::Display + Clone> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.size == 0 {
            write!(f, "[]")?;
        } else {
            for item in &self.vector {
                write!(f, "[{}]\n", item)?;
            }
        }
        Ok(())
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
