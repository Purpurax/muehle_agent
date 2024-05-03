use std::collections::HashSet;

use crate::ai;

use super::{enums::{CarryPiece, FieldError, Piece, State}, game::Game, snapshot};

fn move_piece(player_color: Piece, move_from: (usize, usize), move_to: (usize, usize), mut board: [[Piece; 3]; 8]) -> (bool, [[Piece; 3]; 8]) {
    if board[move_to.0][move_to.1] != Piece::None { // desired spot is not empty
        board[move_from.0][move_from.1] = Piece::None;
        board[move_to.0][move_to.1] = player_color;
        return (false, board);
    }

    if is_neighbor(move_from, move_to) {
        board[move_from.0][move_from.1] = Piece::None;
        board[move_to.0][move_to.1] = player_color;
        return (true, board);
    }

    board[move_from.0][move_from.1] = Piece::None;
    board[move_to.0][move_to.1] = player_color;
    return (false, board);
}

pub fn is_neighbor(position1: (usize, usize), position2: (usize, usize)) -> bool {
    let (pos1_x, pos1_ring): (usize, usize) = position1;
    let (pos2_x, pos2_ring): (usize, usize) = position2;

    if pos1_ring == pos2_ring && (pos1_x.abs_diff(pos2_x) % 6) == 1 {
        return true;
    } else if pos1_ring.abs_diff(pos2_ring) == 1 && pos1_x%2 == 1 && pos1_x == pos2_x {
        return true;
    }
    return false;
}

pub fn is_soft_locked(piece_color: Piece, board: [[Piece; 3]; 8]) -> bool {
    for (x, diagonal_row) in board.iter().enumerate() {
        for (ring, piece) in diagonal_row.iter().enumerate() {
            if *piece == piece_color && has_possible_move((x, ring), board) {
                return false
            }
        }
    }
    return true
}

fn has_possible_move((x, ring): (usize, usize), board: [[Piece; 3]; 8]) -> bool {
    if x%2 == 1 {
        match ring {
            0 => if board[x][1] == Piece::None { return true },
            1 => if board[x][0] == Piece::None || board[x][2] == Piece::None { return true },
            2 => if board[x][1] == Piece::None { return true },
            _ => {}
        }
    }
    if board[((x as i8) - 1).rem_euclid(8) as usize][ring] == Piece::None || board[(x+1)%8][ring] == Piece::None {
        return true
    }
    return false
}

pub fn can_take_piece(piece_color: Piece, (board_x, board_ring): (usize, usize), board: [[Piece; 3]; 8]) -> bool {
    !is_piece_part_of_mill(piece_color, (board_x, board_ring), board) || all_pieces_are_in_mills(piece_color, board)
}

fn is_piece_part_of_mill(piece_color: Piece, (board_x, board_ring): (usize, usize), board: [[Piece; 3]; 8]) -> bool {
    is_creating_mill(piece_color, (board_x, board_ring), board)
}


fn all_pieces_are_in_mills(piece_color: Piece, board: [[Piece; 3]; 8]) -> bool {
    for (x, diagonal_row) in board.iter().enumerate() {
        for (ring, piece) in diagonal_row.iter().enumerate() {
            if *piece == piece_color && !is_piece_part_of_mill(piece_color, (x, ring), board) {
                return false;
            }
        }
    }
    return true;
}

fn is_creating_mill(player_color: Piece, position: (usize, usize), board: [[Piece; 3]; 8]) -> bool {
    let pos_x: i8 = position.0 as i8;
    let pos_ring: i8 = position.1 as i8;

    
    if pos_x%2 == 0 {
        if board[((pos_x + 1)%8) as usize][pos_ring as usize] == player_color && board[((pos_x + 2)%8) as usize][pos_ring as usize] == player_color {
            return true;
        }
        if board[((pos_x - 1).rem_euclid(8)) as usize][pos_ring as usize] == player_color && board[((pos_x - 2).rem_euclid(8)) as usize][pos_ring as usize] == player_color {
            return true;
        }
    } else {
        if board[((pos_x + 1).rem_euclid(8)) as usize][pos_ring as usize] == player_color && board[((pos_x - 1).rem_euclid(8)) as usize][pos_ring as usize] == player_color {
            return true;
        }
        if board[pos_x as usize][((pos_ring + 1)%3) as usize] == player_color && board[pos_x as usize][((pos_ring + 2)%3) as usize] == player_color {
            return true;
        }
    }

    return false;
}


