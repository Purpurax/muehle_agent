use std::time::{Duration, Instant};
use itertools::Itertools;

use crate::ai::action::forward_step_boards;
use crate::ai::position::negate_token;
use crate::ai::utils::{extract_black_move_count_from_board, extract_black_token_count_from_board, extract_white_move_count_from_board, extract_white_token_count_from_board};
use crate::ai::{Phase, PhaseEnum};

pub fn minimax(board: u64, depth: usize, mut alpha: isize, mut beta: isize, maximizing_player: u8, phase: Phase, time: Instant) -> Option<isize> {
    if time.elapsed() > Duration::from_millis(980) {
        return None;
    }
    
    if depth == 0 {
        return Some(evaluate_action(board, phase));
    }
    
    let black_token_count = extract_black_token_count_from_board(board);
    let white_token_count = extract_white_token_count_from_board(board);
    if phase.phase == PhaseEnum::Move {
        if (extract_black_move_count_from_board(board) == 0 && black_token_count > 3) || black_token_count == 2 {
        return Some(isize::MAX - phase.step_counter as isize)
        } else if (extract_white_move_count_from_board(board) == 0 && white_token_count > 3) || white_token_count == 2 {
            return Some(isize::MIN + phase.step_counter as isize)
        }
    }

    let forward_step_boards = forward_step_boards(&board, maximizing_player, phase)
        .sorted_by(|board1, board2| {
            let board1_eval = evaluate_action(*board1, phase);
            let board2_eval = evaluate_action(*board2, phase);
            if maximizing_player == 0b11 {
                board2_eval.cmp(&board1_eval)
            } else {
                board1_eval.cmp(&board2_eval)
            }
    });
    
    if maximizing_player == 0b11 {
        let mut max_eval = isize::MIN;

        for forward_board in forward_step_boards {
            let eval = minimax(forward_board, depth - 1, alpha, beta, negate_token(maximizing_player), phase.increased(), time);
            if eval.is_none() {
                return None;
            }
            max_eval = std::cmp::max(max_eval, eval.unwrap());
            
            alpha = std::cmp::max(alpha, eval.unwrap());
            if beta <= alpha {
                break;
            }
        }
        return Some(max_eval)
    } else {
        let mut min_eval = isize::MAX;
        for forward_board in forward_step_boards {
            let eval = minimax(forward_board, depth - 1, alpha, beta, negate_token(maximizing_player), phase.increased(), time);
            if eval.is_none() {
                return None;
            }
            min_eval = std::cmp::min(min_eval, eval.unwrap());
            
            beta = std::cmp::min(beta, eval.unwrap());
            if beta <= alpha {
                break;
            }
        }
        return Some(min_eval)
    }
}

fn evaluate_action(positions: u64, phase: Phase) -> isize {
    let mut score: isize = 0;
    let black_move_count = extract_black_move_count_from_board(positions);
    let white_move_count = extract_white_move_count_from_board(positions);
    let black_token_count = extract_black_token_count_from_board(positions);
    let white_token_count = extract_white_token_count_from_board(positions);

    if phase.phase == PhaseEnum::Move {
        if (black_move_count == 0 && black_token_count > 3) || black_token_count == 2 {
            return isize::MAX - phase.step_counter as isize
        } else if (white_move_count == 0 && white_token_count > 3) || white_token_count == 2 {
            return isize::MIN + phase.step_counter as isize
        }
    }
    
    score += (white_token_count as isize - black_token_count as isize) * 1000;
    score += white_move_count as isize - black_move_count as isize;
    
    return score
}