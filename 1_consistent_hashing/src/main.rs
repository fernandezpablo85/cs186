use clap::Parser;
use rand::Rng;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
mod server;

#[derive(Deserialize)]
struct Node {
    id: String,
    port: String,
}

#[derive(Parser, PartialEq, Eq, Debug)]
enum ClientCommand {
    #[command(about = "Ping random server")]
    Ping,
    #[command(about = "Generate bcrypt hash from plaintext")]
    Hash { plain: String },
    #[command(about = "Run in server mode")]
    Server,
}

impl ClientCommand {
    fn to_string(&self) -> String {
        match self {
            Self::Ping => String::from("PING"),
            Self::Hash { plain } => format!("HASH {plain}"),
            Self::Server => String::from("SERVER"),
        }
    }
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Pablo Fernandez")]
struct Cli {
    #[clap(subcommand)]
    pub cmd: ClientCommand,
}

fn main() {
    let args = Cli::parse();
    if args.cmd == ClientCommand::Server {
        server::server()
    } else {
        client(args)
    }
}

fn client(cli: Cli) {
    if cli.cmd == ClientCommand::Server {
        panic!("can't happen, we're running in client mode")
    }
    let host = read_nodes().map(|nodes| random_host(&nodes)).unwrap();
    match send(&host, &cli.cmd) {
        Ok(response) => println!("{} ✅ success: {}", host, response),
        Err(err) => eprintln!("{} ❌ server error '{}'", host, err),
    }
}

fn random_host(nodes: &Vec<Node>) -> String {
    let random_index = rand::thread_rng().gen_range(0..nodes.len() - 1);
    let node = &nodes[random_index];
    format!("0.0.0.0:{}", node.port)
}

fn read_nodes() -> std::io::Result<Vec<Node>> {
    let mut file = File::open("nodes.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let nodes = serde_json::from_str(&contents)?;
    Ok(nodes)
}

fn send(host: &str, command: &ClientCommand) -> std::io::Result<String> {
    let command = command.to_string();
    eprintln!("client > {command}");
    let mut stream = TcpStream::connect(host)?;
    stream.write_all(command.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    eprintln!("{} < {response}", host);
    Ok(response)
}
