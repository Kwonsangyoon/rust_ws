use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    // 7878 포트에서 듣기
    let socket = UdpSocket::bind("0.0.0.0:7878")?;
    let mut buf = [0; 1024]; // 수신을 위한 버퍼

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;

        // 수신된 데이터를 UTF-8 문자열로 변환
        let received_data = match str::from_utf8(&buf[..amt]) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid UTF-8 sequence: {}", e);
                continue;
            },
        };

        println!("Received from {}: {}", src, received_data);
    }
}
