use core::panic;

use crate::ai::action::Action;
use crate::core::enums::{CarryPiece, FieldError, State};
use crate::core::game::Game;
use crate::core::position::negate_token;
use crate::core::utils::{is_beat_possible, is_mill_closing, is_move_valid, is_part_of_mill};

use super::enums::Difficulty;
use super::utils::possible_move_count_of_position;

// coords, positions
const POSSIBLE_POSITIONS_X: [(f32, [usize; 3]); 8] = [
    (1110.0, [1, 2, 3]),
    (950.0, [9, 10, 11]),
    (790.0, [17, 18, 19]),
    (640.0, [0, 4, 8]),
    (640.0, [12, 16, 20]),
    (490.0, [21, 22, 23]),
    (330.0, [13, 14, 15]),
    (170.0, [5, 6, 7])
];
const POSSIBLE_POSITIONS_Y: [(f32, [usize; 3]); 8] = [
    (1110.0, [3, 4, 5]),
    (950.0, [11, 12, 13]),
    (790.0, [19, 20, 21]),
    (640.0, [2, 6, 10]),
    (640.0, [14, 18, 22]),
    (490.0, [16, 17, 23]),
    (330.0, [8, 9, 15]),
    (170.0, [0, 1, 7])
];

const POSSIBLE_PANEL_INDICES_X: [(f32, usize); 6] = [
    (130.0, 0),
    (310.0, 1),
    (490.0, 2),
    (790.0, 3),
    (970.0, 4),
    (1150.0, 5)
];
const POSSIBLE_PANEL_INDICES_Y: f32 = 1480.0;

const RESTART_X: f32 = 640.0;
const RESTART_Y: f32 = 1350.0;

pub fn coords_to_board_position(x: f32, y: f32) -> Result<usize, FieldError> {
    const ACCURACY: f32 = 60.0;

    let mut possible_positions: Vec<usize> = vec![];
    for (coords, positions) in POSSIBLE_POSITIONS_X {
        if is_within_accuracy(x, coords, ACCURACY) {
            possible_positions.extend(&positions);
        }
    }
    
    for (coords, positions) in POSSIBLE_POSITIONS_Y {
        if is_within_accuracy(y, coords, ACCURACY) {
            for position in positions {
                if possible_positions.contains(&position) {
                    return Ok(position)
                }
            }
        }
    }

    Err(FieldError::new(format!("Invalid position: {}.x {}.y", x, y)))
}

pub fn coords_to_bottom_panel_position(x: f32, y: f32) -> Result<usize, FieldError> {
    const ACCURACY: f32 = 70.0;

    if !is_within_accuracy(y, POSSIBLE_PANEL_INDICES_Y, ACCURACY) {
        return Err(FieldError::new("".to_string()));
    }
    for (coords, index) in POSSIBLE_PANEL_INDICES_X {
        if is_within_accuracy(x, coords, ACCURACY) {
            return Ok(index)
        }
    }
    Err(FieldError::new("".to_string()))
}

pub fn is_restart_clicked(x: f32, y: f32) -> bool {
    const ACCURACY: f32 = 30.0;

    is_within_accuracy(x, RESTART_X, ACCURACY) && is_within_accuracy(y, RESTART_Y, ACCURACY)
}

fn is_within_accuracy(a: f32, target: f32, accuracy: f32) -> bool {
    (target - accuracy) <= a && (target + accuracy) >= a
}


pub fn compute_button_down(position: usize, game: &mut Game) -> Result<(), FieldError> {
    let color: u8 = game.get_token_at(position);
    match game.get_state() {
        State::Normal => {
            if color == game.get_player_turn() &&
                    (possible_move_count_of_position(game.get_board(), position) > 0 || game.get_piece_count(color) == 3) {
                game.set_token_at(position, 0b0);
                game.set_carry_piece(Option::Some((position, color)));
            } else {
                return Err(FieldError::empty());
            }
        },
        State::Setup | State::Take | State::Win => {}
    }
    Ok(())
}

pub fn compute_button_up(position: usize, game: &mut Game) -> Result<(), FieldError> {
    let carry_piece: Option<CarryPiece> = game.get_carry_piece();
    let token_type: u8 = game.get_token_at(position);

    match game.get_state() {
        State::Setup => {
            if token_type == 0b0 {
                let player_color = game.get_player_turn();

                game.set_token_at(position, player_color);
                game.reduce_setup_pieces_left();
                
                if is_part_of_mill(game.get_board(), position, player_color) {
                    game.update_state(Option::Some(State::Take));
                } else {
                    game.next_player_turn();
                }
            }
        }, State::Normal => {
            if carry_piece.is_some() {
                let (carry_pos, carry_piece_color) = carry_piece.unwrap().into();

                if is_move_valid(carry_pos, position, token_type, game.get_piece_count(carry_piece_color) + 1) {
                    let board_before: u64 = game.get_board();
                    game.set_token_at(position, carry_piece_color);
                    game.set_carry_piece(Option::None);
                    
                    if is_mill_closing(board_before, game.get_board(), carry_piece_color) {
                        game.update_state(Option::Some(State::Take));
                        println!("{} has created a mill", carry_piece_color);
                    } else {
                        game.next_player_turn();
                    }
                }
            }
        }, State::Take => {
            if token_type == negate_token(game.get_player_turn()) && is_beat_possible(game.get_board(), position, game.get_player_turn()) {
                game.set_token_at(position, 0b0);
                game.next_player_turn();
                game.update_state(Option::Some(State::Normal));
            }
        }, State::Win => { }
    }
    game.undo_carry();
    game.update_state(Option::None);

    Ok(())
}

pub fn compute_computer_step(action: Action, game: &mut Game) -> Result<(), FieldError> {
    let (start_position, end_position, beatable_position): (Option<usize>, usize, Option<usize>) = action.into();

    if start_position.is_some() && compute_button_down(start_position.unwrap(), game).is_err() {
        panic!("Invalid AI start position: {}", start_position.unwrap());
    }

    if compute_button_up(end_position, game).is_err() {
        panic!("Invalid AI end position: {}", end_position);
    }
    
    if let Some(beatable_position) = beatable_position {
        if compute_button_up(beatable_position, game).is_err() {
            panic!("Invalid AI beatable position: {}", beatable_position);
        }
    }
    
    Ok(())
}


pub fn compute_bottom_panel(old_white: Difficulty, old_black: Difficulty, index: usize) -> (Difficulty, Difficulty) {
    let toggle = |old: Difficulty, new: Difficulty| {
        if old == new {
            Difficulty::Off
        } else {
            new
        }
    };

    match index {
        0 => (toggle(old_white, Difficulty::Hard), old_black),
        1 => (toggle(old_white, Difficulty::Medium), old_black),
        2 => (toggle(old_white, Difficulty::Easy), old_black),
        3 => (old_white, toggle(old_black, Difficulty::Easy)),
        4 => (old_white, toggle(old_black, Difficulty::Medium)),
        5 => (old_white, toggle(old_black, Difficulty::Hard)),
        _ => (old_white, old_black)
    }
}