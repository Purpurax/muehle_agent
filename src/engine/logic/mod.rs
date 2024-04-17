use crate::engine;

pub fn check(game: engine::Game) {
    print!("outside engine: {}", game.board[0][0]);
}