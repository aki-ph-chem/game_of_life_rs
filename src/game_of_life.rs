use game_of_life_rs;

fn main() {
    let mut game = game_of_life_rs::render::MainRender::new();

    game.init();
    game.draw();
}
