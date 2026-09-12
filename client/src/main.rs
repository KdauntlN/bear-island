use std::net::TcpStream;
use raylib::prelude::*;

fn main() {
    let tcp_stream = TcpStream::connect("127.0.0.1:7878");
    
    let (mut rl, thread) = raylib::init()
        .title("Bear Island")
        .resizable()
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        if let Err(ref e) = tcp_stream {
            d.draw_text(&format!("Error connecting to server: {e}"), 10, 10, 15, Color::BLACK);
        }
    }
}