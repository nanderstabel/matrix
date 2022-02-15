use matrix::vector::Vector;

fn main() {
    let mut u = Vector::from([-5., -4., 4.]);

    println!("{}", u.norm_inf());
}

#[cfg(test)]
mod norm {
    use super::*;

    #[test]
    fn vector_norm() {
        let mut u = Vector::from([0., 0., 0.]);
        assert_eq!((u.norm_1(), u.norm(), u.norm_inf()), (0., 0., 0.));

        let mut u = Vector::from([1., 2., 3.]);
        assert_eq!((u.norm_1(), u.norm(), u.norm_inf()), (6., 3.74165738, 3.));

        let mut u = Vector::from([-1., -2.]);
        assert_eq!((u.norm_1(), u.norm(), u.norm_inf()), (3., 2.236067977, 2.));
    }
}
