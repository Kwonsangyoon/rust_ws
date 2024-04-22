extern crate ndarray;

use ndarray::{Array2, arr1};

struct Position
{
    x : f32,
    y : f32,
    z : f32,
}

struct Orientation
{
    q_x : f32,
    q_y : f32,
    q_z : f32,
    q_w : f32,
}

struct Pose
{
    point : Position,
    quaternion : Orientation,
}



fn main()
{
    let test = Pose
    {
        point: Position{
            x: 1.0f32,
            y: 2.0f32,
            z: 3.0f32,
        },
        quaternion: Orientation{
            q_x: 0.0f32,
            q_y: 0.0f32,
            q_z: 0.0f32,
            q_w: 1.0f32,
        },
    };
    
    println!("{}",quaternion_to_transform(test));
}



fn quaternion_to_transform( _object_position : Pose) -> Array2<f32>
{
    
    let q = Array2::from_shape_vec((4, 4), vec![ _object_position.quaternion.q_w,  _object_position.quaternion.q_z, -_object_position.quaternion.q_y,  _object_position.quaternion.q_x,
                                                -_object_position.quaternion.q_z,  _object_position.quaternion.q_w,  _object_position.quaternion.q_x,  _object_position.quaternion.q_y,
                                                 _object_position.quaternion.q_y, -_object_position.quaternion.q_x,  _object_position.quaternion.q_w,  _object_position.quaternion.q_z,
                                                -_object_position.quaternion.q_x, -_object_position.quaternion.q_y, -_object_position.quaternion.q_z,  _object_position.quaternion.q_w]).unwrap();
    let mut result = q.dot(&q);

    result.column_mut(3).assign(&arr1(&[_object_position.point.x, _object_position.point.y, _object_position.point.z, 1.0]));

    return result;
}