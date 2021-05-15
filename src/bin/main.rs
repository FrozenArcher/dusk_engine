use dusk_engine::game::{GameWindowBuilder, Game};

fn main() {
    let mut game = Game::new(GameWindowBuilder::new()
        .size(600, 400)
        .title("Some game"));
    game.run();
}
