use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;
use mserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(20);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle(stream);
        });
    }
    println!("Goodbye!");
}

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let response;
    if buffer.starts_with(get) {
        response = get_endpoint();
    } else {
        response = does_not_exists();
    }

    stream.write(response.as_bytes());
    stream.flush()
}

fn get_endpoint() -> String {
    let contents = fs::read_to_string("pages/index.md").unwrap();
    let html: String = markdown::to_html(&contents);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    );
    response
}

fn does_not_exists() -> String {
    let status = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("pages/404.md").unwrap();
    let html = markdown::to_html(&contents);
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        html.len(),
        html
    );
    response
}