use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

//FileSystem
use std::fs;

pub fn web_listener() {
    //Binding to a port below 1023 requires admin privileges
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

//HTTP Request format
//Method Request-URI HTTP-Version CRLF
//headers CRLF
//message-body

//HTTP Response
//HTTP-Version Status-Code Reason-Phrase CRLF
//header CRLF
//message-body
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    //The relative directory goes from the base of the project (not src)
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}