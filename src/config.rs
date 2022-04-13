use directories::ProjectDirs;
use serde::Deserialize;
use serde::Serialize;
use std::{env, fs};
use std::ffi::OsString;
use std::io::ErrorKind;


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub host: String,
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
            let cwd = Self::get_working_directory();
            let file = fs::read_to_string(format!("{}/mserver.toml", Self::get_working_directory().unwrap())).unwrap_or("".to_string());
            let parsed = toml::from_str(&file).unwrap_or(Config {
                host: "127.0.0.1".to_string(),
                port: 8080,
                data_dir: cwd.unwrap().to_string(),
                pages: vec![
                    Page {
                        title: "Welcome to my internet space".to_string(),
                        markdown: "index.md".to_string(),
                    },
                    Page {
                        title: "About me".to_string(),
                        markdown: "about.md".to_string(),
                    },
                ],
            });

            Self::store(Self::get_working_directory().unwrap().to_string(), &parsed);
            return parsed;
        }
        panic!("No configuration file has been found.");
    }

    fn get_working_directory() -> Result<String, OsString> {
        let cwd = match env::current_dir() {
            Ok(p) => { p.into_os_string().into_string() }
            _ => panic!("No working directory.")
        };
        cwd
    }

    fn store(path: String, parsed: &Config) {
        let string = format!("{}/{}", path, &"mserver.toml");
        let b = toml::to_string(&parsed).unwrap();
        match fs::write(string, b.as_bytes()) {
            Ok(_) => {
                println!("Creating a configuration file:");
            }
            Err(reason) => {
                println!("Configuration file couldn't be stored: {}", reason.to_string());
            }
        }
    }
}
