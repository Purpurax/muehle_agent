pub mod logic;
pub mod game;

use ggez::conf::{FullscreenType, WindowMode};
use ggez::mint::Vector2;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::graphics::{self, Color, Rect};
use ggez::event::{self, EventHandler};

use crate::engine::game::Piece;

use self::game::{Game, State};


impl EventHandler for game::Game {
    fn update(&mut self, _gtx: &mut Context) -> GameResult {
        
        
        Ok(())
    }
    
    fn draw(&mut self, gtx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(gtx, Color::WHITE);
        
        canvas.draw(
            &self.images["background"],
            graphics::DrawParam::new()
            .scale(Vector2::from_slice(&[self.window_scale; 2]))
            .z(0)
        );

        if self.get_carry_piece().is_some() {
            canvas.draw(
                &self.get_carry_piece().unwrap().3,
                graphics::DrawParam::new()
                .dest_rect(Rect {
                    x: gtx.mouse.position().x - (80.0*self.window_scale),
                    y: gtx.mouse.position().y - (80.0*self.window_scale),
                    h: self.window_scale,
                    w: self.window_scale
                })
                .z(2)
            );
        }
    
        for (x, diagonal_row) in self.get_board().iter().enumerate() {
            for (ring, element) in diagonal_row.iter().enumerate() {
                let image;
                match (self.get_state(), self.get_player_turn(), element) {
                    (State::Setup, Piece::White, Piece::White) => image = self.images["white"].clone(),
                    (State::Setup, Piece::White, Piece::Black) => image = self.images["black"].clone(),
                    (State::Setup, Piece::Black, Piece::Black) => image = self.images["black"].clone(),
                    (State::Setup, Piece::Black, Piece::White) => image = self.images["white"].clone(),
                    (State::Setup, Piece::White, Piece::None) => image = self.images["empty white outlined"].clone(),
                    (State::Setup, Piece::Black, Piece::None) => image = self.images["empty black outlined"].clone(),
                    (State::Normal | State::End, Piece::White, Piece::White) => {
                        if self.get_carry_piece().is_some() {
                            image = self.images["white"].clone();
                        } else {
                            image = self.images["white outlined"].clone();
                        }
                    },
                    (State::Normal | State::End, Piece::White, Piece::Black) => image = self.images["black"].clone(),
                    (State::Normal | State::End, Piece::Black, Piece::Black) => {
                        if self.get_carry_piece().is_some() {
                            image = self.images["black"].clone();
                        } else {
                            image = self.images["black outlined"].clone();
                        }
                    },
                    (State::Normal | State::End, Piece::Black, Piece::White) => image = self.images["white"].clone(),
                    (State::Normal, _, Piece::None) => {
                        let carry_piece = self.get_carry_piece();
                        if carry_piece.is_some() {
                            let (carry_x, carry_ring, _piece_color, _image) = carry_piece.unwrap();
                            if logic::is_neighbor((carry_x, carry_ring), (x, ring)) {
                                image = self.images["outline"].clone();
                            } else { continue }
                        } else { continue }
                    },
                    (State::Take, Piece::White, Piece::White) => image = self.images["white"].clone(),
                    (State::Take, Piece::White, Piece::Black) => image = self.images["take black"].clone(),
                    (State::Take, Piece::Black, Piece::Black) => image = self.images["black"].clone(),
                    (State::Take, Piece::Black, Piece::White) => image = self.images["take white"].clone(),
                    (State::End, _, Piece::None) => {
                        let piece_color = self.get_player_turn();
                        if self.get_piece_count(piece_color) == 3 {
                            let carry_piece = self.get_carry_piece();
                            if carry_piece.is_some() {
                                let (carry_x, carry_ring, _piece_color, _image) = carry_piece.unwrap();
                                if carry_x != x || carry_ring != ring {
                                    image = self.images["outline"].clone();
                                } else { continue }
                            } else { continue }
                        } else {
                            let carry_piece = self.get_carry_piece();
                            if carry_piece.is_some() {
                                let (carry_x, carry_ring, _piece_color, _image) = carry_piece.unwrap();
                                if logic::is_neighbor((carry_x, carry_ring), (x, ring)) {
                                    image = self.images["outline"].clone();
                                } else { continue }
                            } else { continue }
                        }
                    },
                    (State::Win, _, _) => {
                        if self.get_player_turn() == Piece::Black {
                            image = self.images["white"].clone();
                        } else {
                            image = self.images["black"].clone();
                        }
                    },
                    (_, _, _) => continue
                }
                let image_position: Rect = Rect {
                    x: (if [0,6,7].contains(&x) {
                        165.0 + (ring as f32)*160.0
                    } else if [1,5].contains(&x) {
                        635.0
                    } else {
                        1105.0 - (ring as f32)*160.0
                    } -75.0 ) * self.window_scale,
                    y: (if [0,1,2].contains(&x) {
                        165.0 + (ring as f32)*160.0
                    } else if [3,7].contains(&x) {
                        635.0
                    } else {
                        1105.0 - (ring as f32)*160.0
                    } -75.0) * self.window_scale,
                    w: self.window_scale,
                    h: self.window_scale
                };
                
                canvas.draw(
                    &image,
                    graphics::DrawParam::new()
                        .dest_rect(image_position)
                        .z(1)
                );
            }
        }
        


        canvas.finish(gtx)
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {
        
        match self.get_state() {
            State::Setup => {},
            State::Normal | State::End => {
                match self.get_board_indices(x, y) {
                    Ok((board_x, board_ring)) => {
                        let piece_color: Piece = self.get_piece_color(board_x, board_ring);
                        if piece_color == self.get_player_turn() {
                            self.set_field(board_x, board_ring, Piece::None);
                            self.set_carry_piece(Option::Some((board_x, board_ring, piece_color)));
                        } else {
                            if piece_color == Piece::None {
                                println!("Invalid position: {}.x {}.y", x, y);
                            } else if piece_color == Piece::White {
                                println!("Not matching player piece at {}.x {}.y for {}", x, y, "Black");
                            } else {
                                println!("Not matching player piece at {}.x {}.y for {}", x, y, "White");
                            }
                        }
                    },
                    Err(e) => println!("{}", e.message)
                };
            }, State::Take => {},
            State::Win => { }
        }

        Ok(())
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {
        
        match self.get_state() {
            State::Setup => {
                match self.get_board_indices(x, y) {
                    Ok((board_x, board_ring)) => {
                        if self.get_piece_color(board_x, board_ring) == Piece::None {
                            let piece_color = self.get_player_turn();
                            self.set_field(board_x, board_ring, piece_color);
                            self.reduce_setup_pieces_left();
                            
                            if logic::is_creating_mill(piece_color, (board_x, board_ring), self.get_board()) {
                                self.update_state(Option::Some(State::Take));
                            } else {
                                self.next_player_turn();
                                self.update_state(Option::None);
                            }
                        }
                    },
                    Err(e) => {println!("{}", e.message)}
                }
            }, State::Normal => {
                match self.get_board_indices(x, y) {
                    Ok((board_x, board_ring)) => {
                        let carry_piece = self.get_carry_piece();
                        if carry_piece.is_some() {
                            let (carry_x, carry_ring, carry_piece_color, _image) = carry_piece.unwrap();
                            let (successful, new_board) = logic::move_piece(carry_piece_color, (carry_x, carry_ring), (board_x, board_ring), self.get_board());
                            if successful {
                                self.set_board(new_board);
                                self.set_carry_piece(Option::None);

                                if logic::is_creating_mill(carry_piece_color, (board_x, board_ring), self.get_board()) {
                                    self.update_state(Option::Some(State::Take));
                                    println!("{} has created a mill", if carry_piece_color == Piece::White { "White" } else { "Black" });
                                } else {
                                    self.next_player_turn();
                                }
                            } else {
                                println!("Invalid position: {}.x {}.y", x, y);
                                self.undo_carry();
                            }
                        } else {
                            println!("Invalid position: {}.x {}.y", x, y);
                            self.undo_carry();
                        }
                    },
                    Err(e) => {println!("{}", e.message); self.undo_carry();}
                };
            }, State::Take => {
                match self.get_board_indices(x, y) {
                    Ok((board_x, board_ring)) => {
                        // TODO check if piece is in mill
                        let clicked_piece_color = self.get_piece_color(board_x, board_ring);
                        if clicked_piece_color != Piece::None && clicked_piece_color != self.get_player_turn() {
                            self.set_field(board_x, board_ring, Piece::None);
                            self.next_player_turn();
                            self.update_state(Option::Some(State::Normal));
                        } else {
                            println!("Invalid position: {}.x {}.y", x, y);
                            self.undo_carry();
                        }
                    },
                    Err(e) => {println!("{}", e.message); self.undo_carry();}
                };
            }, State::End => {
                match self.get_board_indices(x, y) {
                    Ok((board_x, board_ring)) => {
                        let carry_piece = self.get_carry_piece();
                        if carry_piece.is_some() {
                            let (carry_x, carry_ring, carry_piece_color, _image) = carry_piece.unwrap();
                            let (successful, new_board) = logic::move_piece(carry_piece_color, (carry_x, carry_ring), (board_x, board_ring), self.get_board());
                            if self.get_piece_count(carry_piece_color) == 3 && (board_x != carry_x || board_ring != carry_ring) &&self.get_piece_color(board_x, board_ring) == Piece::None {
                                self.set_board(new_board);
                                self.set_carry_piece(Option::None);

                                if logic::is_creating_mill(carry_piece_color, (board_x, board_ring), self.get_board()) {
                                    self.update_state(Option::Some(State::Take));
                                    println!("{} has created a mill", if carry_piece_color == Piece::White { "White" } else { "Black" });
                                } else {
                                    self.next_player_turn();
                                }
                            } else if successful {
                                self.set_board(new_board);
                                self.set_carry_piece(Option::None);

                                if logic::is_creating_mill(carry_piece_color, (board_x, board_ring), self.get_board()) {
                                    self.update_state(Option::Some(State::Take));
                                    println!("{} has created a mill", if carry_piece_color == Piece::White { "White" } else { "Black" });
                                } else {
                                    self.next_player_turn();
                                }
                            } else {
                                println!("Invalid position: {}.x {}.y", x, y);
                                self.undo_carry();
                            }
                        } else {
                            println!("Invalid position: {}.x {}.y", x, y);
                            self.undo_carry();
                        }
                    },
                    Err(e) => {println!("{}", e.message); self.undo_carry();}
                };
            }, State::Win => { }
        }
        
        Ok(())
    }
}

pub fn run(window_scale: f32) {
    let window_mode = WindowMode {
        width: 1280.0*window_scale,
        height: 1280.0*window_scale,
        fullscreen_type: FullscreenType::Windowed,
        ..Default::default()
    };

    let (mut gtx, event_loop) = ContextBuilder::new("Mühle Agent", "Max Warkentin")
        .add_resource_path("assets")
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context!");


    let game = Game::new(&mut gtx, window_scale);

    event::run(gtx, event_loop, game);
}