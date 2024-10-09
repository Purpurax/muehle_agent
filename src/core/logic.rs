use core::panic;

use crate::ai::action::Action;
use crate::core::enums::{CarryPiece, FieldError, State};
use crate::core::game::Game;
use crate::core::position::negate_token;
use crate::core::utils::{is_beat_possible, is_mill_closing, is_move_valid, is_part_of_mill};

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

pub fn calculate_position(x: f32, y: f32, window_scale: f32) -> Result<usize, FieldError> {
    let accuracy: f32 = 65.0 * window_scale;
    
    let mut possible_positions: Vec<usize> = vec![];
    for (coords, positions) in POSSIBLE_POSITIONS_X {
        if coords * window_scale - accuracy <= x && coords * window_scale + accuracy >= x {
            possible_positions.extend(&positions);
        }
    }
    
    for (coords, positions) in POSSIBLE_POSITIONS_Y {
        if coords * window_scale - accuracy <= y && coords * window_scale + accuracy >= y {
            for position in positions {
                if possible_positions.contains(&position) {
                    return Ok(position)
                }
            }
        }
    }

    return Err(FieldError::new(format!("Invalid position: {}.x {}.y", x, y)));
}

pub fn compute_button_down(position: usize, game: &mut Game) -> Result<(), FieldError> {
    let color: u8 = game.get_token_at(position);
    match game.get_state() {
        State::Normal | State::End => {
            if color == game.get_player_turn() {
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
        }, State::End => {
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
        }, State::Win => { }
    }
    game.undo_carry();
    game.update_state(Option::None);
    // snapshot::save_game(game);

    Ok(())
}

pub fn compute_computer_step(action: Action, game: &mut Game) -> Result<(), FieldError> {
    let (start_position, end_position, beatable_position): (Option<usize>, usize, Option<usize>) = action.into();
    if start_position.is_some() {
        if compute_button_down(start_position.unwrap(), game).is_err() {
            panic!("Invalid AI start position");
        }
    }
    
    // sleep(std::time::Duration::from_millis(500));

    if compute_button_up(end_position, game).is_err() {
        panic!("Invalid AI end position");
    }

    // sleep(std::time::Duration::from_millis(500));
    
    if beatable_position.is_some() {
        if compute_button_up(beatable_position.unwrap(), game).is_err() {
            panic!("Invalid AI beatable position2");
        }
    }
    
    Ok(())
}






















#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_position() {
        assert_eq!(7 , calculate_position(70.0, 70.0, 1.0).unwrap())
    }
}