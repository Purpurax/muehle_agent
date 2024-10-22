use std::iter;

use crate::core::position::{create_token_iter, negate_token, set_token_at, BLACK_TOKEN_FIRST_POSITION, WHITE_TOKEN_FIRST_POSITION};
use crate::core::utils::{extract_black_token_count_from_board, extract_white_token_count_from_board, is_beat_possible, is_mill_closing, is_move_valid, update_possible_move_count};
use crate::ai::{Phase, PhaseType};

pub struct Action {
    pub start_position: Option<usize>,
    pub end_position: usize,
    pub beatable_position: Option<usize>,
}

impl Action {
    pub fn new(start_position: Option<usize>, end_position: usize, beatable_position: Option<usize>) -> Self {
        Action { start_position, end_position, beatable_position }
    }

    pub fn into(self) -> (Option<usize>, usize, Option<usize>) {
        (self.start_position, self.end_position, self.beatable_position)
    }
}

pub fn forward_step_boards(board: &u64, token_type: u8, phase: Phase) -> impl Iterator<Item=u64> + '_ {
    list_moves(board, token_type, phase)
        .flat_map(move |applyed_move_board| {
            if is_mill_closing(*board, applyed_move_board, token_type) {
                itertools::Either::Left(
                    create_token_iter(*board).enumerate()
                        .filter(move |(index, _)| is_beat_possible(*board, *index, token_type))
                        .map(move |(beatable_position, _)| {
                            let mut new_board = set_token_at(applyed_move_board, beatable_position, 0b00);
                            new_board -= if token_type == 0b11 {
                                BLACK_TOKEN_FIRST_POSITION
                            } else {
                                WHITE_TOKEN_FIRST_POSITION
                            };
                            update_possible_move_count(new_board, negate_token(token_type), beatable_position, true)
                        }))
            } else {
                itertools::Either::Right(iter::once(applyed_move_board))
            }
    })
}

pub fn list_moves(board: &u64, token_type: u8, phase: Phase) -> impl Iterator<Item=u64> + '_ {
    let token_extended: u64 = if token_type == 0b11 {
        0b111111111111111111111111111111111111111111111111
    } else {
        0b101010101010101010101010101010101010101010101010
    };

    if phase.phase == PhaseType::Set {
        let mut shifted: u64 = 0b11;

        itertools::Either::Left(
            (0..24).filter_map(move |index| {
                let result = if *board & shifted == 0 {
                    let mut new_board = *board | (shifted & token_extended);
                    if phase.step_counter >= 4 {
                        if token_type == 0b11 {
                            new_board += WHITE_TOKEN_FIRST_POSITION;
                        } else {
                            new_board += BLACK_TOKEN_FIRST_POSITION;
                        }
                    }
                    new_board = update_possible_move_count(new_board, token_type, 23 - index, false);
                    Some(new_board)
                } else {
                    None
                };
                shifted <<= 2;
                result
            }))
    } else {
        let number_of_token = if token_type == 0b11 {
            extract_white_token_count_from_board(*board)
        } else {
            extract_black_token_count_from_board(*board)
        };

        return itertools::Either::Right(
            create_token_iter(*board).enumerate()
                .filter(move |(_, token)| *token == token_type)
                    .flat_map(move |(start_position, _)| {
                        let mut new_board = set_token_at(*board, start_position, 0b00);
                        new_board = update_possible_move_count(new_board, token_type, start_position, true);

                        create_token_iter(*board)
                            .enumerate()
                            .filter_map(move |(end_position, end_token)| {
                                if is_move_valid(start_position, end_position, end_token, number_of_token as u8) {
                                    let new_board2 = update_possible_move_count(new_board, token_type, end_position, false);
                                    Some(set_token_at(new_board2, end_position, token_type))
                                } else {
                                    None
                                }
                            })
                    })
        )
    }
}