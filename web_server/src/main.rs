use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listiner = TcpListener::bind("127.0.0.1:42068").unwrap(); // almost
    for stream in listiner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_method = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, content) = if http_method == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    println!("{status_line}");
    let content = fs::read_to_string(content).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap()
}
