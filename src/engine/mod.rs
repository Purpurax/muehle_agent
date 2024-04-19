pub mod logic;
pub mod rendering;

#[derive(Clone, Copy)]
enum Piece {
    None,
    White,
    Black
}


struct Game {
    /* [[left-top-outer, left-top-middle, left-top-inner], [middle-top-outer, middle-top-middle, middle-top-inner] ...]
        0 - empty
        1 - white piece
        2 - black piece
    */
    board: [[Piece; 3]; 8],
    player_turn: u8,
    
    // Amount of stones left to be placed by both players
    setup_phase: u8,
}

impl Default for Game {
    fn default() -> Game {
        Game {
            board: [[Piece::None; 3]; 8],
            player_turn: 1,
            setup_phase: 18,
        }
    }
}

impl Game {
    fn set_example_board(&mut self){
        let example_board: [[Piece; 3]; 8] = [[Piece::None, Piece::White, Piece::Black]; 8];

        self.board = example_board;
        self.player_turn = 1;
        self.setup_phase = 0;
    }
}

pub fn run() {
    println!("Initialising...");
    
    let mut game: Game = Game::default();

    // TODO complete UI rendering
    // rendering::render(game);

    // TODO: Implement the ability to place pieces
    // Temporary solution:
    game.set_example_board();

    // TODO: Create a game loop with rendering after and inbetween each step
        // TODO: Take Input through UI (inside the rendering folder)
        // Temporary solution:
        let desired_move = get_user_input();

        

}

fn get_user_input() {
    todo!()
}