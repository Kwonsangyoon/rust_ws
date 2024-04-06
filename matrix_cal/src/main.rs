extern crate ndarray;

use ndarray::Array2;

fn main()
{
    let k = 3.0;
    let a = Array2::from_shape_vec((3, 3), vec![1.0, 2.0, k, 4.0, 1.0, 1.9, 4.0, 1.0, 1.9 ]).unwrap();
    let b = Array2::from_shape_vec((3, 3), vec![5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 8.0, 1.0, 2.0]).unwrap();

    // 행렬 곱셈
    let result1 = a.dot(&b);
    println!("A dot B:\n{}", result1);

    // 배열 덧셈
    let result2 = &a + &b;
    println!("A + B:\n{}", result2);

    loop {
        // 서버에 메시지 전송
        socket.send_to(result1, server_address)?;

        println!("send message.");

        // 1초 동안 대기
        thread::sleep(Duration::from_secs(1));
    }
}