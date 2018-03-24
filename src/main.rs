use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");

        connection_handler(stream);
    }
}

fn connection_handler(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let request = b"GET / HTTP/1.1\r\n";

    if (buffer.starts_with(request)) {
        let mut file = File::open("index.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let mut error_file = File::open("teapot.html").unwrap();
        let mut error = String::new();
        error_file.read_to_string(&mut error).unwrap();
        let response = format!("HTTP/1.1 418 I'm a teapot\r\n\r\n{}", error);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
