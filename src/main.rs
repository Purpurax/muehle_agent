pub mod ui;
pub mod ai;
pub mod core;

use good_web_game as ggez;
use ggez::GameResult;
use ggez::conf::Conf;
use ui::Engine;

#[allow(dead_code)]
fn main() -> GameResult {
    let conf = Conf::default()
        .cache(Some(include_bytes!("../assets.tar")))
        .window_resizable(true)
        .window_title("Mühle KI | Purpurax".to_string());

    ggez::start(conf, move |context, quad_ctx|
        Box::new(Engine::new(context, quad_ctx).unwrap()))
}