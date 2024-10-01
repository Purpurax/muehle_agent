use action::Action;
use position::negate_token;
use utils::{get_action_from_board, insert_number_of_possible_moves_to_board, insert_token_count_to_board};
use std::time::{Duration, Instant};
use rayon::prelude::*;

use crate::engine::game::Game;
use crate::engine::enums::{State, Piece};
use crate::ai::position::decode_positions;
use crate::ai::action::forward_step_boards;
use crate::ai::minimax::minimax;

mod action;
mod minimax;
mod position;
mod utils;

pub fn compute_step(game: &mut Game) -> (usize, usize, usize, usize) {
    let phase = match game.get_state() {
        State::Setup => Phase::new(PhaseEnum::Set, game.get_setup_pieces_left()),
        _ => Phase::new(PhaseEnum::Move, 0)
    };
    
    let token_type = match game.get_player_turn() {
        Piece::Black => 0b10,
        Piece::White => 0b11,
        _ => panic!("Unknown color")
    };

    let mut board = convert_board_to_binary_board(game.get_board());
    board = insert_token_count_to_board(board);
    board = insert_number_of_possible_moves_to_board(board);


    let now = Instant::now();
    
    let mut depth = 0;
    
    let mut best_action_total = None;
    let mut best_score_total = if token_type == 0b11 { isize::MIN } else { isize::MAX };
    let mut last_depth_time_elapsed = Duration::from_secs(0);
    let mut actions_with_scores: Vec<(u64, Option<isize>)> = Vec::with_capacity(500);

    'outer_loop: loop {
        depth += 1;
        let mut best_action = None;
        let mut best_score = if token_type == 0b11 { isize::MIN } else { isize::MAX };
        actions_with_scores = forward_step_boards(&board, token_type, phase).par_bridge().map(|forward_board| {
            (forward_board, minimax(forward_board, depth, isize::MIN, isize::MAX, negate_token(token_type), phase.increased(), now))
        }).collect();

        for action_with_score in actions_with_scores.into_iter() {
            if action_with_score.1.is_none() {
                break 'outer_loop;
            }

            if token_type == 0b11 && action_with_score.1.unwrap() >= best_score {
                best_action = Some(get_action_from_board(board, action_with_score.0, token_type));
                best_score = action_with_score.1.unwrap();
            } else if token_type == 0b10 && action_with_score.1.unwrap() <= best_score {
                best_action = Some(get_action_from_board(board, action_with_score.0, token_type));
                best_score = action_with_score.1.unwrap();
            }
        }

        best_action_total = best_action;
        best_score_total = best_score;
        last_depth_time_elapsed = now.elapsed();
    }

    let execution_information = format!("-> Execution time {:.3?} \n-> best score {} \n-> depth: {}\n-> step: {}\n", last_depth_time_elapsed, best_score_total, depth, 0);
    println!("{}", execution_information);
    return (0, 0, 0, 0)
    // return get_move_from_board(best_action_total.unwrap());
}

fn convert_board_to_binary_board(board: [[Piece; 3]; 8]) -> u64 {
    let mut res: u64 = 0b0;
    for i in 0..3 {
        for j in 0..8 {
            res |= match board[(j+1) % 8][i] {
                Piece::None => 0b0,
                Piece::Black => 0b10,
                Piece::White => 0b11
            };
            res <<= 2;
        }
    }
    res >>= 2;
    return res;
}

fn get_move_from_board(action: Action) -> (usize, usize, usize, usize, usize, usize) {
    let start_pos: Option<usize> = action.start_position;
    let end_pos: usize = action.end_position;
    let beat_pos: Option<usize> = action.beatable_position;

    return (0,0,0,0,0,0)
}


// use position::negate_token;
// use utils::{get_action_from_board, insert_number_of_possible_moves_to_board, insert_token_count_to_board};


// pub mod action;
// pub mod minimax;
// pub mod position;
// pub mod utils;

