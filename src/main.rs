extern crate log;
extern crate serde;
extern crate serde_yaml;

use serde::*;
use std::collections::HashMap;
use std::fs::{read_to_string};
use log::{info};
use env_logger;
use std::env;
use std::process;

#[derive(Debug, Serialize, Deserialize)]
struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    pub sequence: Vec<HttpRequest>,
    pub users: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerSettings {
    pub name: String, 
    pub job: Job
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientSettings {
    url: String
}

fn run_server(settings: ServerSettings) -> process::ExitCode {
    info!("{:?}", settings);
    process::ExitCode::SUCCESS
}

fn run_client(settings: ClientSettings) -> process::ExitCode {
    info!("{:?}", settings);
    process::ExitCode::SUCCESS
}

fn usage(args: Vec<String>) {
    println!("
Usage: {} [options] [URL]

Options:
  -h, --help       Show this help message and exit
  -s, --server     Activate server mode
  -c, --client     Connect to a server and run job
", args[0]);
}

const MIN_ARGS: usize = 3;

const FIRST: usize = 1;
const SECOND: usize = 2;

fn main() -> process::ExitCode {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    // deal with the arguments first
    let args: Vec<String> = env::args().into_iter().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage(args);
        return process::ExitCode::FAILURE;
    }

    if args.len() < MIN_ARGS {
        usage(args);
        return process::ExitCode::FAILURE;
    }

    if args.len() == 2 && args[FIRST] == "--server" || args[FIRST] == "-s" {
        let yaml = read_to_string("./test.yaml").expect("Unable to read test.yaml");
        let settings: ServerSettings = serde_yaml::from_str(&yaml).expect("Unable to parse test.yaml");

        return run_server(settings);
    } else if args.len() == 3 && args[FIRST] == "--client" || args[FIRST] == "-c" {
        return run_client(ClientSettings { url: args[SECOND].to_owned() });
    }

    usage(args);
    return process::ExitCode::FAILURE;
}
