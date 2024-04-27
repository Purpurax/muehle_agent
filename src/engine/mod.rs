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
                    (State::Normal, Piece::White, Piece::White) => image = self.images["white outlined"].clone(),
                    (State::Normal, Piece::White, Piece::Black) => image = self.images["black"].clone(),
                    (State::Normal, Piece::Black, Piece::Black) => image = self.images["black outlined"].clone(),
                    (State::Normal, Piece::Black, Piece::White) => image = self.images["white"].clone(),
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
            State::Normal => {
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
            }, State::Take => {

            }, State::Win => {

            }
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
                            self.next_player_turn();
                            self.reduce_setup_pieces_left();
                            self.update_state(Option::None);
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
                                self.next_player_turn();
                                self.set_carry_piece(Option::None);
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

            }, State::Win => {

            }
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