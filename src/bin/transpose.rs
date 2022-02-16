use matrix::matrix::Matrix;

fn main() {
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("{}", u.transpose());
}

#[cfg(test)]
mod transpose {
    use super::*;

    #[test]
    fn matrix_transpose() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.transpose(), Matrix::from([[1., 3.], [2., 4.]]));

        let u = Matrix::from([[1., 2., 3.], [3., 4., 5.], [6., 7., 8.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[1., 3., 6.], [2., 4., 7.], [3., 5., 8.]])
        );
    }
}
