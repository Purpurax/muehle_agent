use crate::engine;

pub fn move_piece(player_color: engine::Piece, move_from: (i8, i8), move_to: (i8, i8), mut board: [[engine::Piece; 3]; 8]) -> (bool, [[engine::Piece; 3]; 8]) {
    if board[move_from.0 as usize][move_from.1 as usize] != player_color { // desired piece is of wrong color or none
        return (false, board);
    }
    if board[move_to.0 as usize][move_to.1 as usize] != engine::Piece::None { // desired spot is not empty
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
    } else if (pos1_ring - pos2_ring).abs() == 1 && pos1_x%2 == 1 && pos1_x == pos2_x {
        return true;
    }
    return false;
}

fn is_creating_mill(player_color: engine::Piece, position: (i8, i8), board: [[engine::Piece; 3]; 8]) -> bool {
    let pos_x: usize = position.0 as usize;
    let pos_ring: usize = position.0 as usize;

    
    // mill not changing ring
    if pos_x%2 == 0 { // corner position
        if board[(pos_x + 1)%8][pos_ring] == player_color && board[(pos_x + 2)%8][pos_ring] == player_color {
            return true;
        }
        if board[(pos_x - 1)%8][pos_ring] == player_color && board[(pos_x - 2)%8][pos_ring] == player_color {
            return true;
        }
    } else { // middle position
        if board[(pos_x + 1)%8][pos_ring] == player_color && board[(pos_x - 1)%8][pos_ring] == player_color {
            return true;
        }
        // mill crossing ring
        if board[pos_x][(pos_ring + 1)%3] == player_color && board[pos_x][(pos_ring + 2)%3] == player_color {
            return true;
        }
    }

    return false;
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
            engine::Piece::Black,
            (0,0),
            (1,1),
            [[engine::Piece::Black, engine::Piece::None, engine::Piece::White]; 8]
        ), (false,
        [[engine::Piece::Black, engine::Piece::None, engine::Piece::White]; 8]
        ),
        "Test 1 failed"
    );
    assert_eq!(
        move_piece(
            engine::Piece::Black,
            (1,0),
            (1,1),
            [[engine::Piece::Black, engine::Piece::None, engine::Piece::White]; 8]
        ), (true,
        [[engine::Piece::Black, engine::Piece::None, engine::Piece::White],
            [engine::Piece::None, engine::Piece::Black, engine::Piece::White],
            [engine::Piece::Black, engine::Piece::None, engine::Piece::White],
            [engine::Piece::Black, engine::Piece::None, engine::Piece::White],
            [engine::Piece::Black, engine::Piece::None, engine::Piece::White],
            [engine::Piece::Black, engine::Piece::None, engine::Piece::White],
            [engine::Piece::Black, engine::Piece::None, engine::Piece::White],
            [engine::Piece::Black, engine::Piece::None, engine::Piece::White]]
        ),
        "Test 2 failed"
    );
    assert_eq!(
        move_piece(
            engine::Piece::White,
            (6,1),
            (6,2),
            [[engine::Piece::None, engine::Piece::White, engine::Piece::Black]; 8]
        ), (false,
        [[engine::Piece::None, engine::Piece::White, engine::Piece::Black]; 8]
        ),
        "Test 3 failed"
    );
    assert_eq!(
        move_piece(
            engine::Piece::White,
            (5,1),
            (5,0),
            [[engine::Piece::None, engine::Piece::White, engine::Piece::Black]; 8]
        ), (true,
        [[engine::Piece::None, engine::Piece::White, engine::Piece::Black],
            [engine::Piece::None, engine::Piece::White, engine::Piece::Black],
            [engine::Piece::None, engine::Piece::White, engine::Piece::Black],
            [engine::Piece::None, engine::Piece::White, engine::Piece::Black],
            [engine::Piece::None, engine::Piece::White, engine::Piece::Black],
            [engine::Piece::White, engine::Piece::None, engine::Piece::Black],
            [engine::Piece::None, engine::Piece::White, engine::Piece::Black],
            [engine::Piece::None, engine::Piece::White, engine::Piece::Black]]
        ),
        "Test 4 failed"
    );
    assert_eq!(
        move_piece(
            engine::Piece::Black,
            (0,0),
            (1,0),
            [[engine::Piece::Black, engine::Piece::Black, engine::Piece::Black]; 8]
        ), (false,
        [[engine::Piece::Black, engine::Piece::Black, engine::Piece::Black]; 8]
        ),
        "Test 5 failed"
    );
    assert_eq!(
        move_piece(
            engine::Piece::Black,
            (1,0),
            (1,1),
            [[engine::Piece::Black, engine::Piece::White, engine::Piece::None]; 8]
        ), (false,
            [[engine::Piece::Black, engine::Piece::White, engine::Piece::None]; 8]
        ),
        "Test 5 failed"
    );
}