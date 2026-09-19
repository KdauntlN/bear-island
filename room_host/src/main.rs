use std::{io::{self}, net::TcpListener, process};

fn main() -> io::Result<()> {
    let listener = match bind_listener() {
        Some(listener) => listener,
        None => {
            print!("\n");
            process::exit(1);
        },
    };

    print!("{}\n", listener.local_addr()?.port());

    Ok(())
}

fn bind_listener() -> Option<TcpListener> {
    for port in 27016..27115 {
        if let Ok(listener) = TcpListener::bind(("0.0.0.0", port)) {
            return Some(listener);
        } else {
            continue;
        }
    }

    None
}