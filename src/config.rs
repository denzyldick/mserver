use directories::ProjectDirs;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Borrow;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub data_dir: String,
    pub(crate) pages: Vec<Page>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    title: String,
    pub markdown: String,
}

impl Config {
    pub fn new() -> Config {
        if let Some(project_dirs) = ProjectDirs::from("io", "denzyl", "mserver") {
            let path = project_dirs.config_dir();
            let data = project_dirs.data_dir().to_str().unwrap();
            let result = fs::create_dir(data);
            match result {
                Result::Ok(T) => {
                    println!("Data directory has been created")
                }
                Result::Err(E) if E.kind() == ErrorKind::AlreadyExists => {
                    println!("{} already exists", data);
                }
                Result::Err(E) => {
                    panic!("{}", E)
                }
            }

            let file = fs::read_to_string(path.join("mserver.toml")).unwrap_or("".to_string());
            let parsed = toml::from_str(&file).unwrap_or(Config {
                ip: "127.0.0.1".to_string(),
                port: 8080,
                data_dir: data.to_string(),
                pages: vec![
                    Page {
                        title: "Welcome to my internet space".to_string(),
                        markdown: "index.md".to_string(),
                    }
                ],
            });
            /// todo do not override file if it already exits.
            let b = toml::to_string(&parsed).unwrap();
            println!(
                "Configuration file has been written to: {}",
                path.to_str().unwrap()
            );
            println!("Looking for markdown files in: {}", data.to_string());
            fs::write(path, b.as_bytes()).unwrap();
            return parsed;
        }
        panic!("No configuration file has been found.");
    }
}
