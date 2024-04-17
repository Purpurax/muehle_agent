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
    player_turn: u8,
}

impl Game {
    pub fn set_board(&mut self, new_board: [[u8; 3]; 8]) -> bool {
        
        self.board = new_board;
        
        // TODO: check for valid new board (correct move, board size unchanged, correct piece amount)
        
        return true
    }

    pub fn get_board(&self) -> [[u8; 3]; 8] {
        self.board
    }
}

pub fn run() {
    println!("Initialising...");
    

    // TODO: Implement the ability to place pieces
    // Temporary solution:
    let game: Game = set_example_board();

    print!("The game inside engine: {}", game.board[0][0]);

    logic::check(game);
}

fn set_example_board() -> Game {
    let board: [[u8; 3]; 8]  = [[1, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let player_turn: u8 = 0;
    Game{ board, player_turn }
}