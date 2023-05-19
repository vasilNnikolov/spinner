pub type Vector = nalgebra::SVector<f32, 3>;

macro_rules! into_vec {
    ($x: expr) => {
        Into::<Vector>::into($x)
    };
}
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

fn proj(u: &Vector, v: &Vector) -> Vector {
    u.dot(v) / u.dot(u) * u
}
/// turns the input matrix into an orthogonal one, while keeping the direction of the first
/// element. WARNING: if `first_element` is greater than 2, the fn panics
pub fn gram_schmidt_orthogonalization(input: &Matrix, first_element: u8) -> Matrix {
    assert!(
        first_element <= 2,
        "the `first_element` of the gram_schmidt_orthogonalization fn has to be less than 3"
    );
    let input_columns: Vec<Vector> = input.column_iter().map(|v| into_vec!(v)).collect();
    let mut output_columns = input_columns.clone();
    for i in 0..3 {
        let current_column = input_columns[((i + first_element) % 3) as usize];
        let vector_to_subtract: Vector = (0..3)
            .filter(|&x| x < i)
            .map(|x| {
                proj(
                    &output_columns[((x + first_element) % 3) as usize],
                    &current_column,
                )
            })
            .sum();
        output_columns[((i + first_element) % 3) as usize] -= vector_to_subtract;
    }
    output_columns = output_columns.iter().map(|col| col.normalize()).collect();
    matrix_from_columns([output_columns[0], output_columns[1], output_columns[2]])
}

pub fn matrix_from_columns(columns: [Vector; 3]) -> Matrix {
    Matrix::from_columns(&columns)
}

#[cfg(test)]
mod test_math {
    use super::*;
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
    #[test]
    fn test_gso() {
        let gso = gram_schmidt_orthogonalization(&Matrix::identity(), 0);
        println!("{:?}", gso);
        assert!(gso == Matrix::identity());

        let gso = gram_schmidt_orthogonalization(&(2. * Matrix::identity()), 1);
        println!("{:?}", gso);
        assert!(gso == Matrix::identity());

        let gso = gram_schmidt_orthogonalization(
            &(matrix_from_columns([vector!(1, 1, 1), vector!(2, 1, -4), vector!(0, 1, 0)])),
            2,
        );
        println!("{:?}", gso);
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
