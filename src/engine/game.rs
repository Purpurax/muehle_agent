use std::collections::{HashMap, HashSet};

use ggez::{graphics::Image, input::gamepad::gilrs::MappingError, Context};

use crate::engine;

#[derive(Debug)]
pub struct FieldError {
    pub message: String,
}
impl FieldError {
    fn new(message: String) -> FieldError {
        FieldError {
            message
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    None,
    White,
    Black
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Setup,
    Normal,
    Take,
    Win
}

pub struct Game {
    /* [[left-top-outer, left-top-middle, left-top-inner], [middle-top-outer, middle-top-middle, middle-top-inner] ...]
        0 - empty
        1 - white piece
        2 - black piece
    */
    board: [[Piece; 3]; 8],
    player_turn: Piece,
    carry_piece: Option<(usize, usize, Piece, Image)>,
    
    // Amount of stones left to be placed by both players
    state: State,
    setup_pieces_left: u8,
    
    pub images: HashMap<String, Image>,
    pub window_scale: f32,
}


impl Game {
    pub fn new(_gtx: &mut Context, window_scale: f32) -> Game {

        let background_image = Image::from_path(_gtx, "/muehle_board.png").unwrap();
        let piece_white_image = Image::from_path(_gtx, "/muehle_white_piece.png").unwrap();
        let piece_white_outlined_image = Image::from_path(_gtx, "/muehle_white_piece_outlined.png").unwrap();
        let piece_black_image = Image::from_path(_gtx, "/muehle_black_piece.png").unwrap();
        let piece_black_outlined_image = Image::from_path(_gtx, "/muehle_black_piece_outlined.png").unwrap();
        let empty_white_outlined_image = Image::from_path(_gtx, "/muehle_no_white_piece_outlined.png").unwrap();
        let empty_black_outlined_image = Image::from_path(_gtx, "/muehle_no_black_piece_outlined.png").unwrap();

        Game {
            board: [[Piece::None; 3]; 8],
            player_turn: Piece::White,
            state: State::Setup,
            setup_pieces_left: 18,
            images: HashMap::from([
                ("background".to_string(), background_image),
                ("white".to_string(), piece_white_image),
                ("white outlined".to_string(), piece_white_outlined_image),
                ("black".to_string(), piece_black_image),
                ("black outlined".to_string(), piece_black_outlined_image),
                ("empty white outlined".to_string(), empty_white_outlined_image),
                ("empty black outlined".to_string(), empty_black_outlined_image),
            ]),
            window_scale: window_scale,
            carry_piece: Option::None,
        }
    }
    
    pub fn get_board(&mut self) -> [[Piece; 3]; 8] {
        self.board.clone()
    }

    pub fn set_board(&mut self, new_board: [[Piece; 3]; 8]) {
        self.board = new_board;
    }

    pub fn next_player_turn(&mut self) {
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

    pub fn get_carry_piece(&mut self) -> Option<(usize, usize, Piece, Image)> {
        self.carry_piece.clone()
    }

    pub fn set_carry_piece(&mut self, params: Option<(usize, usize, Piece)>) {
        if params.is_none() {
            self.carry_piece = Option::None;
            return
        }

        let (x, ring, piece_color) = params.unwrap();
        let image: Image;
        if piece_color == Piece::White {
            image = self.images["white"].clone();
        } else if piece_color == Piece::Black {
            image = self.images["black"].clone();
        } else {
            self.carry_piece = Option::None;
            return
        }
        self.carry_piece = Option::Some((x, ring, piece_color, image));
    }

    pub fn undo_carry(&mut self) {
        if self.get_carry_piece().is_some() {
            let (x, ring, piece_color, _image) = self.get_carry_piece().unwrap();
            self.board[x][ring] = piece_color;
            self.set_carry_piece(Option::None);
        }
    }

    /*
        Calculates which field is being clicked by having formed a rectangle around each position

        A Hashset is being created and will keep (through set intersection) possible values for x and ring.
        If the Hashset is inconclusive or empty, the click is not on any field.
    */
    pub fn get_board_indices(&mut self, x:f32, y:f32) -> Result<(usize, usize), FieldError>{
        let accuracy: f32 = 65.0;

        let mut remaining_x: HashSet<i32> = HashSet::from([0,1,2,3,4,5,6,7]);
        let mut remaining_ring: HashSet<i32> = HashSet::from([0,1,2]);

        for (index, spot) in [165.0, 325.0, 485.0, 635.0, 785.0, 945.0, 1105.0].iter().enumerate() {
            let min_border: f32 = (spot - accuracy)*self.window_scale;
            let max_border: f32 = (spot + accuracy)*self.window_scale;

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

    pub fn set_field(&mut self, x:usize, ring:usize, piece_color: Piece) {
        let mut new_board: [[Piece; 3]; 8] = self.board.clone();
        new_board[x][ring] = piece_color;

        self.board = new_board;
    }
    
    pub fn get_state(&mut self) -> State {
        self.state.clone()
    }

    pub fn update_state(&mut self, state: Option<State>) {
        if state.is_some() {
            self.state = state.unwrap();
        } else if self.state == State::Setup && self.setup_pieces_left == 0 {
            self.state = State::Normal;
        } else {

        }
    }

    pub fn reduce_setup_pieces_left(&mut self) {
        self.setup_pieces_left -= 1;
    }
}