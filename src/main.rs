pub mod engine;
pub mod ai;

fn main() {
    
    // TODO: Create an argument parser, which can take additional Parameters

    // Windows size is 1280px on scale = 1
    let window_scale:f32 = 0.7;
    let play_against_computer = false;
    let computer_color: &str = "Black";
    // Leave empty to start new game
    let load_game_abs_path: &str = "../outputs/snapshots/game.txt";
    // let load_game_abs_path: &str = "";




    if play_against_computer {
        ai::activate_ai(load_game_abs_path.to_string(), computer_color.to_string().clone());
    }

    if load_game_abs_path == "" {
        engine::run(window_scale, play_against_computer, computer_color.to_string());
    } else {
        engine::load(load_game_abs_path.to_string(), window_scale, play_against_computer, computer_color.to_string());
    }
}
