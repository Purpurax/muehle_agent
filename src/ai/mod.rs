use action::Action;
use std::time::{Duration, Instant};
use rayon::prelude::*;

use crate::core::game::Game;
use crate::core::enums::State;
use crate::core::utils::{get_action_from_board, insert_number_of_possible_moves_to_board, insert_token_count_to_board};
use crate::ai::action::forward_step_boards;
use crate::ai::minimax::minimax;
use crate::core::position::negate_token;

pub mod action;
mod minimax;

pub fn compute_step(game: &Game) -> Option<Action> {
    let setup_pieces_left: u8 = game.get_setup_pieces_left();
    
    let phase = match game.get_state() {
        State::Setup => Phase::new(PhaseEnum::Set, 18 - setup_pieces_left),
        _ => Phase::new(PhaseEnum::Move, 20)
    };
    
    let token_type = game.get_player_turn();

    let mut board = game.get_board();
    board = insert_token_count_to_board(board);
    board = insert_number_of_possible_moves_to_board(board);


    let now = Instant::now();
    
    let mut depth = 0;
    
    let mut best_action_total = None;
    let mut best_score_total = if token_type == 0b11 { isize::MIN } else { isize::MAX };
    let mut last_depth_time_elapsed = Duration::from_secs(0);
    let mut _actions_with_scores: Vec<(u64, Option<isize>)> = Vec::with_capacity(500);

    'outer_loop: loop {
        if depth == 4 {
            break;
        }
        depth += 1;
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
        last_depth_time_elapsed = now.elapsed();
    }

    let execution_information = format!("-> Execution time {:.3?} \n-> best score {} \n-> depth: {}\n-> step: {}\n", last_depth_time_elapsed, best_score_total, depth, 0);
    println!("{}", execution_information);
    return best_action_total;
}

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