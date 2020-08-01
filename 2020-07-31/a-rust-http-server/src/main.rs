use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{fs, io};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088")?;
    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    const GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let (status, filename) = if buffer.starts_with(GET) {
        ("HTTP/1.1 200 OK\r\n", "hola.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };

    let content = fs::read_to_string(filename)?;
    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );
    stream.write(response.as_bytes())?;
    stream.flush()
}
