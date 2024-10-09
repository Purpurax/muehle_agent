use std::collections::HashMap;

use ggez::graphics::{Image, Rect};

use crate::core::game::Game;
use crate::core::utils::{is_beat_possible, is_move_valid, is_neighbor, possible_move_count_of_position};
use crate::core::enums::{CarryPiece, State};


/// Chooses the right image out of an image list according to those factors:
///  - state
///  - player color
///  - field color
///  - field position
///  - carry piece
///  - piece count of player color
pub fn calculate_image(game: &Game, position: usize, images: HashMap<String, Image>) -> Option<Image> {
    let state = game.get_state();
    match state {
        State::Setup => calculate_image_for_setup(game, position, images),
        State::Normal => calculate_image_for_normal(game, position, images),
        State::Take => calculate_image_for_take(game, position, images),
        State::End => calculate_image_for_end(game, position, images),
        State::Win => calculate_image_for_win(game, position, images), 
    }
}


/// Calculates the centered and scaled image position for 160x160 images
pub fn calculate_image_position(position: u8, window_scale: f32) -> Rect {
    let x: f32 = 
        if [0, 4].contains(&(position % 8)) {
            640.0
        } else if [1, 2, 3].contains(&(position % 8)) {
            1110.0 - 160.0 * (position / 8) as f32
        } else {
            170.0 + 160.0 * (position / 8) as f32
        };
    let y: f32 =
        if [2, 6].contains(&(position % 8)) {
            640.0
        } else if [3, 4, 5].contains(&(position % 8)) {
            1110.0 - 160.0 * (position / 8) as f32
        } else {
            170.0 + 160.0 * (position / 8) as f32
        };
    
    let x_centered: f32 = x - 80.0;
    let y_centered: f32 = y - 80.0;

    let x_scaled: f32 = x_centered * window_scale;
    let y_scaled: f32 = y_centered * window_scale;

    Rect {
        x: x_scaled,
        y: y_scaled,
        w: window_scale,
        h: window_scale
    }
}


fn calculate_image_for_setup(game: &Game, position: usize, images: HashMap<String, Image>) -> Option<Image> {
    let player_color: u8 = game.get_player_turn();
    let field_color: u8 = game.get_token_at(position);

    let image = 
        match field_color {
            0b11 => images["white"].clone(),
            0b10 => images["black"].clone(),
            _ => if player_color == 0b11 {
                images["empty white outlined"].clone()
            } else if player_color == 0b10 {
                images["empty black outlined"].clone()
            } else {
                return Option::None;
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_normal(game: &Game, position: usize, images: HashMap<String, Image>) -> Option<Image> {
    let player_color: u8 = game.get_player_turn();
    let field_color: u8 = game.get_token_at(position);
    let carry_piece: Option<CarryPiece> = game.get_carry_piece();
    let board: u64 = game.get_board();
    
    let image = 
        if carry_piece.is_some() {
            match field_color {
                0b11 => images["white"].clone(),
                0b10 => images["black"].clone(),
                _ => {
                    let (carry_pos, _piece_color) = carry_piece.unwrap().into();
                    if is_neighbor(carry_pos, position) {
                        images["outline"].clone()
                    } else {
                        return Option::None;
                    }
                }
            }
        } else {
            if field_color == 0b11 && player_color == 0b11 && possible_move_count_of_position(board, position) > 0 {
                images["white outlined"].clone()
            } else if field_color == 0b11 {
                images["white"].clone()
            } else if field_color == 0b10 && player_color == 0b10 && possible_move_count_of_position(board, position) > 0 {
                images["black outlined"].clone()
            } else if field_color == 0b10 {
                images["black"].clone()
            } else {
                return Option::None;
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_take(game: &Game, position: usize, images: HashMap<String, Image>) -> Option<Image> {
    let player_color: u8 = game.get_player_turn();
    let field_color: u8 = game.get_token_at(position);
    let board: u64 = game.get_board();
    
    let image = 
        match (player_color, field_color) {
            (0b11, 0b11) => images["white"].clone(),
            (0b11, 0b10) => {
                if is_beat_possible(board, position, 0b11) {
                    images["take black"].clone()
                } else {
                    images["black"].clone()
                }
            },
            (0b10, 0b11) => {
                if is_beat_possible(board, position, 0b10) {
                    images["take white"].clone()
                } else {
                    images["white"].clone()
                }
            }
            (0b10, 0b10) => images["black"].clone(),
            (_, _) => return Option::None
        };
    return Option::Some(image);
}

fn calculate_image_for_end(game: &Game, position: usize, images: HashMap<String, Image>) -> Option<Image> {
    let player_color: u8 = game.get_player_turn();
    let field_color: u8 = game.get_token_at(position);
    let carry_piece: Option<CarryPiece> = game.get_carry_piece();
    let piece_count: u8 = game.get_piece_count(player_color);
    
    let image = 
        if carry_piece.is_some() {
            match field_color {
                0b11 => images["white"].clone(),
                0b10 => images["black"].clone(),
                _ => {
                    let (carry_pos, _carry_color) = carry_piece.unwrap().into();
                    if is_move_valid(carry_pos, position, field_color, piece_count) {
                            images["outline"].clone()
                    } else {
                        return Option::None;
                    }
                }
            }
        } else {
            match (player_color, field_color) {
                (0b11, 0b11) => images["white outlined"].clone(),
                (0b11, 0b10) => images["black"].clone(),
                (0b10, 0b11) => images["white"].clone(),
                (0b10, 0b10) => images["black outlined"].clone(),
                (_, _) => return Option::None
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_win(game: &Game, position: usize, images: HashMap<String, Image>) -> Option<Image> {
    let field_color: u8 = game.get_token_at(position);
    let player_color: u8 = game.get_player_turn();

    let image =
        match (player_color, field_color) {
            (0b11, 0b11) => images["take white"].clone(),
            (0b11, 0b10) => images["black outlined"].clone(),
            (0b10, 0b11) => images["white outlined"].clone(),
            (0b10, 0b10) => images["take black"].clone(),
            (_, _) => return Option::None
        };
    return Option::Some(image);
}

#[cfg(test)]
mod tests {
    use super::*;

    use ggez::graphics::Rect;
    #[test]
    fn test_calculate_image_position() {
        assert_eq!(Rect{
            x: 90.0,
            y: 90.0,
            w: 1.0,
            h: 1.0
        }, calculate_image_position(7, 1.0));
    }
}