use crate::engine;

pub fn move_piece(player_color: engine::Piece, move_from: (i8, i8), move_to: (i8, i8), mut board: [[engine::Piece; 3]; 8]) -> (bool, [[engine::Piece; 3]; 8]) {
    if board[move_from.0 as usize][move_from.1 as usize] != player_color {
        return (false, board);
    }
    if board[move_to.0 as usize][move_to.1 as usize] != engine::Piece::None {
        return (false, board);
    }

    if is_neighbor(move_from, move_to) {
        board[move_from.0 as usize][move_from.1 as usize] = engine::Piece::None;
        board[move_to.0 as usize][move_to.1 as usize] = player_color;
        return (true, board);
    }
    return (false, board);
}

fn is_neighbor(position1: (i8, i8), position2: (i8, i8)) -> bool {
    let (pos1_x, pos1_ring): (i8, i8) = position1;
    let (pos2_x, pos2_ring): (i8, i8) = position2;

    if pos1_ring == pos2_ring && ((pos1_x - pos2_x).abs() % 6) == 1 {
        return true;
    } else if (pos1_ring - pos2_ring).abs() == 1 && pos1_x == pos2_x {
        return true;
    }
    return false;
}