/// Calculates which field is being clicked by having formed a rectangle around each position
/// 
/// A Hashset is being created and will keep (through set intersection) possible values for x and ring.
/// If the Hashset is inconclusive or empty, the click is not on any field.
/// See more on coordinates-datastructure-connection.jpg
pub fn get_board_indices(game: &mut Game, x:f32, y:f32) -> Result<(usize, usize), FieldError>{
    let accuracy: f32 = 65.0;

    let mut remaining_x: HashSet<i32> = HashSet::from([0,1,2,3,4,5,6,7]);
    let mut remaining_ring: HashSet<i32> = HashSet::from([0,1,2]);

    for (index, spot) in [165.0, 325.0, 485.0, 635.0, 785.0, 945.0, 1105.0].iter().enumerate() {
        let min_border: f32 = (spot - accuracy)*game.window_scale;
        let max_border: f32 = (spot + accuracy)*game.window_scale;

        if x > min_border && x < max_border {
            if index < 3 {
                remaining_x = remaining_x.intersection(&HashSet::from([0,6,7])).cloned().collect();
                remaining_ring = remaining_ring.intersection(&HashSet::from([index as i32])).cloned().collect();
            } else if index == 3 {
                remaining_x = remaining_x.intersection(&HashSet::from([1,5])).cloned().collect();
            } else {
                remaining_x = remaining_x.intersection(&HashSet::from([2,3,4])).cloned().collect();
                remaining_ring = remaining_ring.intersection(&HashSet::from([6 - (index as i32)])).cloned().collect();
            }
        }
        if y > min_border && y < max_border {
            if index < 3 {
                remaining_x = remaining_x.intersection(&HashSet::from([0,1,2])).cloned().collect();
                remaining_ring = remaining_ring.intersection(&HashSet::from([index as i32])).cloned().collect();
            } else if index == 3 {
                remaining_x = remaining_x.intersection(&HashSet::from([3,7])).cloned().collect();
            } else {
                remaining_x = remaining_x.intersection(&HashSet::from([4,5,6])).cloned().collect();
                remaining_ring = remaining_ring.intersection(&HashSet::from([6 - (index as i32)])).cloned().collect();
            }
        }
    }

    if remaining_x.len() != 1 || remaining_ring.len() != 1 {
        return Err(FieldError::new(format!("Invalid position: {}.x {}.y", x, y)));
    }
    return Ok((*remaining_x.iter().next().unwrap() as usize, *remaining_ring.iter().next().unwrap() as usize));
}

/// Does all the computation that is needed for moving pieces around and changing game states
pub fn compute_step(mouse_button_down: bool, x: f32, y: f32, game: &mut Game) -> Result<(), FieldError> {
    let (board_x, board_ring) = 
        if game.get_player_turn() == Piece::White || !game.play_against_computer { // TODO add ability to play black as player
            match get_board_indices(game, x, y) {
                Ok((board_x, board_ring)) => (board_x, board_ring),
                Err(e) => {
                    game.undo_carry();
                    return Err(e)
                }
            }
        } else {
            ai::compute_step()
        };
    let piece_color: Piece = game.get_piece_color(board_x, board_ring);
        
    if mouse_button_down {
        compute_button_down(board_x, board_ring, piece_color, game)
    } else {
        let carry_piece = game.get_carry_piece();
        let state = game.get_state();

        if carry_piece.is_none() && (state == State::Normal || state == State::End) {
            game.undo_carry();
            return Err(FieldError::empty());
        }

        compute_button_up(board_x, board_ring, piece_color, carry_piece, state, game)
    }
}

fn compute_button_down(board_x: usize, board_ring: usize, piece_color: Piece, game: &mut Game) -> Result<(), FieldError> {
    match game.get_state() {
        State::Normal | State::End => {
            if piece_color == game.get_player_turn() {
                game.set_field(board_x, board_ring, Piece::None);
                game.set_carry_piece(Option::Some((board_x, board_ring, piece_color)));
            } else {
                return Err(FieldError::empty());
            }
        },
        State::Setup | State::Take | State::Win => {}
    }
    Ok(())
}

