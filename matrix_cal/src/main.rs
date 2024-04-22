extern crate ndarray;
extern crate ndarray_linalg;

use ndarray::Array2;
use ndarray_linalg::Inverse;  


fn main()
{
    let k = 3.0;
    let a = Array2::from_shape_vec((3, 3), vec![1.0, 0.0, 0.0, 0.0, 1.0, 1.9, 0.0, k, 1.9 ]).unwrap();
    let b = Array2::from_shape_vec((3, 3), vec![5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 8.0, 1.0, 2.0]).unwrap();
    let mut c = Array2::from_shape_vec((3, 3),vec![0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]).unwrap();

    // 행렬 곱셈
    let result1 = a.dot(&b);
    println!("A dot B:\n{}", result1);

    // 배열 덧셈
    let result2 = &a + &b;
    println!("A + B:\n{}", result2);

    match a.inv() {
        Ok(inv_a) => c=inv_a,
        Err(e) => println!("역행렬 계산 중 오류: {}", e),
    }
    println!("{}",c);
}