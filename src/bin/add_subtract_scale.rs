use matrix::{matrix::Matrix, vector::Vector, VectorSpace};

#[allow(dead_code)]
fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u._add(&v);
    println!("{}", Vector::from([7., 10.]));
    println!("{}", Vector::from([7., 10.]));
}

#[cfg(test)]
mod add_subtract_scale {
    use super::*;

    #[test]
    fn vector_add() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u._add(&v);
        assert_eq!(u, Vector::from([7., 10.]));

        let mut u = Vector::from([-2., 3.]);
        let v = Vector::from([5., 7.]);
        u._add(&v);
        assert_eq!(u, Vector::from([3., 10.]));

        let mut u = Vector::<f32>::from([]);
        let v = Vector::from([]);
        u._add(&v);
        assert_eq!(u, Vector::from([]));
    }

    #[test]
    fn matrix_add() {
        let mut m = Matrix::from([[1., 2.], [3., 4.]]);
        let ma = Matrix::from([[7., 4.], [-2., 2.]]);
        m._add(&ma);
        assert_eq!(m, Matrix::from([[8., 6.], [1., 6.]]));
    }
}
