use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let dt = 0.001f32;
    let mut local_accelation;
    let mut local_position = 0.0f32;
    let mut local_velocity = 0.0f32;


    loop 
    {
        local_accelation = spring_damper(local_position,local_velocity,1.0f32);
        local_velocity += local_accelation*dt;
        local_position += local_velocity*dt;
        println!("{}",local_position);
        thread::sleep(Duration::from_millis(10));

    }
}


fn spring_damper(_local_position: f32 , _local_velocity: f32, _external_force: f32) -> f32 
{
    let k = 1.0f32;
    let c = 0.707;
    let m = 1.0f32;

    let result = (-k*_local_position - c*_local_velocity + _external_force)/m;

    return result;
}