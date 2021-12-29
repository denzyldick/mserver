use std::borrow::Borrow;
use std::path::Path;
use directories::ProjectDirs;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
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

            let file = fs::read_to_string(
                path.join("mserver.toml")
            ).unwrap_or(
                "".to_string()
            );
            let parsed = toml::from_str(&file).unwrap_or(Config {
                ip: "127.0.0.1".to_string(),
                port: 8080,
                pages: vec![Page {
                    title: "Welcome to my internet space".to_string(),
                    markdown: "index.md".to_string(),
                }, Page {
                    title: "".to_string(),
                    markdown: "".to_string(),
                }],
            });
            /// todo do not override file if it already exits.

            let b = toml::to_string(&parsed).unwrap();
            println!("Configuration file has been written to: {}", path.to_str().unwrap());
            fs::write(path, b.as_bytes()).unwrap();
            return parsed;
        }
        panic!("No configuration file has been found.");
    }
}
