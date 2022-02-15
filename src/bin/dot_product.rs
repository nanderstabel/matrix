use matrix::vector::Vector;

fn main() {
    let v1 = Vector::from([2., 1.]);
    let v2 = Vector::from([20., 10.]);

    v1.dot(v2);

    println!("{}", v1);
}

#[cfg(test)]
mod dot_product {
    use super::*;

    #[test]
    fn vector_dot_product() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(u.dot(v), 0.0);

        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(u.dot(v), 2.0);

        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        assert_eq!(u.dot(v), 9.0);
    }
}
