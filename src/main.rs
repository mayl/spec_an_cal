use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8080")?;

    stream.write_all(b"Hello, World!")?; 
    let mut buf = [0; 10];
    let _n = stream.read(&mut buf[..]);
    let rx_val = String::from_utf8_lossy(&buf);
    println!("Received the bytes: {:?}", rx_val);
    Ok(())
}
