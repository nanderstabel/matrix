use matrix::matrix::Matrix;

fn main() {
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);

    println!("{}", u);
    u.transpose();
    println!("{}", u);
}

#[cfg(test)]
mod transpose {
    use super::*;

    #[test]
    fn matrix_transpose() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.transpose(), Matrix::from([[1., 3.], [2., 4.]]));
    }
}
