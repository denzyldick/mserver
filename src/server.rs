use crate::config::{Config, Page};
use mserver::ThreadPool;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

pub struct Route {
    method: String,
    markdown: String,
}

impl Route {
    pub fn generate(&self) -> String {
        let config = Config::new();
        let path = format!("{}/{}", config.data_dir, self.markdown);
        println!("Path:{}", path);
        let contents = fs::read_to_string(path).unwrap();
        let body = markdown::to_html(&contents);
        format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title> {} </title></head><body>{}</body></html>", "welcome to my internet space.", body)
    }
}

pub struct Routes {
    routes: HashMap<String, Page>,
}

impl Routes {
    pub fn add(&mut self, route: Route) {
        todo!()
    }

    pub fn new() -> Routes {
        Routes {
            routes: Default::default(),
        }
    }

    pub fn listen_and_serve(&mut self, config: Config) {
        let addr = format!("{}:{}", config.host, config.port);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(20);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let get = b"GET";
            println!("{}", str::from_utf8(&buffer).unwrap());
            pool.execute(move || {
                let mut headers = [httparse::EMPTY_HEADER; 64];
                let mut req = httparse::Request::new(&mut headers);
                req.parse(&buffer).unwrap();
                let mut path = match req.path {
                    Some(t) => t,
                    _ => &"/",
                };
                if path == "/" {
                    path = "index"
                }
                let route = Self::find_markdown(path);
                match route {
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
                        println!("No route has been found.")
                    }
                }
            });
        }
    }

    fn find_markdown(path: &str) -> std::option::Option<Route> {
        let config = Config::new();
        for page in config.pages {
            let string = page.markdown.replace(".md", "");
            // @todo only remove the first item in the string.
            if path.replace("/", "") == string {
                let route = Some(Route {
                    method: "GET".to_string(),
                    markdown: page.markdown,
                });
                return route;
            }
        }
        None
    }
}
