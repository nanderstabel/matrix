use matrix::matrix::Matrix;
use std::ops::{Add, Mul, Sub};

fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Clone + Mul<f32, Output = V> + Add<Output = V> + Sub<Output = V>,
{
    u.clone() + (v - u) * t
}

fn main() {
    let m1 = Matrix::from([[2., 1.], [3., 4.]]);
    let m2 = Matrix::from([[20., 10.], [30., 40.]]);
    let m3 = lerp(m1, m2, 0.5);
    println!("{}", m3);
}

#[cfg(test)]
mod lerp {
    use super::*;

    #[test]
    fn vector_lerp() {
        assert_eq!(lerp(0., 1., 0.), 0.0);
        assert_eq!(lerp(0., 1., 1.), 1.0);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);
        assert_eq!(
            lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3),
            Vector::from([2.6, 1.3])
        );
        assert_eq!(
            lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20., 10.], [30., 40.]]),
                0.5
            ),
            Matrix::from([[11., 5.5], [16.5, 22.]])
        );
    }
}
