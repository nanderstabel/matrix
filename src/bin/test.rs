#[derive(Debug)]
#[allow(dead_code)]
pub struct Matrix<K> {
    matrix: Vec<Vec<K>>,
}

impl<K: Clone> Matrix<K> {
    pub fn from<T, Row>(matrix: T) -> Self
    where
        T: AsRef<[Row]>,
        Row: Clone + AsRef<[K]>,
    {
        Matrix {
            matrix: matrix
                .as_ref()
                .to_vec()
                .iter()
                .map(|x| x.as_ref().to_vec())
                .collect(),
        }
    }
}

// impl<K, T, Row> From<T> for Matrix<K>
// where
//     K: Clone,
//     T: AsRef<[Row]>,
//     Row: AsRef<[K]>,
// {
//     fn from(m: T) -> Self {
//         Matrix {
//             matrix: m
//                 .as_ref()
//                 .to_vec()
//                 .iter()
//                 .map(|x| x.as_ref().to_vec())
//                 .collect(),
//         }
//     }
// }

fn main() {
    let m = Matrix::from([[1, 2], [3, 4]]);
    println!("{:#?}", m);
}
