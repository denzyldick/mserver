use crate::config::Config;
use crate::server::Routes;
use mserver::ThreadPool;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use daemonize::Daemonize;

mod config;
mod server;

fn main() {
    start();
    println!("Started");
    return;
    let stdout = std::fs::File::create("/tmp/mserver.out").unwrap();
    let stderr = std::fs::File::create("/tmp/mserver.err").unwrap();

    let deamon = Daemonize::new()
        .pid_file("/tmp/mserver.pid")
        .chown_pid_file(false)
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .exit_action(|| println!(""))
        .privileged_action(|| println!(""));

    match deamon.start() {
        Ok(_) => {
            start();
        }
        Err(e) => {
            println!("Error, {}", e)
        }
    }
}

fn start() {
    let config = crate::config::Config::new();
    println!("Listening for incoming connection on {}:{}", config.host, config.port);
    let mut routes = Routes::new();
    routes.listen_and_serve(config);
}
