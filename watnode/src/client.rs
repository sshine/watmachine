use std::io::{Read, Write};
use std::net::TcpStream;

pub fn connect(addr: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    println!("Connected to server!");
    let message = "Hello from the client!";
    stream.write(message.as_bytes())?;
    println!("Message sent: {}", message);

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[0..bytes_read]);
    println!("Server response: {}", response);

    Ok(())
}
