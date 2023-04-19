pub type Vector = nalgebra::SVector<f32, 3>;
pub trait Normalize {
    fn normalise(self) -> Self;
}

impl Normalize for Vector {
    fn normalise(self) -> Vector {
        self / self.norm()
    }
}

macro_rules! vector {
    ($x: expr, $y:expr, $z: expr) => {{
        Vector::from([$x as f32, $y as f32, $z as f32])
    }};
}
pub(crate) use vector;

pub type Matrix = nalgebra::SMatrix<f32, 3, 3>;

pub fn matrix_from_columns(columns: [Vector; 3]) -> Matrix {
    Matrix::from_columns(&columns)
}

#[cfg(test)]
mod test_math {
    use super::{Matrix, Normalize, Vector};
    #[test]
    fn add_arrays() {
        let a = Vector::from_vec(vec![1., 2., 3.]);
        let b = Vector::from_vec(vec![4., 2., 3.]);
        let x = a + b;

        // println!("{:?}", (&b.shape()).into_iter());
        let res = Vector::from_vec(vec![5., 4., 6.]);
        assert!(x == res);
        println!("{}, {}", a, b);
    }
    #[test]
    fn test_vector_macro() {
        let a = vector!(1, 2, 3);
        assert!(a == Vector::from_vec(vec![1., 2., 3.]))
    }
    #[test]
    fn test_normalize_trait() {
        let difference = vector!(3, 4, 5).normalise() - vector!(0.42426, 0.56568, 0.7071);
        assert!(difference.dot(&difference) < 0.001)
    }
    #[test]
    fn test_matrix_from_col() {
        let A = super::matrix_from_columns([vector!(2, 0, 0), vector!(0, 2, 0), vector!(0, 0, 2)]);
        assert!(A * A == 2 as f32 * A)
    }
}
// macro_rules! min {
//     ($x:expr) => ($x);
//     ($x:expr, $($y:expr),+) => {{
//         let a = min!($($y),+);
//         if a < $x {
//             a
//         } else {
//             $x
//         }
//     }};
// }

// macro_rules! max {
//     ($x:expr) => ($x);
//     ($x:expr, $($y:expr),+) => {{
//         let a = max!($($y),+);
//         if a > $x {
//             a
//         } else {
//             $x
//         }
//     }};
// }
// pub(crate) use {max, min};
