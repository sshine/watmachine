use anyhow::{Result, ensure};
use std::fs::{File, metadata};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn connect(addr: &str, filename: &str) -> Result<()> {
    // Do some basic checks
    ensure!(!filename.is_empty(), "Filename cannot be empty");

    ensure!(filename.len() <= 255, "Filename is too long");

    ensure!(
        filename.ends_with(".wasm"),
        "File '{}' does not have a .wasm extension",
        filename
    );

    let metadata = metadata(filename)?;

    ensure!(
        !metadata.is_dir(),
        "'{}' is a directory, not a .wasm file",
        filename
    );

    // Read file
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let file_length = contents.len() as u32;

    println!("File length: {}", file_length);

    // Connect to the server
    let mut stream = TcpStream::connect(addr)?;
    println!("Connected to server!");

    // Send length as 4-byte big endian
    stream.write_all(&file_length.to_be_bytes())?;
    // Send the actual wasm bytes
    stream.write_all(&contents)?;
    println!("File sent!");

    // Read the server's response
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[0..bytes_read]);
    println!("Server response: {}", response);

    Ok(())
}
