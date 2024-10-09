// use std::{fs::File, io::{Read, Write}};

// use ggez::Context;

// use crate::engine::enums::State;

// use super::{enums::{LoadGameError, Piece}, game::Game};



// pub fn load_game(path: String, gtx: &mut Context, window_scale: f32, play_against_computer: bool, computer_color: String) -> Result<Game, LoadGameError> {
//     let mut string: String = String::new();
//     {
//         let file = File::open(path);
//         if file.is_err() {
//             return Err(LoadGameError::new("Cannot load file, because file cannot be found".to_string()));
//         }
//         if File::read_to_string(&mut file.unwrap(), &mut string).is_err() {
//             return Err(LoadGameError::new("Cannot load file, because file cannot be read".to_string()));
//         }
//     }

//     let mut game: Game = Game::new(gtx, window_scale, play_against_computer, computer_color);
//     let elements: Vec<&str> = string
//         .split("\n")
//         .map(|s| s.split(" ").collect::<Vec<&str>>())
//         .map(|split| split[1])
//         .collect::<Vec<&str>>();
    
//     let board = match str_to_board(elements[0]) {
//         Ok(val) => val,
//         Err(err) => return Err(err)
//     };
//     game.set_board(board);
    
//     if game.get_player_turn().to_str() != elements[1] {
//         game.next_player_turn();
//     }

//     game.update_state(Option::Some(State::parse(elements[2])));

//     let setup_pieces_left: Result<u8, std::num::ParseIntError> = elements[3].parse();
//     if setup_pieces_left.is_err() { return Err(LoadGameError::new("The setup pieces cannot be parsed from the loading file".to_string())); }
//     game.set_setup_pieces_left(setup_pieces_left.unwrap());
//     game.refresh();
//     println!("Loaded from file successfully");
//     Ok(game)
// }

// pub fn save_game(game: &mut Game) {
//     let content: String = format!(
//         "board: {}\nplayer_turn: {}\nstate: {}\nsetup_pieces_left: {}",
//         board_to_string(game.get_board()),
//         game.get_player_turn().to_str(),
//         game.get_state().to_str(),
//         game.get_setup_pieces_left());
    
//     let mut f = match File::create(std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/outputs/snapshots/game.txt") {
//         Ok(val) => val,
//         Err(_) => return println!("The save file failed to be created")
//     };
//     if f.write_all(content.as_bytes()).is_err() {
//         return println!("The save file cannot be changed");
//     }
// }

// fn board_to_string(board: [[Piece; 3]; 8]) -> String {
//     let mut string: String = "".to_string();
//     for (_, rings) in board.iter().enumerate() {
//         for (_, element) in rings.iter().enumerate() {
//             match element {
//                 Piece::White => string.push('W'),
//                 Piece::Black => string.push('B'),
//                 Piece::None => string.push('E')
//             }
//         }
//     }
//     return string;
// }

// fn str_to_board(string: &str) -> Result<[[Piece; 3]; 8], LoadGameError> {
//     let mut board: [[Piece; 3]; 8] = [[Piece::None; 3]; 8];
//     let mut chars = string.chars();
//     for x in 0..8 {
//         for ring in 0..=2 {
//             match chars.next() {
//                 Some('W') => board[x][ring] = Piece::White,
//                 Some('B') => board[x][ring] = Piece::Black,
//                 Some('E') => board[x][ring] = Piece::None,
//                 Some(c) => return Err(LoadGameError::new(format!("The letter {} cannot be parsed", c))),
//                 _ => return Err(LoadGameError::new("There are missing coding letters to parse the full board from the loading file".to_string()))
//             }
//         }
//     }
//     return Ok(board);
// }

// /* In case carry piece wants to be saved aswell:
// fn carry_piece_to_string(carry_piece: Option<CarryPiece>) -> String {
//     if carry_piece.is_none() {
//         return "None".to_string();
//     }
//     return format!("{}, {}, {}", carry_piece.clone().unwrap().x, carry_piece.clone().unwrap().ring, carry_piece.unwrap().color.to_str());
// }

// fn string_to_carry_piece(string: &str) -> Result<Option<(usize, usize, Piece)>, LoadGameError> {
//     if string == "None" {
//         return Ok(Option::None);
//     } else {
//         let splitted: Vec<&str> = string.split(", ").collect();
//         if splitted.len() != 3 {
//             return Err(LoadGameError::new("The Carry Piece cannot be parsed from the loading file".to_string()));
//         }
//         let x = splitted[0].parse();
//         let ring = splitted[1].parse();
//         if x.is_err() || ring.is_err() {
//             return Err(LoadGameError::new("The Carry Piece x cannot be parsed from the loading file".to_string()));
//         }
//         return Ok(Option::Some((x.unwrap(), ring.unwrap(), Piece::parse(splitted[2].to_string()))));
//     }
// }
// */