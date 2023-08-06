use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
mod server;

#[derive(Parser, PartialEq, Eq, Debug)]
pub enum ClientCommand {
    #[command(about = "Ping random server")]
    Ping,
    #[command(about = "Generate bcrypt hash from plaintext")]
    Hash { plain: String },
    #[command(about = "Run in server mode")]
    Server,
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Pablo Fernandez")]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: ClientCommand,
}

fn main() {
    let args = Cli::parse();
    if args.cmd == ClientCommand::Server {
        server::server()
    } else {
        let nodes = parse_nodes_json();
        client(args)
    }
}

fn client(cli: Cli) {
    match cli.cmd {
        ClientCommand::Ping => {
            let node = random_node();
            send(node, "PING");
        }
        ClientCommand::Hash { plain } => {
            let node = random_node();
            send(node, format!("HASH {}", plain));
        }
        ClientCommand::Server => {
            panic!("can't happen, we're running in client mode")
        }
    }
}

fn random_node() -> String {
    let mut file = File::open("nodes.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let nodes: Vec<Node> = serde_json::from_str(&contents).expect("Error parsing the file");
}
