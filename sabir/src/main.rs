use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: receiver <port>");
        return Ok(());
    }

    let listener = TcpListener::bind(format!("0.0.0.0:{}", args[1])).await?;
    println!("Listening on: {}", args[1]);

    loop {
        let (mut socket, _) = listener.accept().await?;

        let mut buf = vec![0; 1024];
        match socket.read(&mut buf).await {
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
}