fn compute_button_up(board_x: usize, board_ring: usize, piece_color: Piece, carry_piece: Option<CarryPiece>, state: State, game: &mut Game) -> Result<(), FieldError> {
    match state {
        State::Setup => {
            if piece_color == Piece::None {
                let player_color = game.get_player_turn();
                game.set_field(board_x, board_ring, player_color);
                game.reduce_setup_pieces_left();
                
                if is_creating_mill(player_color, (board_x, board_ring), game.get_board()) {
                    game.update_state(Option::Some(State::Take));
                } else {
                    game.next_player_turn();
                }
            }
        }, State::Normal => {
            if carry_piece.is_some() {
                let (carry_x, carry_ring, carry_piece_color, _carry_image) = carry_piece.unwrap().into();
                let (successful, new_board) = move_piece(carry_piece_color, (carry_x, carry_ring), (board_x, board_ring), game.get_board());
                if successful {
                    game.set_board(new_board);
                    game.set_carry_piece(Option::None);
                    
                    if is_creating_mill(carry_piece_color, (board_x, board_ring), new_board) {
                        game.update_state(Option::Some(State::Take));
                        println!("{} has created a mill", carry_piece_color.to_str());
                    } else {
                        game.next_player_turn();
                    }
                }
            }
        }, State::Take => {
            if piece_color == game.get_player_turn().neg() && can_take_piece(piece_color, (board_x, board_ring), game.get_board()) {
                game.set_field(board_x, board_ring, Piece::None);
                game.next_player_turn();
                game.update_state(Option::Some(State::Normal));
            }
        }, State::End => {
            let (carry_x, carry_ring, carry_piece_color, _image) = carry_piece.unwrap().into();
            let (successful, new_board) = move_piece(carry_piece_color, (carry_x, carry_ring), (board_x, board_ring), game.get_board());
            let count = game.get_piece_count(carry_piece_color);
            if (count == 3 && (board_x != carry_x || board_ring != carry_ring) && piece_color == Piece::None) || successful {
                game.set_board(new_board);
                game.set_carry_piece(Option::None);
                if is_creating_mill(carry_piece_color, (board_x, board_ring), game.get_board()) {
                    game.update_state(Option::Some(State::Take));
                    println!("{} has created a mill", carry_piece_color.to_str());
                } else {
                    game.next_player_turn();
                }
            }
            
        }, State::Win => { }
    }
    game.undo_carry();
    snapshot::save_game(game);
    Ok(())
}























#[test]
fn test_is_neighbor() {
    assert!(is_neighbor((0,0), (1,0)), "Test 1 failed");
    assert!(is_neighbor((0,0), (7,0)), "Test 2 failed");
    assert!(is_neighbor((3,2), (4,2)), "Test 3 failed");
    assert!(is_neighbor((6,1), (5,1)), "Test 4 failed");
    assert!(!is_neighbor((6,1), (6,2)), "Test 5 failed");
    assert!(!is_neighbor((6,1), (4,1)), "Test 6 failed");
    assert!(!is_neighbor((0,0), (0,1)), "Test 7 failed");
    assert!(!is_neighbor((0,0), (1,1)), "Test 8 failed");
}

#[test]
fn test_move_piece() {
    assert_eq!(
        move_piece(
            Piece::Black,
            (0,0),
            (1,1),
            [[Piece::Black, Piece::None, Piece::White]; 8]
        ), (false,
        [[Piece::Black, Piece::None, Piece::White]; 8]
        ),
        "Test 1 failed"
    );
    assert_eq!(
        move_piece(
            Piece::Black,
            (1,0),
            (1,1),
            [[Piece::Black, Piece::None, Piece::White]; 8]
        ), (true,
        [[Piece::Black, Piece::None, Piece::White],
            [Piece::None, Piece::Black, Piece::White],
            [Piece::Black, Piece::None, Piece::White],
            [Piece::Black, Piece::None, Piece::White],
            [Piece::Black, Piece::None, Piece::White],
            [Piece::Black, Piece::None, Piece::White],
            [Piece::Black, Piece::None, Piece::White],
            [Piece::Black, Piece::None, Piece::White]]
        ),
        "Test 2 failed"
    );
    assert_eq!(
        move_piece(
            Piece::White,
            (6,1),
            (6,2),
            [[Piece::None, Piece::White, Piece::Black]; 8]
        ), (false,
        [[Piece::None, Piece::White, Piece::Black]; 8]
        ),
        "Test 3 failed"
    );
    assert_eq!(
        move_piece(
            Piece::White,
            (5,1),
            (5,0),
            [[Piece::None, Piece::White, Piece::Black]; 8]
        ), (true,
        [[Piece::None, Piece::White, Piece::Black],
            [Piece::None, Piece::White, Piece::Black],
            [Piece::None, Piece::White, Piece::Black],
            [Piece::None, Piece::White, Piece::Black],
            [Piece::None, Piece::White, Piece::Black],
            [Piece::White, Piece::None, Piece::Black],
            [Piece::None, Piece::White, Piece::Black],
            [Piece::None, Piece::White, Piece::Black]]
        ),
        "Test 4 failed"
    );
    assert_eq!(
        move_piece(
            Piece::Black,
            (0,0),
            (1,0),
            [[Piece::Black, Piece::Black, Piece::Black]; 8]
        ), (false,
        [[Piece::Black, Piece::Black, Piece::Black]; 8]
        ),
        "Test 5 failed"
    );
    assert_eq!(
        move_piece(
            Piece::Black,
            (1,0),
            (1,1),
            [[Piece::Black, Piece::White, Piece::None]; 8]
        ), (false,
            [[Piece::Black, Piece::White, Piece::None]; 8]
        ),
        "Test 5 failed"
    );
}