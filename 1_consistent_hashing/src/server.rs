use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

// On a 32gb Apple M1 Pro
// A cost of 12 takes about ~1 second
// A cost of 15 takes about ~7 seconds
const BCRYPT_COST: u32 = 15;
const SERVER_PORT: u32 = 8888;

enum ServerCommand {
    Ping,
    Hash(String),
}

impl ServerCommand {
    fn new(input: &str) -> Option<Self> {
        if input.trim().len() == 0 {
            return None;
        }
        let parts: Vec<&str> = input.trim().split(" ").collect();
        match parts[0].to_uppercase().as_str() {
            "PING" => Some(ServerCommand::Ping),
            "HASH" if parts.len() > 1 => Some(ServerCommand::Hash(parts[1].to_string())),
            _ => None,
        }
    }
}

pub fn server() {
    let addr = format!("0.0.0.0:{SERVER_PORT}");
    let listener = TcpListener::bind(&addr);
    assert!(listener.is_ok(), "can't bind to port {}", SERVER_PORT);
    let listener = listener.unwrap();
    println!("listening on {addr}");
    for stream in listener.incoming() {
        match stream {
            Ok(req) => handle_req(req),
            Err(err) => eprintln!("error while receiving req: {}", err),
        }
    }
}

fn handle_req(mut socket: TcpStream) {
    let mut buffer = vec![0; 256];
    let read = socket.read(&mut buffer).expect("couldn't read from buffer");

    let request = String::from_utf8_lossy(&buffer[..read]);
    let command = ServerCommand::new(request.as_ref());
    println!("request: {}", request);
    match command {
        Some(ServerCommand::Ping) => {
            socket.write(b"PONG").unwrap();
        }
        Some(ServerCommand::Hash(plain)) => {
            let h = hash(&plain);
            socket.write(format!("{h}").as_bytes()).unwrap();
        }
        None => {
            socket
                .write(format!("unknown command: {}\n", request).as_bytes())
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
