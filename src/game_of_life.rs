use game_of_life_rs;

fn main() {
    let mut game = game_of_life_rs::render::MainRender::new();
    if cfg!(debug_assertions) {
        eprintln!("MainRender::new(): OK");
    }

    game.init();
    if cfg!(debug_assertions) {
        eprintln!("MainRender.init(): OK");
    }

    game.draw();
}
