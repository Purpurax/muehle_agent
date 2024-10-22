use action::Action;
use rayon::prelude::*;
use good_web_game::timer;

use crate::core::game::Game;
use crate::core::enums::{State, Difficulty};
use crate::core::utils::{get_action_from_board, insert_number_of_possible_moves_to_board, insert_token_count_to_board};
use crate::ai::action::forward_step_boards;
use crate::ai::minimax::minimax;
use crate::core::position::negate_token;

pub mod action;
mod minimax;

pub fn compute_step(game: &Game, difficulty: Difficulty) -> Option<Action> {
    let setup_pieces_left: u8 = game.get_setup_pieces_left();
    
    let phase = match game.get_state() {
        State::Setup => Phase::new(PhaseType::Set, 18 - setup_pieces_left),
        _ => Phase::new(PhaseType::Move, 20)
    };
    
    let token_type = game.get_player_turn();

    let mut board = game.get_board();
    board = insert_token_count_to_board(board);
    board = insert_number_of_possible_moves_to_board(board);


    let now = timer::time();
    
    let mut depth = 0;
    let maximum_depth = match difficulty {
        Difficulty::Off => 1,
        Difficulty::Easy => 1,
        Difficulty::Medium => 6,
        Difficulty::Hard => 50
    };
    
    let mut best_action_total = None;
    let mut best_score_total = if token_type == 0b11 { isize::MIN } else { isize::MAX };
    let mut last_depth_time_elapsed: f64 = 0.0;
    let mut _actions_with_scores: Vec<(u64, Option<isize>)> = Vec::with_capacity(500);

    'outer_loop: loop {
        if depth == maximum_depth {
            break;
        }

        let mut best_action = None;
        let mut best_score = if token_type == 0b11 { isize::MIN } else { isize::MAX };
        _actions_with_scores = forward_step_boards(&board, token_type, phase).par_bridge().map(|forward_board| {
            (forward_board, minimax(forward_board, depth, isize::MIN, isize::MAX, negate_token(token_type), phase.increased(), now))
        }).collect();

        for action_with_score in _actions_with_scores.into_iter() {
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
        last_depth_time_elapsed = timer::time() - now;
        depth += 1;
    }

    println!("-> Execution time {:.3?} \n-> best score {} \n-> depth: {}\n", last_depth_time_elapsed, best_score_total, depth);
    best_action_total
}

#[derive(Clone, Copy, PartialEq)]
pub enum PhaseType {
    Set,
    Move
}
#[derive(Clone, Copy)]
pub struct Phase {
    pub phase: PhaseType,
    pub step_counter: u8
}
impl Phase {
    pub fn new(phase: PhaseType, step_counter: u8) -> Self {
        Phase {
            phase,
            step_counter
        }
    }
    pub fn increased(&self) -> Phase {
        Phase::new(
            if self.phase == PhaseType::Set && self.step_counter + 1 >= 18 {
                PhaseType::Move
            } else {
                self.phase
            },
            self.step_counter + 1
        )
    }
}