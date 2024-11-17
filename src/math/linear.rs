// Linear Algebra, special thanks to Alireza Sayyidmousavi


#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<T>>
}



impl<T> Matrix<T> // todo: block non numeric types from existing here
where
T:Default + Clone
{
    pub fn new(width: usize, height: usize) -> Matrix<T> {
        let data = vec![vec![T::default(); width]; height];

        Matrix {
            width,
            height,
            data
        }
    }
}

pub fn transpose<T>(matrix: Matrix<T>) -> Matrix<T> {

    if matrix.width * matrix.height == 0 {
        matrix
    }

    else {
        matrix
    }

}