use std::{fs::File, io::Read, str::Chars};
use itertools::Itertools;

use crate::engine::{enums::Piece, logic};


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    ring: usize
}
impl Position {
    pub fn new(x: usize, ring: usize) -> Position {
        Position {
            x,
            ring
        }
    }
}

static mut GAME_FILE_PATH: String = String::new();
static mut COMPUTER_COLOR: u8 = 0;

pub fn activate_ai(path: String, computer_color: String) {
    unsafe {
        GAME_FILE_PATH = path;
        COMPUTER_COLOR = if computer_color == "White" {
            1
        } else {
            2
        };
    };
}


pub fn compute_step() -> (usize, usize, usize, usize){
    let (board, pieces, _pieces_enemy): ([[u8; 3]; 8], Vec<Position>, Vec<Position>) = get_current_board(unsafe { GAME_FILE_PATH.clone() }, unsafe { COMPUTER_COLOR.clone() });
    
    let mut binding = pieces.clone().into_iter();
    let mut possible_moves = possible_moves(board, &mut binding);

    let random_move = possible_moves.next().unwrap();

    return (random_move.0.x, random_move.0.ring, random_move.1.x, random_move.1.ring);
}


fn possible_moves(board: [[u8; 3]; 8], pieces: &mut dyn Iterator<Item=Position>) -> impl Iterator<Item=(Position, Position)> + '_ {
    let (piece_count, _): (usize, _) = pieces.size_hint();
    pieces.flat_map(move |pos| possible_move_for_one_piece(board, pos, piece_count))
}

fn possible_move_for_one_piece(board: [[u8; 3]; 8], pos: Position, piece_count: usize) -> impl Iterator<Item=(Position, Position)> {
    let (x, ring) = (pos.x, pos.ring);
    let mut result: [Position; 24] = [Position::new(10, 10); 24];
    if piece_count == 3 {
        for iter_x in 0..8 {
            for iter_ring in 0..3 {
                if board[iter_x][iter_ring] == 0{
                    result[(iter_x*3) + iter_ring] = Position::new(iter_x, iter_ring)
                }
            }
        }
    } else {
        if x%2 == 1 {
            if ring != 1 {
                if board[x][1] == 0 {
                    result[2] = Position::new(x, 1);
                }
            } else {
                if board[x][0] == 0 {
                    result[2] = Position::new(x, 0);
                }
                if board[x][2] == 0 {
                    result[3] = Position::new(x, 2);
                }
            }
        }

        let next_x = (x+1)%8;
        let prev_x = (x+7)%8;

        if board[next_x][ring] == 0 {
            result[0] = Position::new(next_x, ring);
        }
        if board[prev_x.rem_euclid(8)][ring] == 0 {
            result[1] = Position::new(prev_x, ring);
        }

    };
    return result.into_iter().filter(|x| *x != Position::new(10, 10)).map(move |x| (pos, x));
}

fn get_current_board(path: String, computer_color: u8) -> ([[u8; 3]; 8], Vec<Position>, Vec<Position>) {
    let mut string: String = String::new();
    let mut file = File::open(path).expect("AI couldn't open the game files");
    file.read_to_string(&mut string).expect("AI couldn't read the board through the game files");

    let binding = string.replace("board: ", "");
    let board_chars = binding.chars().take(24);
    
    get_current_board_from_chars(board_chars, computer_color)
}

fn get_current_board_from_chars(board_string: std::iter::Take<Chars<'_>>, computer_color: u8) -> ([[u8; 3]; 8], Vec<Position>, Vec<Position>) {
    let binding = board_string.clone();
    let mut board_iter = binding.map(|c| {
        match c {
            'W' => 1,
            'B' => 2,
            'E' => 0,
            _ => 3
        }
    });
    let mut board: [[u8; 3]; 8] = [[0; 3]; 8];
    for x in 0..8 {
        for y in 0..3 {
            board[x][y] = board_iter.next().unwrap() as u8;
        }
    }    

    let pieces_position = board_string.enumerate().map(|(i, c)| (Position::new(i/3, i%3), c));
    let computer_color_key = if computer_color == 2 { 'B' } else { 'W' };
    let not_computer_color_key = if computer_color == 2 { 'W' } else { 'B' };

    let pieces = pieces_position
        .clone()
        .filter_map(|(pos, c)| if c == computer_color_key { Some(pos) } else { None })
        .collect::<Vec<Position>>();
    let pieces_enemy = pieces_position
        .clone()
        .filter_map(|(pos, c)| if c == not_computer_color_key { Some(pos) } else { None })
        .collect::<Vec<Position>>();
    (board, pieces, pieces_enemy)
}








