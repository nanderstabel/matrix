use matrix::matrix::Matrix;

fn main() {
    let mut u = Matrix::from([
        [-1., 2., 4., 6.],
        [0., 0., 1., 7.],
        [-1., 2., 4., 14.],
        [0., 2., 4., 6.],
    ]);
    println!("{}", u);
    println!("{}", u.determinant());
    println!("{}", u.trace());
}

#[cfg(test)]
mod determinant {
    use super::*;

    #[test]
    fn matrix_determinant() {
        let mut u = Matrix::from([[1., -1.], [-1., 1.]]);
        assert_eq!(u.determinant(), 0.0);

        let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(u.determinant(), 8.0);

        let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_eq!(u.determinant(), -174.0);

        //     let mut u = Matrix::from([
        //         [8., 5., -2., 4.],
        //         [4., 2.5, 20., 4.],
        //         [8., 5., 1., 4.],
        //         [28., -4., 17., 1.],
        //     ]);
        //     assert_eq!(u.determinant(), 1032.0);
    }
}
