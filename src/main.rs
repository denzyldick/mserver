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
use clap::Parser;

mod config;
mod server;

fn main() {
    let args = Args::parse();
    if args.detach == false {
        println!("Started");
        start();
    } else {
        let stdout = std::fs::File::create("./mserver.out").unwrap();
        let stderr = std::fs::File::create("./mserver.err").unwrap();
        let deamon = Daemonize::new()
            .pid_file("mserver.pid")
            .chown_pid_file(false)
            .working_directory("./")
            .umask(0o777)
            .stdout(stdout)
            .stderr(stderr)
            .exit_action(|| println!(""))
            .privileged_action(|| println!(""));
        match deamon.start() {
            Ok(_) => {
                println!("Starting");
                start();
            }
            Err(e) => {
                println!("Error, {}", e)
            }
        }
    }
}

fn start() {
    let config = crate::config::Config::new();
    println!("Listening for incoming connection on {}:{}", config.host, config.port);
    let mut routes = Routes::new();
    routes.listen_and_serve(config);
}


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    detach: bool,
}