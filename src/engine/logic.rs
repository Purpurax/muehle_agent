use crate::engine;

pub fn move_piece(player_color: engine::Piece, move_from: (usize, usize), move_to: (usize, usize), mut board: [[engine::Piece; 3]; 8]) -> (bool, [[engine::Piece; 3]; 8]) {
    if board[move_to.0][move_to.1] != engine::Piece::None { // desired spot is not empty
        board[move_from.0][move_from.1] = engine::Piece::None;
        board[move_to.0][move_to.1] = player_color;
        return (false, board);
    }

    if is_neighbor(move_from, move_to) {
        board[move_from.0][move_from.1] = engine::Piece::None;
        board[move_to.0][move_to.1] = player_color;
        return (true, board);
    }
    
    board[move_from.0][move_from.1] = engine::Piece::None;
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

pub fn is_creating_mill(player_color: engine::Piece, position: (usize, usize), board: [[engine::Piece; 3]; 8]) -> bool {
    let pos_x: i8 = position.0 as i8;
    let pos_ring: i8 = position.1 as i8;

    
    // mill not changing ring
    if pos_x%2 == 0 { // corner position
        if board[((pos_x + 1)%8) as usize][pos_ring as usize] == player_color && board[((pos_x + 2)%8) as usize][pos_ring as usize] == player_color {
            return true;
        }
        if board[((pos_x - 1).rem_euclid(8)) as usize][pos_ring as usize] == player_color && board[((pos_x - 2).rem_euclid(8)) as usize][pos_ring as usize] == player_color {
            return true;
        }
    } else { // middle position
        if board[((pos_x + 1).rem_euclid(8)) as usize][pos_ring as usize] == player_color && board[((pos_x - 1).rem_euclid(8)) as usize][pos_ring as usize] == player_color {
            return true;
        }
        // mill crossing ring
        if board[pos_x as usize][((pos_ring + 1)%3) as usize] == player_color && board[pos_x as usize][((pos_ring + 2)%3) as usize] == player_color {
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