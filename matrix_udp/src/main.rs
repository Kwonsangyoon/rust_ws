extern crate ndarray;

use ndarray::Array2;
use std::net::UdpSocket;
use std::time::Duration;
use std::thread;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_address = "127.0.0.1:7878";
    let k = 3.0;
    let a = Array2::from_shape_vec((3, 3), vec![1.0, 2.0, k, 4.0, 1.0, 1.9, 4.0, 1.0, 1.9]).unwrap();
    let b = Array2::from_shape_vec((3, 3), vec![5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 8.0, 1.0, 2.0]).unwrap();

    // 행렬 곱셈
    let result1 = a.dot(&b);
    println!("A dot B:\n{}", result1);

    // 배열 덧셈
    let result2 = &a + &b;
    println!("A + B:\n{}", result2);

    loop {
        // 행렬을 문자열로 변환
        let message = format!("A dot B:\n{}", result1);

        // 문자열을 바이트 슬라이스로 변환하여 서버에 전송
        socket.send_to(message.as_bytes(), server_address)?;

        println!("Message sent to the server.");

        // 1초 동안 대기
        thread::sleep(Duration::from_secs(1));
    }
}
