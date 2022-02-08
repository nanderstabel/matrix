use matrix::{vector::Matrix, vector::Vector};

fn main() {
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);

    println!("{}", u.dot(v.clone()));
}

#[cfg(test)]
mod linear_map {
    use super::*;

    #[test]
    fn matrix_linear_map() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        asser_eq!(u.mul_vec(&v), Vector::from([4., 2.]));

        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        asser_eq!(u.mul_vec(&v), Vector::from([8., 4.]));

        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        asser_eq!(u.mul_vec(&v), Vector::from([4., -4.]));
    }

    // #[test]
    // fn matrix_multiplication() {
    //     let u = Matrix::from([[1., 0.], [0., 1.]]);
    //     let v = Matrix::from([[1., 0.], [0., 1.]]);
    //     asser_eq!(u.mul_mat(&v), Matrix::from([[1., 0.], [0., 1.]]));

    //     let u = Matrix::from([[1., 0.], [0., 1.]]);
    //     let v = Matrix::from([[2., 1.], [4., 2.]]);
    //     asser_eq!(u.mul_mat(&v), Matrix::from([[2., 1.], [4., 2.]]));

    //     let u = Matrix::from([[3., -5.], [6., 8.]]);
    //     let v = Matrix::from([[2., 1.], [4., 2.]]);
    //     asser_eq!(u.mul_mat(&v), Matrix::from([[-14., -7.], [44., 22.]]));
    // }
}
