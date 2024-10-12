use std::collections::HashMap;

use good_web_game::cgmath::Vector2;
use good_web_game as ggez;
use ggez::cgmath::Point2;
use ggez::graphics::Image;

use crate::core::game::Game;
use crate::core::utils::{is_beat_possible, is_move_valid, possible_move_count_of_position};
use crate::core::enums::{CarryPiece, Difficulty, State};


pub fn calculate_image(game: &Game, position: usize, images: &HashMap<String, Image>, computer_white: bool, computer_black: bool) -> Option<Image> {
    let state = game.get_state();
    match state {
        State::Setup => calculate_image_for_setup(game, position, images, computer_white, computer_black),
        State::Normal => calculate_image_for_normal(game, position, images, computer_white, computer_black),
        State::Take => calculate_image_for_take(game, position, images),
        State::Win => calculate_image_for_win(game, position, images),
    }
}


/// Calculates the centered and scaled image position for 160x160 images
pub fn calculate_image_position(position: usize, offsets: Point2<f32>, scales: Vector2<f32>) -> Point2<f32> {
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

    let x_scaled: f32 = x_centered * scales.x;
    let y_scaled: f32 = y_centered * scales.y;

    Point2::new(x_scaled + offsets.x, y_scaled + offsets.y)
}


fn calculate_image_for_setup(game: &Game, position: usize, images: &HashMap<String, Image>, computer_white: bool, computer_black: bool) -> Option<Image> {
    let player_color: u8 = game.get_player_turn();
    let field_color: u8 = game.get_token_at(position);

    let image = 
        match field_color {
            0b11 => images["white"].clone(),
            0b10 => images["black"].clone(),
            _ => if player_color == 0b11 && !computer_white {
                images["empty white outlined"].clone()
            } else if player_color == 0b10 && !computer_black {
                images["empty black outlined"].clone()
            } else {
                return Option::None;
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_normal(game: &Game, position: usize, images: &HashMap<String, Image>, computer_white: bool, computer_black: bool) -> Option<Image> {
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
                    let (carry_pos, piece_color) = carry_piece.unwrap().into();
                    if is_move_valid(carry_pos, position, field_color, game.get_piece_count(piece_color)) {
                        images["outline"].clone()
                    } else {
                        return Option::None;
                    }
                }
            }
        } else {
            if field_color == 0b11 && player_color == 0b11 && (possible_move_count_of_position(board, position) > 0 || game.get_piece_count(player_color) <= 3) && !computer_white {
                images["white outlined"].clone()
            } else if field_color == 0b11 {
                images["white"].clone()
            } else if field_color == 0b10 && player_color == 0b10 && (possible_move_count_of_position(board, position) > 0 || game.get_piece_count(player_color) <= 3) && !computer_black {
                images["black outlined"].clone()
            } else if field_color == 0b10 {
                images["black"].clone()
            } else {
                return Option::None;
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_take(game: &Game, position: usize, images: &HashMap<String, Image>) -> Option<Image> {
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

fn calculate_image_for_win(game: &Game, position: usize, images: &HashMap<String, Image>) -> Option<Image> {
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


pub fn calculate_bottom_panel_image(images: &HashMap<String, Image>, comp_white: Difficulty, comp_black: Difficulty) -> Image {
    let mut image_index;
    match comp_white {
        Difficulty::Off => image_index = 0,
        Difficulty::Easy => image_index = 4,
        Difficulty::Medium => image_index = 8,
        Difficulty::Hard => image_index = 12,
    }
    match comp_black {
        Difficulty::Off => image_index += 0,
        Difficulty::Easy => image_index += 1,
        Difficulty::Medium => image_index += 2,
        Difficulty::Hard => image_index += 3,
    }
    let image_name: String = format!("bottom panel {}", image_index);
    images[image_name.as_str()].clone()
}

pub fn calculate_bottom_panel_setup_image(images: &HashMap<String, Image>, setup_pieces_left: u8) -> Image {
    let image_name: String = format!("bottom panel setup {}", 18 - setup_pieces_left);
    images[image_name.as_str()].clone()
}