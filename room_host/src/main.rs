use std::{io, net::TcpListener};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    println!("{addr}\n");

    Ok(())
}