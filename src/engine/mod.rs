pub mod logic;
pub mod rendering;


pub struct Game {
    /* [[left-top-outer, left-top-middle, left-top-inner], [middle-top-outer, middle-top-middle, middle-top-inner] ...]
        0 - empty
        1 - white piece
        2 - black piece
    */
    board: [[u8; 3]; 8],
    
    // 0 - first player, 1 - second player
    player_turn: u8
}

impl Default for Game {
    fn default() -> Game {
        Game {
            board: [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]],
            player_turn: 0,
        }
    }
}

impl Game {
    fn set_example_board(&mut self){
        let example_board: [[u8; 3]; 8] = [[1, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];
        self.board = example_board;
    }
}

pub fn run() {
    println!("Initialising...");
    
    let mut game: Game = Game::default();

    rendering::render(game);

    // TODO: Implement the ability to place pieces
    // Temporary solution:
    game.set_example_board();


}