use core::panic;
use std::env;
use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream, SocketAddrV6, Ipv6Addr};
use getopts::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "port", "set port", "PORT");
    opts.optopt("h", "host", "set host", "HOST");
    opts.optopt("m", "message", "set message", "MESSAGE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("Does not match!") }
    };

    if args.len() < 2 {
        eprintln!("Usage: sabi send -h <host> -p <port> -m <message> | listen -p <port>");
        return Ok(());
    }

    match args[1].as_str() {
        "listen" => {
            let port = matches.opt_str("p").unwrap_or(String::from("6942"));

            let listener = TcpListener::bind(format!("[::]:{}", port))?;
            println!("Listening on: {}", port);

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
        }
        "send" => {
            if !matches.opt_present("h") || !matches.opt_present("m") {
                eprintln!("Usage: sabi send -h <host> -p <port> -m <message>");
                return Ok(());
            }

            let host = matches.opt_str("h").unwrap();
            let port = matches.opt_str("p").unwrap_or(String::from("6942"));
            let message = matches.opt_str("m").unwrap();

            let sock: SocketAddrV6 = SocketAddrV6::new(host.parse()?, port.parse::<u16>().unwrap(), 0, 3);
            let mut strm = TcpStream::connect(sock)?;

            strm.write_all(&message.as_bytes())?;
            println!("Message sent");
        }
        _ => {
            eprintln!("Invalid command: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}
