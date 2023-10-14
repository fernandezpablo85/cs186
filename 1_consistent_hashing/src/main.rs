use clap::Parser;
use rand::Rng;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
mod server;

#[derive(Parser, PartialEq, Eq, Debug)]
enum ClientCommand {
    #[command(about = "Ping random server")]
    Ping,
    #[command(about = "Generate bcrypt hash from plaintext")]
    Hash { plain: String },
    #[command(about = "Run in server mode")]
    Server,
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
        // let nodes = parse_nodes_json();
        client(args)
    }
}

fn client(cli: Cli) {
    match cli.cmd {
        ClientCommand::Ping => {
            let port = random_port().unwrap();
            let command = "PING";
            println!("client > {command}");
            let res = send(&port, "PING").unwrap();
            println!("{} < {res}", host(&port));
        }
        ClientCommand::Hash { plain } => {
            let port = random_port().unwrap();
            let command = format!("HASH {plain}");
            println!("client > {command}");
            let res = send(&port, &command).unwrap();
            println!("{} < {res}", host(&port));
        }
        ClientCommand::Server => {
            panic!("can't happen, we're running in client mode")
        }
    }
}

fn host(port: &str) -> String {
    format!("0.0.0.0:{port}")
}

fn random_port() -> std::io::Result<String> {
    let mut file = File::open("nodes.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let nodes: Value = serde_json::from_str(&contents)?;
    let s = nodes.as_array().unwrap().len();
    let n = rand::thread_rng().gen_range(0..s);
    Ok(nodes[n]["port"].as_str().unwrap().to_string())
}

fn send(port: &str, command: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(host(port))?;
    stream.write_all(command.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}
