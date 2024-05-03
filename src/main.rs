pub mod engine;
pub mod ai;

fn main() {
    
    // TODO: Create an argument parser, which can take additional Parameters

    // Windows size is 1280px on scale = 1
    let window_scale:f32 = 0.7;
    let play_against_computer = false;
    // Leave empty to start new game
    let load_game_abs_path = "C://1//PROJECTS//Rust//muehle_agent//outputs//snapshots//game.txt";




    if play_against_computer {
        ai::activate_ai();
    }

    if load_game_abs_path == "" {
        engine::run(window_scale, play_against_computer);
    } else {
        engine::load(load_game_abs_path.to_string(), window_scale, play_against_computer);
    }
}
