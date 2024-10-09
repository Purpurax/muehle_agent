pub mod ui;
pub mod ai;
pub mod core;

use ui::Engine;

fn main() {
    // TODO instead of setting
    // Windows size is 1280px on scale = 1
    let window_scale: f32 = 0.7;
    let computer_white: bool = true;
    let computer_black: bool = false;
    let asset_folder: &str = "assets";

    let engine: Engine = Engine::new(window_scale, computer_white, computer_black, asset_folder);
    engine.run();
}
