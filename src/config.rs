use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsString;

use std::process;
use std::{env, fs};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub data_dir: String,
    pub markdown_location: String,
    pub(crate) pages: Vec<Page>,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Serialize, Debug)]
enum AssetKind {
    JAVASCRIPT,
    STYLESHEET,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Asset {
    path: String,
    kind: AssetKind,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    title: String,
    pub markdown: String,
}

impl Config {
    pub fn new() -> Config {
        match Self::retrieve_stored_configuration() {
            Ok(file) => file,
            Err(_err) => {
                let cwd = Self::get_working_directory();
                Config {
                    host: "127.0.0.1".to_string(),
                    port: 8080,
                    data_dir: cwd.unwrap().to_string(),
                    assets: vec![],
                    pages: vec![
                        Page {
                            title: "Welcome to my internet space".to_string(),
                            markdown: "index.md".to_string(),
                        },
                        Page {
                            title: "Write_your_own_static_analyzer_for_PHP".to_string(),
                            markdown: "Write_your_own_static_analyzer_for_PHP.md".to_string(),
                        },
                        Page {
                            title: "How_I_made_impossible_to_write_spaghetti_code".to_string(),
                            markdown: "How_I_made_impossible_to_write_spaghetti_code.md"
                                .to_string(),
                        },
                        Page {
                            title: "Detecting_spaghetti_code_in_AST_of_a_PHP_source_code".to_string(),
                            markdown: "Detecting_spaghetti_code_in_AST_of_a_PHP_source_code.md"
                                .to_string(),
                        },
                        Page{
                            title:"improve_your_ci_output.".to_string(),
                            markdown: "Improve_your_CI_output.md".to_string(),
                        },
                        Page {
                            title: "Why_using_unserialize_PHP_is_a_bad_idea".to_string(),
                            markdown:"Why_using_unserialize_PHP_is_a_bad_idea.md".to_string(),
                        }

                    ],
                    markdown_location: "{markdown}".to_string(),
                }
            }
        }
    }

    /// Get the current working directory.
    fn get_working_directory() -> Result<String, OsString> {
        match env::current_dir() {
            Ok(p) => p.into_os_string().into_string(),
            _ => panic!("No working directory."),
        }
    }

    fn retrieve_stored_configuration() -> Result<Config, toml::de::Error> {
        let file = fs::read_to_string(format!(
            "{}/mserver.toml",
            Self::get_working_directory().unwrap()
        ))
        .unwrap_or("".to_string());
        toml::from_str(&file)
    }

    /// Store the configuration file.
    fn store(path: String, parsed: Config) {
        let result = match toml::to_string(&parsed) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Configuration file could't be saved: {}", e);
                process::exit(1);
            }
        };

        match fs::write(format!("{}/{}", path, &"mserver.toml"), result.as_bytes()) {
            Ok(_) => {
                println!("Creating a configuration file:");
            }
            Err(reason) => {
                println!("Configuration file couldn't be stored: {}", reason);
            }
        }
    }
}
