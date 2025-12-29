use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

/// Recieves Incoming Requests
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_connection(stream);
    }
}

/// Handles Connection To the Server 
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Handles the main page and treat any other page as an error 
    let (status_line,filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
        } else {
            ("HTTP/1.1 400 404 PAGE NOT FOUND", "404.html")
        };

        // Opens and Read Html File 
        let contents = fs::read_to_string(filename).unwrap();

        let length = contents.len();

        let response = 
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // Returns Message to the Server 
        stream.write_all(response.as_bytes()).unwrap();
    }

