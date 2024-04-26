use std::collections::HashSet;

use ggez::{graphics::Image, input::gamepad::gilrs::MappingError, Context};

use crate::engine;

#[derive(Debug)]
pub struct FieldError {
    pub message: String,
}
impl FieldError {
    fn new(message: &str) -> FieldError {
        FieldError {
            message: message.to_string()
        }
    }
}


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
    carry_piece: (Piece, Option<Image>), 
    
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
            window_scale: window_scale,
            carry_piece: (Piece::None, Option::None),
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

    pub fn get_player_turn(&mut self) -> Piece {
        self.player_turn
    }

    pub fn get_piece_color(&mut self, x:usize, ring:usize) -> Piece {
        self.board[x][ring]
    }

    pub fn get_carry_piece(&mut self) -> (Piece, Option<Image>) {
        self.carry_piece.clone()
    }

    pub fn set_carry_piece(&mut self, piece_color: Piece) {
        let image: Option<Image>;
        if piece_color == Piece::White {
            image = Option::Some(self.images[1].clone());
        } else if piece_color == Piece::Black {
            image = Option::Some(self.images[2].clone());
        } else {
            image = Option::None;
        }
        self.carry_piece = (piece_color, image);
    }

    /*
        Calculates which field is being clicked by having formed a rectangle around each position

        A Hashset is being created and will keep (through set intersection) possible values for x and ring.
        If the Hashset is inconclusive or empty, the click is not on any field.
    */
    pub fn get_board_indices(&mut self, x:f32, y:f32) -> Result<(usize, usize), FieldError>{
        let accuracy: f32 = 0.2; // 1 -> pixel perfect, 0 -> will lead to hitbox overlapping

        let mut remaining_x: HashSet<i32> = HashSet::from([0,1,2,3,4,5,6,7]);
        let mut remaining_ring: HashSet<i32> = HashSet::from([0,1,2]);

        for (index, spot) in [165.0, 325.0, 485.0, 635.0, 785.0, 945.0, 1105.0].iter().enumerate() {
            let min_border: f32 = spot*self.window_scale*(1.0-accuracy);
            let max_border: f32 = spot*self.window_scale*(1.0+accuracy);

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
            return Err(FieldError::new("The clicked position is invalid, please try again"));
        }
        return Ok((*remaining_x.iter().next().unwrap() as usize, *remaining_ring.iter().next().unwrap() as usize));
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

    pub fn clear_field(&mut self, x:usize, ring:usize) {
        let mut new_board = self.board.clone();
        new_board[x][ring] = Piece::None;

        self.board = new_board;
    }

    fn set_win(&mut self, win: bool) {
        self.win = win;
    }
}