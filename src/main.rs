use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8080")?;

    stream.write_all(b"Hello, World!")?;
    println!("Hello, world!");
    Ok(())
}
