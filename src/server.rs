use std::net::{TcpListener, TcpStream};
use std::fs;
use std::io::{Read, Write};
use mserver::ThreadPool;
use crate::config::Config;

pub struct Route {
    method: String,
    markdown: String,
}

impl Route {
    pub fn generate(&self) -> String {
        let contents = fs::read_to_string(format!("pages/{}", self.markdown)).unwrap();
        let body = markdown::to_html(&contents);

        format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title> {} </title></head><body>{}</body></html>", "welcome to my internet space.", body)
    }
}

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn add(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn new() -> Routes {
        Routes {
            routes: vec![],
        }
    }

    pub fn listen_and_serve(&mut self, config: Config) {
        for page in config.pages {
            self.routes.push(
                Route {
                    method: "GET".to_string(),
                    markdown: page.markdown,
                }
            )
        }
        let addr = format!("{}:{}", config.ip, config.port);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(20);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let get = b"GET / HTTP / 1.1\r\n
        ";
            if buffer.starts_with(get) {
                pool.execute(move || {
                    let route = Route {
                        method: "GET".to_string(),
                        markdown: "index.md".to_string(),
                    };

                    match Some(route) {
                        Some(route) => {
                            let html = route.generate();
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                html.len(),
                                html
                            );
                            stream.write(response.as_bytes());
                            stream.flush();
                        }
                        None => {
                            println!("No
        route
        has
        been
        found.
        ")
                        }
                    }
                });
            }
        }
    }
}
