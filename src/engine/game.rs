use std::collections::HashMap;

use ggez::{graphics::Image, Context};

use super::{enums::{CarryPiece, Piece, State}, logic};


pub struct Game {
    /* [[left-top-outer, left-top-middle, left-top-inner], [middle-top-outer, middle-top-middle, middle-top-inner] ...]
        0 - empty
        1 - white piece
        2 - black piece
    */
    board: [[Piece; 3]; 8],
    player_turn: Piece,
    carry_piece: Option<CarryPiece>,
    
    // Amount of stones left to be placed by both players
    state: State,
    setup_pieces_left: u8,
    piece_count: HashMap<Piece, u8>,
    
    pub images: HashMap<String, Image>,
    pub window_scale: f32,
    pub play_against_computer: bool
}


impl Game {
    pub fn new(_gtx: &mut Context, window_scale: f32, play_against_computer: bool) -> Game {

        let background_image: Image = Image::from_path(_gtx, "/muehle_board.png").unwrap();
        let piece_white_image: Image = Image::from_path(_gtx, "/muehle_white_piece.png").unwrap();
        let piece_white_outlined_image: Image = Image::from_path(_gtx, "/muehle_white_piece_outlined.png").unwrap();
        let piece_black_image: Image = Image::from_path(_gtx, "/muehle_black_piece.png").unwrap();
        let piece_black_outlined_image: Image = Image::from_path(_gtx, "/muehle_black_piece_outlined.png").unwrap();
        let take_white_image: Image = Image::from_path(_gtx, "/muehle_white_piece_take.png").unwrap();
        let take_black_image: Image = Image::from_path(_gtx, "/muehle_black_piece_take.png").unwrap();
        let empty_white_outlined_image: Image = Image::from_path(_gtx, "/muehle_no_white_piece_outlined.png").unwrap();
        let empty_black_outlined_image: Image = Image::from_path(_gtx, "/muehle_no_black_piece_outlined.png").unwrap();
        let empty_outlined_image: Image = Image::from_path(_gtx, "/muehle_no_piece_outlined.png").unwrap();

        Game {
            board: [[Piece::None; 3]; 8],
            player_turn: Piece::White,
            state: State::Setup,
            setup_pieces_left: 18,
            piece_count: HashMap::from([(Piece::White, 0), (Piece::Black, 0)]),
            images: HashMap::from([
                ("background".to_string(), background_image),
                ("white".to_string(), piece_white_image),
                ("white outlined".to_string(), piece_white_outlined_image),
                ("black".to_string(), piece_black_image),
                ("black outlined".to_string(), piece_black_outlined_image),
                ("take white".to_string(), take_white_image),
                ("take black".to_string(), take_black_image),
                ("empty white outlined".to_string(), empty_white_outlined_image),
                ("empty black outlined".to_string(), empty_black_outlined_image),
                ("outline".to_string(), empty_outlined_image),
            ]),
            window_scale,
            carry_piece: Option::None,
            play_against_computer
        }
    }
    
    pub fn set_example_board(&mut self) {
        let new_board = [
            [Piece::None, Piece::Black, Piece::Black],
            [Piece::None, Piece::None, Piece::Black],
            [Piece::None, Piece::None, Piece::Black],
            [Piece::None, Piece::None, Piece::Black],
            [Piece::Black, Piece::None, Piece::Black],
            [Piece::White, Piece::White, Piece::White],
            [Piece::None, Piece::None, Piece::None],
            [Piece::None, Piece::None, Piece::None],
        ];
        self.board = new_board;
        self.player_turn = Piece::White;
        self.setup_pieces_left = 0;
        self.state = State::End;
        self.update_piece_count();
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

    pub fn set_setup_pieces_left(&mut self, amount: u8) {
        self.setup_pieces_left = amount;
    }

    pub fn get_setup_pieces_left(&mut self) -> u8 {
        self.setup_pieces_left
    }

    pub fn get_carry_piece(&mut self) -> Option<CarryPiece> {
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
        self.carry_piece = Option::Some(CarryPiece::new(x, ring, piece_color, image));
    }

    pub fn undo_carry(&mut self) {
        if self.get_carry_piece().is_some() {
            let (x, ring, piece_color, _image) = self.get_carry_piece().unwrap().into();
            self.board[x][ring] = piece_color;
            self.set_carry_piece(Option::None);
        }
    }

    fn update_piece_count(&mut self) {
        let mut count_white: u8 = 0;
        let mut count_black: u8 = 0;
        for pack in self.board.iter() {
            for element in pack.iter() {
                if *element == Piece::White {
                    count_white += 1;
                } else if *element == Piece::Black {
                    count_black += 1;
                }
            }
        }
        self.piece_count.insert(Piece::White, count_white);
        self.piece_count.insert(Piece::Black, count_black);
    }

    pub fn get_piece_count(&mut self, piece_color: Piece) -> u8 {
        return self.piece_count[&piece_color];
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
        self.update_piece_count();
        if state.is_some() {
            self.state = state.unwrap();
        }
        if self.state != State::Take {
            if self.setup_pieces_left > 0 {
                self.state = State::Setup;
            } else if self.piece_count[&Piece::White] < 3 || self.piece_count[&Piece::Black] < 3 || logic::is_soft_locked(self.get_player_turn(), self.get_board()) {
                self.state = State::Win;
            } else if self.piece_count[&Piece::White] == 3 || self.piece_count[&Piece::Black] == 3 {
                self.state = State::End;
            } else {
                self.state = State::Normal;
            }
        }
    }

    pub fn reduce_setup_pieces_left(&mut self) {
        let setup_pieces_left = self.setup_pieces_left - 1;
        self.setup_pieces_left = setup_pieces_left;
        self.update_piece_count();

        let current_player_count: u8 = setup_pieces_left / 2;
        let next_player_count: u8 = setup_pieces_left / 2 + (setup_pieces_left % 2);
        println!("{} can place {} more pieces and {} can place {} more pieces",
            Piece::White.to_str(), current_player_count,
            Piece::Black.to_str(), next_player_count);
    }

    pub fn refresh(&mut self) {
        self.update_piece_count();
        self.update_state(Option::None);
    }
}