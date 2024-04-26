pub mod logic;
pub mod game;

use std::io::Cursor;

use ggez::conf::{FullscreenType, WindowMode};
use ggez::mint::Vector2;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::graphics::{self, Color, Rect};
use ggez::event::{self, EventHandler};

use crate::engine::game::Piece;

use self::game::Game;


impl EventHandler for game::Game {
    fn update(&mut self, _gtx: &mut Context) -> GameResult {
        
        
        Ok(())
    }
    
    fn draw(&mut self, gtx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(gtx, Color::WHITE);
        
        canvas.draw(
            &self.images[0],
            graphics::DrawParam::new()
            .scale(Vector2::from_slice(&[self.window_scale; 2]))
        );

        if self.get_carry_piece().1.is_some() {
            canvas.draw(
                &self.get_carry_piece().1.unwrap(),
                graphics::DrawParam::new()
                .dest_rect(Rect {
                    x: gtx.mouse.position().x - (80.0*self.window_scale),
                    y: gtx.mouse.position().y - (80.0*self.window_scale),
                    h: self.window_scale,
                    w: self.window_scale
                })
            );
        }
    
        for (x, diagonal_row) in self.get_board().iter().enumerate() {
            for (ring, element) in diagonal_row.iter().enumerate() {
                let image;
                match element {
                    Piece::White => image = self.images[1].clone(),
                    Piece::Black => image = self.images[2].clone(),
                    Piece::None => continue
                }
                let image_position: Rect = Rect {
                    x: (if [0,6,7].contains(&x) {
                        165.0 + (ring as f32)*160.0
                    } else if [1,5].contains(&x) {
                        635.0
                    } else {
                        1105.0 - (ring as f32)*160.0
                    } -80.0 ) * self.window_scale,
                    y: (if [0,1,2].contains(&x) {
                        165.0 + (ring as f32)*160.0
                    } else if [3,7].contains(&x) {
                        635.0
                    } else {
                        1105.0 - (ring as f32)*160.0
                    } -80.0) * self.window_scale,
                    w: self.window_scale,
                    h: self.window_scale
                };
                
                canvas.draw(
                    &image,
                    graphics::DrawParam::new()
                        .dest_rect(image_position)
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
        
        match self.get_board_indices(x, y) {
            Ok((board_x, board_ring)) => {
                let piece_color: Piece = self.get_piece_color(board_x, board_ring);
                if piece_color == self.get_player_turn() {
                    self.clear_field(board_x, board_ring);
                    
                }
            },
            Err(e) => println!("{}", e.message)
        };

        Ok(())
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), GameError> {
        

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


    let mut game = Game::new(&mut gtx, window_scale);

    game.set_example_board();


    

    event::run(gtx, event_loop, game);
}