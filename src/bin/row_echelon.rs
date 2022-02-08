use matrix::matrix::Matrix;

fn main() {
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);

    println!("{}", u);
    u.row_echelon();
    println!("{}", u);
}

#[cfg(test)]
mod row_echelon {
    use super::*;

    #[test]
    fn matrix_row_echelon() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );

        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 0.], [0., 1.]]));

        let u = Matrix::from([[1., 2.], [2., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 0.]]));

        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([
                [1., 0.625, 0., 0., -12.1666667],
                [0., 0., 1., 0., -3.6666667],
                [0., 0., 0., 1., 29.5],
            ])
        );
    }
}
