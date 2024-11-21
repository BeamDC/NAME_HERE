// Linear Algebra, special thanks to Alireza Sayyidmousavi

use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<T>>
}

impl<T> Matrix<T>
where
T:  Default + Clone + Add<Output = T>
{
    pub fn new(width: usize, height: usize) -> Matrix<T> {
        let data = vec![vec![T::default(); width]; height];
        Matrix {
            width,
            height,
            data
        }
    }

    pub fn transpose(matrix: Matrix<T>) -> Matrix<T> {

        if matrix.width * matrix.height == 0 {
            matrix
        }

        else {
            matrix
        }

    }
}

