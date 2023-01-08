use matrix::matrix::Matrix;

fn main() {
    let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("{}", u.trace());
}

#[cfg(test)]
mod trace {
    use super::*;

    #[test]
    fn matrix_trace() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.trace(), 2.0);

        let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(u.trace(), 9.0);

        let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(u.trace(), -21.0);
    }
}
