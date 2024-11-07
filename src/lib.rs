use std::{fmt::Debug, ops::{Add, Div, Mul, Sub}};

/// Vector Subtraction
///
/// Subtract two vectors.
///
/// # Examples
///
/// ```
/// use vector_operations::sub;
///
/// let a = [1, 2];
/// let b = [5, 4];
/// let expected = [-4, -2];
/// assert_eq!(sub(&a, &b), expected);
/// ```
///
/// # Panics
///
/// This function will panic if the lengths of the two vectors are not equal.
///
/// # Type Parameters
///
/// - `F`: The length of the vectors.
///
/// # Arguments
///
/// - `vec_a`: The first vector.
/// - `vec_b`: The second vector.
///
/// # Returns
///
/// A new vector containing the difference of the two input vectors.
pub fn sub<'a, 'b, const F: usize, T: Sub<Output = T> + Div<Output = T> + Debug + Copy>(vec_a: &'a [T; F], vec_b: &'b [T; F]) -> [T; F]
where 
    &'a T: Sub<&'a T>
{
    vec_a
        .iter()
        .zip(vec_b.iter())
        .map(|(a, b)| *a - *b)
        .collect::<Vec<T>>()
        .try_into()
        .unwrap()
}

/// Vector Addition
///
/// Add two vectors together.
///
/// # Examples
///
/// ```
/// use vector_operations::add;
///
/// let a = [1, 2, 3, 4, 5];
/// let b = [5, 4, 3, 2, 1];
/// let expected = [6, 6, 6, 6, 6];
/// assert_eq!(add(&a, &b), expected);
/// ```
///
/// # Panics
///
/// This function will panic if the lengths of the two vectors are not equal.
///
/// # Type Parameters
///
/// - `F`: The length of the vectors.
///
/// # Arguments
///
/// - `vec_a`: The first vector.
/// - `vec_b`: The second vector.
///
/// # Returns
///
/// A new vector containing the sum of the two input vectors.
pub fn add<'a, 'b, const F: usize, T: Add<Output = T> + Div<Output = T> + Debug + Copy>(vec_a: &'a [T; F], vec_b: &'b [T; F]) -> [T; F]
where 
    &'a T: Add<&'a T>
{
    vec_a
        .iter()
        .zip(vec_b.iter())
        .map(|(a, b)| *a + *b)
        .collect::<Vec<T>>()
        .try_into()
        .unwrap()
}

/// Vector Scaling
///
/// Scale the Vector a specified amount.
///
/// # Examples
///
/// ```
/// use vector_operations::scale;
///
/// let a = [1, 2, 3, 4, 5];
/// let expected = [5, 10, 15, 20, 25];
/// assert_eq!(scale(&a, &5), expected);
/// ```
///
/// # Type Parameters
///
/// - `F`: The length of the vector.
///
/// # Arguments
///
/// - `vec`: The vector to scale.
/// - `scalar`: The scalar value to multiply the vector by.
///
/// # Returns
///
/// A new vector containing the scaled values of the input vector.
pub fn scale<'a, const F: usize, T: Mul<Output = T> + Div<Output = T> + Debug + Copy>(vec: &'a [T; F], scalar: &T) -> [T; F]
where 
    &'a T: Mul<&'a T>
{
    vec.iter()
        .map(|a| *a * *scalar)
        .collect::<Vec<T>>()
        .try_into()
        .unwrap()
}

/// Matrix Vector Multiplication
///
/// Multiply the vector by the matrix.
///
/// # Examples
///
/// ```
/// use vector_operations::matrix_vec_multiply;
///
/// let matrix = [[1, 2], [-3, 4]];
/// let vector = [5, 7];
/// let expected = [-16, 38];
/// assert_eq!(matrix_vec_multiply(&matrix, &vector), expected);
/// ```
///
/// # Type Parameters
///
/// - `M`: The number of rows in the matrix.
/// - `N`: The number of columns in the matrix.
///
/// # Arguments
///
/// - `matrix`: The matrix to multiply.
/// - `vector`: The vector to multiply with the matrix.
///
/// # Returns
///
/// A new vector containing the result of multiplying the matrix and vector.
pub fn matrix_vec_multiply<'a, 'b, const M: usize, const N: usize, T: Mul<Output = T> + Add<Output = T> + Debug + Copy + Default + std::ops::AddAssign>(
    matrix: &'a [[T; N]; M],
    vector: &'b [T; N],
) -> [T; M] {
    let zero: T = T::default();
    let mut result: [T; M] = [zero; M];
    for i in 0..M {
        for j in 0..N {
            result[i] += matrix[j][i] * vector[j];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let a: [i32; 2] = [1, 2];
        let b = [5, 4];
        let expectedi32 = [-4, -2];
        assert_eq!(sub(&a, &b), expectedi32);
        let c: [i64; 2] = [1, 2];
        let d = [5, 4];
        let expectedi64 = [-4, -2];
        assert_eq!(sub(&c, &d), expectedi64);
    }

    #[test]
    fn test_add() {
        let a = [1, 2, 3, 4, 5];
        let b = [5, 4, 3, 2, 1];
        let expected = [6, 6, 6, 6, 6];
        assert_eq!(add(&a, &b), expected);
    }

    #[test]
    fn test_scale() {
        let a = [1, 2, 3, 4, 5];
        let expected = [5, 10, 15, 20, 25];
        assert_eq!(scale(&a, &5), expected);
    }

    #[test]
    fn test_matrix_vec_multiply() {
        let matrix = [[1, 2], [-3, 4]];
        let vector = [5, 7];
        let expected = [-16, 38];
        assert_eq!(matrix_vec_multiply(&matrix, &vector), expected);
    }
}
