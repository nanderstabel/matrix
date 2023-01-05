// use matrix::matrix::Matrix;

// fn main() {
//     let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
//     println!("{}", u.rank());
// }

// #[cfg(test)]
// mod rank {
//     use super::*;

//     #[test]
//     fn matrix_rank() {
//         let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
//         assert_eq!(u.rank(), 3);

//         let mut u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
//         assert_eq!(u.rank(), 2);

//         let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
//         assert_eq!(u.rank(), 3);
//     }
// }
