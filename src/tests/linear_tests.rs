use crate::math::linear::Matrix;

#[test]
fn matrix_type_test(){
    let mut a:Matrix<u32> = Matrix::new(3,3);
    assert_eq!(a.data, vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]]);

    let mut b:Matrix<i32> = Matrix::new(3,3);
    assert_eq!(b.data, vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]]);

    let mut c:Matrix<f32> = Matrix::new(3,3);
    assert_eq!(c.data, vec![vec![0.0,0.0,0.0],vec![0.0,0.0,0.0],vec![0.0,0.0,0.0]]);

    let mut d:Matrix<String> = Matrix::new(3,3);
    assert_eq!(d.data, vec![vec!["","",""],vec!["","",""],vec!["","",""]]);

}
