use std::collections::HashSet;

use ggez::{graphics::Image, Context};

use crate::engine;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    None,
    White,
    Black
}

pub struct Game {
    /* [[left-top-outer, left-top-middle, left-top-inner], [middle-top-outer, middle-top-middle, middle-top-inner] ...]
        0 - empty
        1 - white piece
        2 - black piece
    */
    board: [[Piece; 3]; 8],
    player_turn: Piece,
    
    // Amount of stones left to be placed by both players
    setup_phase: u8,
    win: bool,
    
    pub images: [Image; 3],
    pub window_scale: f32,
}


impl Game {
    pub fn new(_gtx: &mut Context, window_scale: f32) -> Game {

        let background_image = Image::from_path(_gtx, "/muehle_board.png").unwrap();
        let piece_white_image = Image::from_path(_gtx, "/muehle_white_piece.png").unwrap();
        let piece_black_image = Image::from_path(_gtx, "/muehle_black_piece.png").unwrap();

        Game {
            board: [[Piece::None; 3]; 8],
            player_turn: Piece::White,
            setup_phase: 18,
            win: false,
            images: [background_image, piece_white_image, piece_black_image],
            window_scale: window_scale
        }
    }
    
    pub fn get_board(&mut self) -> [[Piece; 3]; 8] {
        self.board.clone()
    }

    pub fn set_example_board(&mut self) {
        let example_board: [[Piece; 3]; 8] = [[Piece::None, Piece::White, Piece::Black]; 8];

        self.board = example_board;
        self.player_turn = Piece::White;
        self.setup_phase = 0;
    }

    fn next_player_turn(&mut self) {
        if self.player_turn == Piece::White {
            self.player_turn = Piece::Black;
        } else if self.player_turn == Piece::Black {
            self.player_turn = Piece::White;
        } else {
            panic!("invalid player turn detected");
        }
    }

    pub fn get_player_turn(self) -> Piece {
        self.player_turn
    }

    /*
        Calculates which field is being clicked by having formed a rectangle around each position

        A Hashset is being created and will keep (through set intersection) possible values for x and ring.
        If the Hashset is inconclusive or empty, the click is not on any field.
    */
    pub fn get_piece(self, x:f32, y:f32) -> (usize, usize, Piece){
        let accuracy: f32 = 0.2; // 1 -> pixel perfect, 0 -> will lead to hitbox overlapping

        let remaining_x: HashSet<i32> = HashSet::from([0,1,2,3,4,5,6,7]);
        let remaining_ring: HashSet<i32> = HashSet::from([0,1,2]);

        for (index, spot) in [165.0, 325.0, 485.0, 635.0, 785.0, 945.0, 1105.0].iter().enumerate() {
            let min_border: f32 = spot*self.window_scale*(1.0-accuracy);
            let max_border: f32 = spot*self.window_scale*(1.0+accuracy);

            if x > min_border && x < max_border {
                if index < 3 {
                    remaining_x.intersection(&HashSet::from([0,6,7]));
                    remaining_ring.intersection(&HashSet::from([index as i32]));
                } else if index == 3 {
                    remaining_x.intersection(&HashSet::from([1,5]));
                } else {
                    remaining_x.intersection(&HashSet::from([2,3,4]));
                    remaining_ring.intersection(&HashSet::from([6 - (index as i32)]));
                }
            }
            if y > min_border && y < max_border {
                if index < 3 {
                    remaining_x.intersection(&HashSet::from([0,1,2]));
                    remaining_ring.intersection(&HashSet::from([index as i32]));
                } else if index == 3 {
                    remaining_x.intersection(&HashSet::from([3,7]));
                } else {
                    remaining_x.intersection(&HashSet::from([4,5,6]));
                    remaining_ring.intersection(&HashSet::from([6 - (index as i32)]));
                }
            }
        }

        if remaining_x.len() != 1 || remaining_ring.len() != 1 {
            
        }

        return (1,2,Piece::None);
    }

    fn get_sorted_board(&self) -> [[Piece; 3]; 8] {
        let mut sorted_board = [[Piece::None; 3]; 8];
        let mut counter_x: u8 = 0;
        let mut counter_ring: u8 = 0;

        for x in self.board.iter() {
            counter_ring = 0;
            for ele in x.iter() {
                if counter_x <= 2 {
                    sorted_board[counter_ring as usize][counter_x as usize] = *ele;
                } else if counter_x == 3 {
                    sorted_board[4][(2-counter_ring) as usize] = *ele;
                } else if counter_x <= 6 {
                    sorted_board[(7-counter_ring) as usize][(6-counter_x) as usize] = *ele;
                } else {
                    sorted_board[3][counter_ring as usize] = *ele;
                }
                counter_ring += 1;
            }
            counter_x += 1;
        }

        return sorted_board;
    }

    fn set_win(&mut self, win: bool) {
        self.win = win;
    }
}