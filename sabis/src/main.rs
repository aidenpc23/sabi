use std::env;
use std::error::Error;
use std::io::Write;
use std::net::{SocketAddrV6, TcpStream, Ipv6Addr};

fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: sabi send -h <host> -p <port> -m <message> -f <file>");
        return Ok(());
    }
    
    let sock: SocketAddrV6 = SocketAddrV6::new(args[1].parse()?, args[2].parse::<u16>().unwrap(), 0, 3);
    let mut strm = TcpStream::connect(sock)?;

    strm.write_all(&args[3].as_bytes())?;
    println!("Message sent");

    Ok(())
}
