use clap::Parser;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

// On a 32gb Apple M1 Pro
// A cost of 12 takes about ~1 second
// A cost of 15 takes about ~7 seconds
const BCRYPT_COST: u32 = 16;
const SERVER_PORT: u32 = 8888;

enum Command<'a> {
    Ping,
    Hash(&'a str),
    Exit,
    Err,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    /// Run in server mode
    server: bool,
}

fn main() {
    let args = Args::parse();
    if args.server {
        server()
    } else {
        client(args)
    }
}

fn server() {
    let addr = format!("0.0.0.0:{SERVER_PORT}");
    let listener = TcpListener::bind(&addr);
    match listener {
        Err(err) => {
            eprintln!("can't connect to port {SERVER_PORT}, err: {:?}", err)
        }
        Ok(listener) => {
            println!("listening on {addr}");
            for stream in listener.incoming() {
                match stream {
                    Ok(req) => handle_req(req),
                    Err(err) => eprintln!("error while receiving req: {}", err),
                }
            }
        }
    }
}

fn handle_req(mut socket: TcpStream) {
    let mut buffer = vec![0; 256];
    socket.read(&mut buffer).expect("couldn't read from buffer");

    let request = String::from_utf8_lossy(&buffer[..]);
    let command = parse(request.as_ref());
    match command {
        Command::Ping => {
            socket.write(b"PONG\n").unwrap();
        }
        Command::Hash(plain) => {
            let h = hash(plain);
            socket.write(format!("{h}\n").as_bytes()).unwrap();
        }
        Command::Exit => panic!("EXIT"),
        Command::Err => {
            socket
                .write(format!("ERR: {}\n", request).as_bytes())
                .unwrap();
        }
    }
    socket.flush().unwrap()
}

fn hash(plain: &str) -> String {
    println!("hashing: {}", plain);
    let tic = Instant::now();
    let hash = bcrypt::hash(plain, BCRYPT_COST).unwrap();
    let toc = tic.elapsed().as_millis();
    println!("result:  {}", hash);
    println!("elapsed: {}ms", toc);
    hash
}

fn parse(req: &str) -> Command {
    let parts: Vec<&str> = req.trim().split(" ").collect();
    let cmd = parts[0];
    match cmd.to_uppercase().as_str() {
        "PING" => Command::Ping,
        "HASH" if parts.len() > 1 => Command::Hash(parts[1]),
        "EXIT" => Command::Exit,
        _ => Command::Err,
    }
}

fn client(_args: Args) {
    println!("running in client mode")
}