#[test]
fn test_evaluate_move() {
    let mut board_configurations: String = String::new();
    let path = "C://1//PROJECTS//Rust//muehle_agent//outputs//tests//input_felder.txt";
    let mut file = File::open(path).expect("Test cannot find file");
    file.read_to_string(&mut board_configurations).expect("Test cannot read file");

    let boards: Vec<String> = board_configurations.split("\n").map(|s| s.to_string()).collect();

    let mut expected_tuple_output: String = String::new();
    let path = "C://1//PROJECTS//Rust//muehle_agent//outputs//tests//output.txt";
    let mut file = File::open(path).expect("Test cannot find file");
    file.read_to_string(&mut expected_tuple_output).expect("Test cannot read file");

    let expected_tuples: Vec<String> = expected_tuple_output.split("\n").map(|s| s.to_string()).collect();

    for (i, board) in boards.iter().enumerate() {
        assert_eq!(expected_tuples[i], evaluate_move(convert_to_my_datastructure_board(board)));
    }

}

fn convert_to_my_datastructure_board(board: &str) -> String {
    let chars = board.chars();
    let mut my_board: [char; 24] = ['_'; 24];
    let mut counter = 0;

    for char in chars {
        let index: usize = match counter {
            0 => 3,
            1 => 6,
            2 => 9,
            3 => 12,
            4 => 15,
            5 => 18,
            6 => 21,
            7 => 0,
            8 => 4,
            9 => 7,
            10 => 10,
            11 => 13,
            12 => 16,
            13 => 19,
            14 => 22,
            15 => 1,
            16 => 5,
            17 => 8,
            18 => 11,
            19 => 14,
            20 => 17,
            21 => 20,
            22 => 23,
            23 => 2,
            _ => 0
        };
        my_board[index] = char;
        counter += 1;
    }

    my_board.iter().collect()
}

fn evaluate_move(string: String) -> String {
    let (board, pieces_white, pieces_black) = get_current_board_from_chars(string.chars().take(24), 1);

    let mut binding = pieces_white.clone().into_iter();
    let possible_moves = possible_moves(board, &mut binding);
    let mut total_possible_moves: usize = 0;

    let mut total_possible_mills: u8 = 0;
    for possible_move in possible_moves {
        total_possible_moves += 1;
        if logic::is_creating_mill(
            Piece::White,
            (possible_move.1.x, possible_move.1.ring),
            remove_field_from_board(possible_move.0, get_piece_board_from_u8_board(board))
            )
        {
            total_possible_mills += 1;
        }
    }

    let total_possible_takes: usize = if total_possible_mills != 0 {
        if logic::all_pieces_are_in_mills(Piece::Black, get_piece_board_from_u8_board(board)) {
            pieces_black.len()
        } else {
            let mut counter = 0;
            for piece in pieces_black {
                if !logic::is_piece_part_of_mill(Piece::Black, (piece.x, piece.ring), get_piece_board_from_u8_board(board)) {
                    counter += 1
                }
            }
            counter
        }
    } else {
        0
    };


    let mut result = String::new();
    result.push_str(total_possible_moves.to_string().as_str());
    result.push_str(" ");
    result.push_str(total_possible_mills.to_string().as_str());
    result.push_str(" ");
    result.push_str(total_possible_takes.to_string().as_str());
    return result
}

fn remove_field_from_board(pos: Position, board: [[Piece; 3]; 8]) -> [[Piece; 3]; 8] {
    let mut new_board = board;
    new_board[pos.x][pos.ring] = Piece::None;
    return new_board
}

fn get_piece_board_from_u8_board(board: [[u8; 3]; 8]) -> [[Piece; 3]; 8] {
    let mut result: [[Piece; 3]; 8] = [[Piece::None; 3]; 8];
    for (x, rings) in board.iter().enumerate() {
        for (ring, ele) in rings.iter().enumerate() {
            result[x][ring] = match ele {
                1 => Piece::White,
                2 => Piece::Black,
                _ => Piece::None
            }
        }
    }

    result
}