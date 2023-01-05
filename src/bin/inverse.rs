// use matrix::matrix::Matrix;

// fn main() {
//     let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
//     if let Ok(inverse) = u.inverse() {
//         println!("{}", inverse);
//     }
// }

// #[cfg(test)]
// mod inverse {
//     use super::*;

//     #[test]
//     fn matrix_inverse() {
//         let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
//         if let Ok(inverse) = u.inverse() {
//             assert_eq!(
//                 inverse,
//                 Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
//             );
//         }

//         let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
//         if let Ok(inverse) = u.inverse() {
//             assert_eq!(
//                 inverse,
//                 Matrix::from([[0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 0.5]])
//             );
//         }

//         let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
//         if let Ok(inverse) = u.inverse() {
//             assert_eq!(
//                 inverse,
//                 Matrix::from([
//                     [0.649425287, 0.097701149, -0.655172414],
//                     [-0.781609195, -0.126436782, 0.965517241],
//                     [0.143678161, 0.0747126454, -0.206896552]
//                 ])
//             );
//         }
//     }
// }
