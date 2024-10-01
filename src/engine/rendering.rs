use std::collections::HashMap;

use ggez::graphics::{Image, Rect};

use super::{enums::{CarryPiece, Piece, State}, logic};


/// Chooses the right image out of an image list according to those factors:
///  - state
///  - player color
///  - field color
///  - field position
///  - carry piece
///  - piece count of player color
pub fn calculate_image(state: State, player_color: Piece, field_color: Piece, field_position: (usize, usize), board: [[Piece; 3]; 8], carry_piece: Option<CarryPiece>, piece_count: u8, images: HashMap<String, Image>) -> Option<Image> {
    match state {
        State::Setup => calculate_image_for_setup(player_color, field_color, images),
        State::Normal => calculate_image_for_normal(player_color, field_color, field_position, carry_piece, images),
        State::Take => calculate_image_for_take(player_color, field_color, field_position, board, images),
        State::End => calculate_image_for_end(player_color, field_color, field_position, carry_piece, piece_count, images),
        State::Win => calculate_image_for_win(player_color, images), 
    }
}


/// Calculates the centered and scaled image position for 160x160 images
/// Note: ggez has an intern image shifting, which is not under our control and has to be accounted for
pub fn calculate_image_position(board_x: usize, board_ring: usize, window_scale: f32) -> Rect {
    let x: f32 =
        if [0,6,7].contains(&board_x) {
            165.0 + (board_ring as f32)*160.0
        } else if [1,5].contains(&board_x) {
            635.0
        } else {
            1105.0 - (board_ring as f32)*160.0
        };
    let y: f32 =
        if [0,1,2].contains(&board_x) {
            165.0 + (board_ring as f32)*160.0
        } else if [3,7].contains(&board_x) {
            635.0
        } else {
            1105.0 - (board_ring as f32)*160.0
        };
        
        let x_centered = x - 75.0;
        let y_centered = y - 75.0;
        
        let x_scaled = x_centered*window_scale;
        let y_scaled = y_centered*window_scale;
        
        Rect {
            x: x_scaled,
        y: y_scaled,
        w: window_scale,
        h: window_scale
    }
}





fn calculate_image_for_setup(player_color: Piece, field_color: Piece, images: HashMap<String, Image>) -> Option<Image> {
    let image = 
        match field_color {
            Piece::White => images["white"].clone(),
            Piece::Black => images["black"].clone(),
            Piece::None => if player_color == Piece::White {
                images["empty white outlined"].clone()
            } else if player_color == Piece::Black {
                images["empty black outlined"].clone()
            } else {
                return Option::None;
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_normal(player_color: Piece, field_color: Piece, field_position: (usize, usize), carry_piece: Option<CarryPiece>, images: HashMap<String, Image>) -> Option<Image> {
    let image = 
        if carry_piece.is_some() {
            match field_color {
                Piece::White => images["white"].clone(),
                Piece::Black => images["black"].clone(),
                Piece::None => {
                    let (carry_x, carry_ring, _piece_color) = carry_piece.unwrap().into();
                    if logic::is_neighbor((carry_x, carry_ring), field_position) {
                        images["outline"].clone()
                    } else {
                        return Option::None;
                    }
                }
            }
        } else {
            match (player_color, field_color) {
                (Piece::White, Piece::White) => images["white outlined"].clone(),
                (Piece::White, Piece::Black) => images["black"].clone(),
                (Piece::Black, Piece::White) => images["white"].clone(),
                (Piece::Black, Piece::Black) => images["black outlined"].clone(),
                (_, _) => return Option::None
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_take(player_color: Piece, field_color: Piece, field_position: (usize, usize), board: [[Piece; 3]; 8], images: HashMap<String, Image>) -> Option<Image> {
    let image = 
        match (player_color, field_color) {
            (Piece::White, Piece::White) => images["white"].clone(),
            (Piece::White, Piece::Black) => {
                if logic::can_take_piece(Piece::Black, field_position, board)
                {
                    images["take black"].clone()
                } else {
                    images["black"].clone()
                }
            },
            (Piece::Black, Piece::White) => {
                if logic::can_take_piece(Piece::White, field_position, board)
                {
                    images["take white"].clone()
                } else {
                    images["white"].clone()
                }
            }
            (Piece::Black, Piece::Black) => images["black"].clone(),
            (_, _) => return Option::None
        };
    return Option::Some(image);
}

fn calculate_image_for_end(player_color: Piece, field_color: Piece, field_position: (usize, usize), carry_piece: Option<CarryPiece>, piece_count: u8, images: HashMap<String, Image>) -> Option<Image> {
    let image = 
        if carry_piece.is_some() {
            match field_color {
                Piece::White => images["white"].clone(),
                Piece::Black => images["black"].clone(),
                Piece::None => {
                    let (carry_x, carry_ring, _piece_color) = carry_piece.unwrap().into();
                    if (piece_count == 3 && (carry_x, carry_ring) != field_position) || logic::is_neighbor((carry_x, carry_ring), field_position) {
                            images["outline"].clone()
                    } else {
                        return Option::None;
                    }
                }
            }
        } else {
            match (player_color, field_color) {
                (Piece::White, Piece::White) => images["white outlined"].clone(),
                (Piece::White, Piece::Black) => images["black"].clone(),
                (Piece::Black, Piece::White) => images["white"].clone(),
                (Piece::Black, Piece::Black) => images["black outlined"].clone(),
                (_, _) => return Option::None
            }
        };
    return Option::Some(image);
}

fn calculate_image_for_win(player_color: Piece, images: HashMap<String, Image>) -> Option<Image> {
    Option::Some(if player_color == Piece::Black {
        images["white"].clone()
    } else {
        images["black"].clone()
    })
}