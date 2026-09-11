use bear_island::game::Game;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(720, 480)
        .fullscreen()
        .title("Jelly Bear Island")
        .build();

    rl.set_target_fps(60);

    let mut game = Game::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        game.update(&mut d);
        game.draw(&mut d);
    }
}
