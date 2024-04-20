use std::io::Write;
use regex;

pub mod logic;
pub mod rendering;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
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
    win: bool
}

impl Default for Game {
    fn default() -> Game {
        Game {
            board: [[Piece::None; 3]; 8],
            player_turn: 1,
            setup_phase: 18,
            win: false,
        }
    }
}

impl Game {
    fn set_example_board(&mut self) {
        let example_board: [[Piece; 3]; 8] = [[Piece::None, Piece::White, Piece::Black]; 8];

        self.board = example_board;
        self.player_turn = 1;
        self.setup_phase = 0;
    }

    fn next_player_turn(&mut self) {
        if self.player_turn == 1 {
            self.player_turn = 2;
        } else if self.player_turn == 2 {
            self.player_turn = 1;
        } else {
            panic!("invalid player turn detected");
        }
    }

    fn set_board(&mut self, new_board: [[Piece; 3]; 8]) {
        self.board = new_board;
    }

    fn print_board(&self) {
        let sorted_board: [[Piece; 3]; 8] = self.get_sorted_board();
        let mut counter_x: u8 = 0;
        let amount_of_spaces_inbetween: [usize; 8] = [6,4,2,1,1,2,4,6];
        let amount_of_spaces_before_line: [usize; 8] = [0,2,4,0,0,4,2,0];

        for x in sorted_board.iter() {
            print!("{: <1$}", "", amount_of_spaces_before_line[counter_x as usize]);
            for ele in x.iter() {
                match ele {
                    Piece::None => print!("-"),
                    Piece::White => print!("w"),
                    Piece::Black => print!("b"),
                }
                print!("{: <1$}", "", amount_of_spaces_inbetween[counter_x as usize]);
            }
            if counter_x != 3 {
                println!();
            } else {
                print!("    ");
            }
            counter_x += 1;
        }
    }

    fn get_sorted_board(&self) -> [[Piece; 3]; 8] {
        let mut sorted_board = [[Piece::None; 3]; 8];
        let mut counter_x: u8 = 0;
        let mut counter_ring: u8 = 0;

        for x in self.board.iter() {
            counter_ring = 0;
            for ele in x.iter() {
                if counter_x <= 2 {
                    sorted_board[counter_ring as usize][counter_x as usize] = *ele;
                } else if counter_x == 3 {
                    sorted_board[4][(2-counter_ring) as usize] = *ele;
                } else if counter_x <= 6 {
                    sorted_board[(7-counter_ring) as usize][(6-counter_x) as usize] = *ele;
                } else {
                    sorted_board[3][counter_ring as usize] = *ele;
                }
                counter_ring += 1;
            }
            counter_x += 1;
        }

        return sorted_board;
    }

    fn set_win(&mut self, win: bool) {
        self.win = win;
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

    println!("Starting game loop...");
    while game.win == false {
        // TODO: Take Input through UI (inside the rendering folder)
        // Temporary solution:
        game.print_board();
        let (move_from, move_to): ((i8, i8), (i8, i8)) = get_user_input(game.player_turn);

        let (successful, new_board): (bool, [[Piece; 3]; 8]) = logic::move_piece(
                if game.player_turn == 1 {
                    Piece::White
                } else {
                    Piece::Black
                },
                move_from,
                move_to,
                game.board
            );
        if successful {
            println!("Player {} has moved a piece from ({}, {}) to ({}, {})", game.player_turn, move_from.0, move_from.1, move_to.0, move_to.1);
            game.next_player_turn();
            game.set_board(new_board);
        } else {
            println!("Invalid move, Player {} please try again", game.player_turn);
        }
    }
    
    println!("Player {} has won the game 🎊🎊", game.player_turn);
}

/*
 * The input format is as follows:
 * number, number, number, number
 * or number,number,number,number
 * or number number number number
*/
fn get_user_input(player_turn: u8) -> ((i8, i8), (i8, i8)) {
    print!("Player {}: Move piece \"from_x, from_ring, to_x, to_ring\" ", player_turn);
    let mut input = String::new();

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    let split_reg = regex::Regex::new(r", | |,").unwrap();
    let input_array: Vec<String> = split_reg.split(&input).map(|m| m.to_string()).collect();
    if input_array.len() < 4 {
        println!("Invalid input, try again");
        return ((0,0), (0,0));
    }

    let from_x: i8 = input_array[0].trim().parse().unwrap();
    let from_ring: i8 = input_array[1].trim().parse().unwrap();
    let to_x: i8 = input_array[2].trim().parse().unwrap();
    let to_ring: i8 = input_array[3].trim().parse().unwrap();

    return ((from_x, from_ring), (to_x, to_ring));
}