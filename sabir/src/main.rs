use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: sender <host:port> <message>");
        return Ok(());
    }

    let mut stream = TcpStream::connect(&args[1]).await?;
    stream.write_all(args[2].as_bytes()).await?;
    println!("Message sent");

    Ok(())
}
