// use matrix::matrix::Matrix;
use matrix::{vector::Vector, VectorSpace};

#[allow(dead_code)]
fn main() {}

#[cfg(test)]
mod add_subtract_scale {
    use super::*;

    #[test]
    fn vector_add() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.add(&v);
        assert_eq!(u, Vector::from([7., 10.]));

        let mut u = Vector::from([-2., 3.]);
        let v = Vector::from([5., 7.]);
        u.add(&v);
        assert_eq!(u, Vector::from([3., 10.]));

        let mut u = Vector::<i32>::from([]);
        let v = Vector::from([]);
        u.add(&v);
        assert_eq!(u, Vector::from([]));
    }

    //     #[test]
    //     fn matrix_add() {
    //         let mut m = Matrix::from([[1., 2.], [3., 4.]]);
    //         let ma = Matrix::from([[7., 4.], [-2., 2.]]);
    //         m.add(&ma);
    //         assert_eq!(m, Matrix::from([[8., 6.], [1., 6.]]));
    //     }
}
