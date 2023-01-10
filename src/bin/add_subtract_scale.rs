use matrix::vector::Vector;

#[allow(dead_code)]
fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u += v;
    println!("{}\n", u);
}

#[cfg(test)]
mod add_subtract_scale {
    use super::*;
    use matrix::matrix::Matrix;

    #[test]
    fn test_vector_add() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u += v;
        assert_eq!(u, Vector::from([7., 10.]));
    }

    #[test]
    fn test_vector_sub() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u -= v;
        assert_eq!(u, Vector::from([-3., -4.]));
    }

    #[test]
    fn test_vector_scl() {
        let mut u = Vector::<f32>::from([2., 3.]);
        u *= 2.;
        assert_eq!(u, Vector::from([4.0, 6.0]));
    }

    #[test]
    fn test_matrix_add() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u += v;
        assert_eq!(u, Matrix::from([[8., 6.], [1., 6.]]));
    }

    #[test]
    fn test_matrix_sub() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u -= v;
        assert_eq!(u, Matrix::from([[-6., -2.], [5., 2.]]));
        println!("{}\n", u);
    }

    #[test]
    fn test_matrix_scl() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        u *= 2.;
        assert_eq!(u, Matrix::from([[2., 4.], [6., 8.]]));
    }
}
