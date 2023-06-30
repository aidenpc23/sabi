use std::net::TcpListener;
use std::io::{Read, Error};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: sabi listen -p <port>");
        return Ok(());
    }

    let listener = TcpListener::bind(format!("[::]:{}", args[1]))?;
    println!("Listening on: {}", args[1]);

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                let mut buf = vec![0; 1024];
                match socket.read(&mut buf) {
                    Ok(_n) => {
                        let message = String::from_utf8_lossy(&buf);
                        println!("Message received: {}", message.trim_matches(char::from(0)));
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Err(e) => { return Err(e.into()); }
        }
    }

    Ok(())
}
