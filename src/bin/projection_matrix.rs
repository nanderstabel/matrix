use matrix::matrix::Matrix;

/// This function computes a projection matrix which can be used to demonstrate the renering of 3D objects.
fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    Matrix::from([
        [fov / ratio, 0., 0., 0.],
        [0., fov, 0., 0.],
        [
            0.,
            0.,
            ((far + near) / (near - far)),
            ((2. * far * near) / (near - far)),
        ],
        [0., 0., -1., 0.],
    ])
}

fn main() {
    print!("{}", projection(45., 30. / 40., 25., 300.));
}
