use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use threadpool::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:2121").unwrap();
    let pool = ThreadPool::new(8);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, file_name) = routing(buffer);

    let content = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
        status_line,
        content.len(),
        "text/html",
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn routing(buffer: [u8; 1024]) -> (String, String) {
    let get_root = b"GET / HTTP/1.1\r\n";
    let get_about = b"GET /about HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK".to_string(), "./index.html".to_string())
    } else if buffer.starts_with(get_about) {
        ("HTTP/1.1 200 OK".to_string(), "./about.html".to_string())
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK".to_string(), "./index.html".to_string())
    } else {
        (
            "HTTP/1.1 404 NOT FOUND".to_string(),
            "./404.html".to_string(),
        )
    }
}
