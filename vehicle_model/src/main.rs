extern crate ndarray;

use ndarray::{Array2, concatenate, Axis};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Vehicle Dynamics Model Opened");

    //let k =0.0f32;
    let mut ddot_position = Array2::from_shape_vec((6,1),vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
    let mut dot_position = Array2::from_shape_vec((6,1),vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
    let mut position = Array2::from_shape_vec((6,1),vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
    let external_force = Array2::from_shape_vec((6,1),vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
    let mut vehicle_position = Array2::from_shape_vec((6,1),vec![3.0, 0.0, 0.0, 0.0, 0.0, 3.141592]).unwrap();
    let mut camera_position = Array2::from_shape_vec((6,1),vec![1.0, 0.0, 0.0, 0.0, 0.0, 3.141592]).unwrap();
    

    loop
    {   
        position = integral(&position,&dot_position,0.001f32);
        dot_position = integral(&dot_position,&ddot_position,0.001f32);
        ddot_position = dynamics_model(&dot_position,&position,&external_force);
        
        //coordinate_transform(&vehicle_position,&camera_position);
        println!("{}\n",coordinate_transform(&vehicle_position,&camera_position));
        //println!("{}\n",position);
        thread::sleep(Duration::from_millis(1));
    }
}





fn dynamics_model(_dot_variable: &Array2<f32>, _variable: &Array2<f32>, _external_input: &Array2<f32>) -> Array2<f32>
{
    let result = -_dot_variable  - _variable + _external_input;

    return result;
}

fn integral(_variable: &Array2<f32>, _dot_variable: &Array2<f32>,_delta_time: f32) -> Array2<f32>
{
    let result = _variable + _dot_variable*_delta_time;
    return result;
}

fn coordinate_transform(_parent_coordinate: &Array2<f32>, _child_coordinate: &Array2<f32>) -> Array2<f32>
{
    let mut result;

    let translation = Array2::from_shape_vec((3, 1), vec![
        _child_coordinate[(0, 0)], 
        _child_coordinate[(1, 0)], 
        _child_coordinate[(2, 0)]
    ]).unwrap(); // unwrap() 추가

    let rotation = Array2::from_shape_vec((3, 1), vec![
        _child_coordinate[(3, 0)], 
        _child_coordinate[(4, 0)], 
        _child_coordinate[(5, 0)]
    ]).unwrap(); // unwrap() 추가

    // rotation_matrix 함수의 결과를 직접 사용
    let rotation_matrix_result = rotation_matrix(_parent_coordinate); // 가정: 이 함수가 Array2<f32> 반환

    let trans_result = Array2::from_shape_vec((3, 1), vec![
        _parent_coordinate[(0, 0)], 
        _parent_coordinate[(1, 0)], 
        _parent_coordinate[(2, 0)]
    ]).unwrap() + rotation_matrix_result.dot(&translation); // unwrap() 추가

    let rot_result = Array2::from_shape_vec((3, 1), vec![
        _parent_coordinate[(3, 0)], 
        _parent_coordinate[(4, 0)], 
        _parent_coordinate[(5, 0)]
    ]).unwrap() + rotation_matrix_result.dot(&rotation); // unwrap() 추가

    result = concatenate(Axis(0), &[trans_result.view(), rot_result.view()]).unwrap();
    //result.extend(rot_result);

    return result;
}

fn rotation_matrix(_vector: &Array2<f32>) -> Array2<f32>
{
    let mut result;
    
    let roll = _vector[(3,0)];
    let pitch = _vector[(4,0)];
    let yaw = _vector[(5,0)];

    let rroll = Array2::from_shape_vec((3,3),
        vec![
            1.0f32, 0.0f32, 0.0f32,
            0.0f32, f32::cos(roll), -f32::sin(roll),
            0.0f32, f32::sin(roll), f32::cos(roll)
            ]
    ).unwrap();
    let rpitch = Array2::from_shape_vec((3,3),
        vec![ 
             f32::cos(pitch), 0.0f32, f32::sin(pitch),
             0.0f32,          1.0f32, 0.0f32,
            -f32::sin(pitch), 0.0f32, f32::cos(pitch)
             ]
    ).unwrap();
    let ryaw = Array2::from_shape_vec((3,3),
        vec![
             f32::cos(yaw), -f32::sin(yaw), 0.0f32,
             f32::sin(yaw),  f32::cos(yaw), 0.0f32,
             0.0f32,           0.0f32,          1.0f32
            ]
    ).unwrap();

    result = ryaw.dot(&(rpitch.dot(&rroll)));
    
    //println!("{}\n",result);
    return result;

}