use crate::core::enums::{CarryPiece, State};
use crate::core::position::{get_token_at, set_token_at};
use crate::core::utils::{extract_black_move_count_from_board, extract_white_move_count_from_board, insert_number_of_possible_moves_to_board, insert_token_count_to_board};

use super::utils::{extract_black_token_count_from_board, extract_white_token_count_from_board};


pub struct Game {
    board: u64,
    player_turn: u8,
    carry_piece: Option<CarryPiece>,
    
    state: State,
    setup_pieces_left: u8,
}


impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: 0b0,
            player_turn: 0b11,
            carry_piece: Option::None,
            state: State::Setup,
            setup_pieces_left: 18,
        }
    }
    
    pub fn new_example_board() -> Game {
        let board: u64 = 0b000000101100000010000000001100001010101010110000;
        Game {
            board: insert_number_of_possible_moves_to_board(insert_token_count_to_board(board)),
            player_turn: 0b11,
            carry_piece: Option::None,
            state: State::Normal,
            setup_pieces_left: 0,
        }
    }

    pub fn get_board(&self) -> u64 {
        self.board
    }

    pub fn set_board(&mut self, new_board: u64) {
        self.board = new_board
    }

    pub fn get_player_turn(&self) -> u8 {
        self.player_turn
    }

    pub fn next_player_turn(&mut self) {
        if self.player_turn == 0b11 {
            self.player_turn = 0b10;
        } else if self.player_turn == 0b10 {
            self.player_turn = 0b11;
        } else {
            panic!("invalid player turn detected");
        }
    }

    pub fn get_carry_piece(&self) -> Option<CarryPiece> {
        self.carry_piece.clone()
    }

    pub fn set_carry_piece(&mut self, new_carry_piece: Option<(usize, u8)>) {
        if new_carry_piece.is_none() {
            self.carry_piece = Option::None;
            return
        }

        let (position, piece_color) = new_carry_piece.unwrap();
        self.carry_piece = Option::Some(CarryPiece::new(position, piece_color));
    }

    pub fn undo_carry(&mut self) {
        if self.carry_piece.is_some() {
            let (position, piece_color) = self.get_carry_piece().unwrap().into();
            self.set_token_at(position, piece_color);
            self.set_carry_piece(Option::None);
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn update_state(&mut self, state: Option<State>) {
        if self.get_state() == State::Win {
            return
        }
        
        if state.is_some() {
            self.state = state.unwrap();
        }
        if self.state != State::Take {
            if self.setup_pieces_left > 0 {
                self.state = State::Setup;
            } else if self.get_winner() != 0b0 {
                self.state = State::Win;
            } else {
                self.state = State::Normal;
            }
        }
    }

    pub fn get_winner(&self) -> u8 {
        let (black_tokens, white_tokens) = (self.get_piece_count(0b10), self.get_piece_count(0b11));
        let (black_possible_moves, white_possible_moves) = (extract_black_move_count_from_board(self.get_board()), extract_white_move_count_from_board(self.get_board()));

        if white_tokens < 3 || (white_tokens > 3 && white_possible_moves == 0) {
            0b10
        } else if black_tokens < 3 || (black_tokens > 3 && black_possible_moves == 0) {
            return 0b11
        } else {
            return 0b00
        }
    }

    pub fn get_setup_pieces_left(&self) -> u8 {
        self.setup_pieces_left
    }

    pub fn get_piece_count(&self, piece_color: u8) -> u8 {
        if piece_color == 0b11 {
            extract_white_token_count_from_board(self.get_board()) as u8
        } else {
            extract_black_token_count_from_board(self.get_board()) as u8
        }
    }

    pub fn reduce_setup_pieces_left(&mut self) {
        let setup_pieces_left = self.setup_pieces_left - 1;
        self.setup_pieces_left = setup_pieces_left;

        let current_player_count: u8 = setup_pieces_left / 2;
        let next_player_count: u8 = setup_pieces_left / 2 + (setup_pieces_left % 2);
        println!("White can place {current_player_count} more pieces and Black can place {next_player_count} more pieces");

        if setup_pieces_left == 0 {
            self.update_state(Option::Some(State::Normal));
        }
    }

    pub fn get_token_at(&self, position: usize) -> u8 {
        let board: u64 = self.get_board();
        get_token_at(board, position)
    }

    pub fn set_token_at(&mut self, position: usize, color: u8) {
        let old_color: u8 = get_token_at(self.get_board(), position);
        let removed_token: bool = old_color != 0b0;
        let placed_token: bool = color != 0b0;
        if old_color == color {
            return
        }

        let mut new_board = self.get_board();
        if removed_token {
            new_board = set_token_at(new_board, position, 0b0);
        }
        if placed_token {
            new_board = set_token_at(new_board, position, color);
        }
        new_board = insert_number_of_possible_moves_to_board(new_board);
        new_board = insert_token_count_to_board(new_board);
        self.set_board(new_board);
    }
}