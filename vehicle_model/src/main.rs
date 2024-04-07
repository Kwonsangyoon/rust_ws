extern crate ndarray;

use ndarray::Array2;


fn main() {
    println!("Vehicle Dynamics Model Opened");

    let mut ddot_position = Array2::from_shape_vec((6,1),vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32]).unwrap();
    let mut dot_position = Array2::from_shape_vec((6,1),vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32]).unwrap();
    let mut position = Array2::from_shape_vec((6,1),vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32]).unwrap();


}