// use crate::ai::position::decode_positions;
// use crate::ai::action::forward_step_boards;
// use std::time::{Duration, Instant};
// use crate::ai::minimax::minimax;
// use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PhaseEnum {
    Set,
    Move
}
#[derive(Clone, Copy)]
pub struct Phase {
    pub phase: PhaseEnum,
    pub step_counter: u8
}
impl Phase {
    pub fn new(phase: PhaseEnum, step_counter: u8) -> Self {
        Phase {
            phase,
            step_counter
        }
    }
    pub fn increased(&self) -> Phase {
        let mut new_phase = Phase::new(self.phase, self.step_counter + 1);
        if new_phase.phase == PhaseEnum::Set && new_phase.step_counter >= 18 {
            new_phase.phase = PhaseEnum::Move;
        }
        new_phase
    }
}

// fn read_input(step_counter: u8) -> (Phase, u8, u64) {
//     let mut input = String::new();

//     std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
//     let mut input = input.trim().split(" ");
    
//     let phase = match input.next().unwrap() {
//         "P" => Phase::new(PhaseEnum::Set, step_counter),
//         "M" => Phase::new(PhaseEnum::Move, step_counter),
//         other => panic!("Unknown phase \"{}\"", other)
//     };
    
//     let token_type = match input.next().unwrap() {
//         "B" => 0b10,
//         "W" => 0b11,
//         _ => panic!("Unknown color")
//     };
//     let mut board = decode_positions(input.next().unwrap().parse().unwrap());
//     board = insert_token_count_to_board(board);
//     board = insert_number_of_possible_moves_to_board(board);

//     (phase, token_type, board)
// }

// #[allow(unused_assignments)]
// fn main() {
//     let mut step_counter = 0;
//     loop {
//         let (phase, token_type, board) = read_input(step_counter);
//         if step_counter == 0 && token_type == 0b10 {
//             step_counter += 1;
//         }
//         let now = Instant::now();
        
//         let mut depth = 0;
        
//         let mut best_action_total = None;
//         let mut best_score_total = if token_type == 0b11 { isize::MIN } else { isize::MAX };
//         let mut last_depth_time_elapsed = Duration::from_secs(0);
//         let mut actions_with_scores: Vec<(u64, Option<isize>)> = Vec::with_capacity(500);

//         'outer_loop: loop {
//             depth += 1;
//             let mut best_action = None;
//             let mut best_score = if token_type == 0b11 { isize::MIN } else { isize::MAX };
//             actions_with_scores = forward_step_boards(&board, token_type, phase).par_bridge().map(|forward_board| {
//                 (forward_board, minimax(forward_board, depth, isize::MIN, isize::MAX, negate_token(token_type), phase.increased(), now))
//             }).collect();

//             for action_with_score in actions_with_scores.into_iter() {
//                 if action_with_score.1.is_none() {
//                     break 'outer_loop;
//                 }

//                 if token_type == 0b11 && action_with_score.1.unwrap() >= best_score {
//                     best_action = Some(get_action_from_board(board, action_with_score.0, token_type));
//                     best_score = action_with_score.1.unwrap();
//                 } else if token_type == 0b10 && action_with_score.1.unwrap() <= best_score {
//                     best_action = Some(get_action_from_board(board, action_with_score.0, token_type));
//                     best_score = action_with_score.1.unwrap();
//                 }
//             }

//             best_action_total = best_action;
//             best_score_total = best_score;
//             last_depth_time_elapsed = now.elapsed();
//         }

//         println!("{}", best_action_total.unwrap().to_string());

//         let execution_information = format!("-> Execution time {:.3?} \n-> best score {} \n-> depth: {}\n-> step: {}\n", last_depth_time_elapsed, best_score_total, depth, step_counter);
//         eprintln!("{}", execution_information);
//         step_counter += 2;
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_phase() {
//         let phase = Phase::new(PhaseEnum::Set, 0);
//         println!("p:{}, s:{}", if phase.phase == PhaseEnum::Set { " Set" } else { " Move" }, phase.step_counter);
//         println!("p:{}, s:{}", if phase.increased().phase == PhaseEnum::Set { " Set" } else { " Move" }, phase.increased().step_counter);
//     }
// }