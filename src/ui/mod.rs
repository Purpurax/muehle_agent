pub mod rendering;
pub mod snapshot;

use std::collections::HashMap;
use Option;

use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::mint::Vector2;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Image, Rect};
use ggez::event::{self, EventHandler, EventLoop};

use crate::core::enums::State;
use crate::core::game::Game;
use crate::core::logic::{self, calculate_position, compute_button_down, compute_button_up};
use crate::ai;

pub struct Engine {
    pub game: Game,

    computer_white: bool,
    computer_black: bool,
    
    pub images: HashMap<String, Image>,
    pub window_scale: f32,
    force_draw: bool, // draw before update
    gtx: Option<Context>,
    event_loop: Option<EventLoop<()>>,
}

impl Engine {
    pub fn new(window_scale: f32, computer_white: bool, computer_black: bool, asset_folder: &str) -> Engine {
        let game = Game::new();
        
        let (mut gtx, event_loop) = Engine::create_window(window_scale, asset_folder);
        let images = Engine::load_images(&mut gtx);

        Engine {
            game,
            computer_white,
            computer_black,
            images,
            window_scale,
            force_draw: false,
            gtx: Option::Some(gtx),
            event_loop: Option::Some(event_loop),
        }
    }
    
    fn create_window(window_scale: f32, asset_folder: &str) -> (ggez::Context, EventLoop<()>) {
        let window_mode = WindowMode {
            width: 1280.0*window_scale,
            height: 1280.0*window_scale,
            fullscreen_type: FullscreenType::Windowed,
            ..Default::default()
        };
        let window_setup = WindowSetup {
            title: "Mühle AI".to_string(),
            icon: "/muehle_board_icon.png".to_string(),
            ..Default::default()
        };
        
        let (gtx, event_loop) = ContextBuilder::new("Mühle Agent", "Purpurax")
            .add_resource_path(asset_folder)
            .window_mode(window_mode)
            .window_setup(window_setup)
            .build()
            .expect("Could not create ggez context!");
    
        return (gtx, event_loop);
    }

    fn load_images(gtx: &mut Context) -> HashMap<String, Image> {
        let background: Image = Image::from_path(gtx, "/muehle_board.png").unwrap();
        let piece_white: Image = Image::from_path(gtx, "/muehle_white_piece.png").unwrap();
        let piece_white_outlined: Image = Image::from_path(gtx, "/muehle_white_piece_outlined.png").unwrap();
        let piece_black: Image = Image::from_path(gtx, "/muehle_black_piece.png").unwrap();
        let piece_black_outlined: Image = Image::from_path(gtx, "/muehle_black_piece_outlined.png").unwrap();
        let take_white: Image = Image::from_path(gtx, "/muehle_white_piece_take.png").unwrap();
        let take_black: Image = Image::from_path(gtx, "/muehle_black_piece_take.png").unwrap();
        let empty_white_outlined: Image = Image::from_path(gtx, "/muehle_no_white_piece_outlined.png").unwrap();
        let empty_black_outlined: Image = Image::from_path(gtx, "/muehle_no_black_piece_outlined.png").unwrap();
        let empty_outlined: Image = Image::from_path(gtx, "/muehle_no_piece_outlined.png").unwrap();

        HashMap::from([
            ("background".to_string(), background),
            ("white".to_string(), piece_white),
            ("white outlined".to_string(), piece_white_outlined),
            ("black".to_string(), piece_black),
            ("black outlined".to_string(), piece_black_outlined),
            ("take white".to_string(), take_white),
            ("take black".to_string(), take_black),
            ("empty white outlined".to_string(), empty_white_outlined),
            ("empty black outlined".to_string(), empty_black_outlined),
            ("outline".to_string(), empty_outlined),
            ])
    }

    fn get_gtx(&mut self) -> Context {
        self.gtx.take().unwrap()
    }

    fn get_event_loop(&mut self) -> EventLoop<()> {
        self.event_loop.take().unwrap()
    }

    pub fn run(mut self) {
        let gtx = self.get_gtx();
        let event_loop = self.get_event_loop();
        event::run(gtx, event_loop, self);
    }
}

impl EventHandler for Engine {
    fn update(&mut self, _gtx: &mut Context) -> GameResult {
        if self.force_draw {
            self.force_draw = false;
            return Ok(())
        }
        if self.game.get_state() != State::Win &&
            ((self.computer_white && self.game.get_player_turn() == 0b11) ||
            (self.computer_black && self.game.get_player_turn() == 0b10)) {
            match ai::compute_step(&self.game) {
                Option::Some(action) => 
                match logic::compute_computer_step(action, &mut self.game) {
                    Ok(()) => {},
                    Err(_) => {
                        println!("Invalid move from computer");
                        self.game.update_state(Option::None);
                        self.force_draw = true;
                    }
                },
                Option::None => println!("AI computation lead to no action in update"),
            };
        }

        Ok(())
    }
    
    fn draw(&mut self, gtx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(gtx, Color::WHITE);
        
        /* Background */
        canvas.draw(
            &self.images["background"],
            DrawParam::new()
            .scale(Vector2::from_slice(&[self.window_scale; 2]))
            .z(0)
        );
        
        /* Drawing Piece which is Grabbed (optional) */
        if self.game.get_carry_piece().is_some() {
            let image: Image;
            match self.game.get_carry_piece().unwrap().color {
                0b11 => image = self.images["white"].clone(),
                0b10 => image = self.images["black"].clone(),
                _ => panic!("Carry piece has no color"),
            }
            canvas.draw(
                &image,
                DrawParam::new()
                .dest_rect(Rect {
                    x: gtx.mouse.position().x - (80.0*self.window_scale),
                    y: gtx.mouse.position().y - (80.0*self.window_scale),
                    h: self.window_scale,
                    w: self.window_scale })
                .z(2)
            );
        }
    
        /* Drawing on each field a piece or a marker */
        for position in 0..24 {
            let image = rendering::calculate_image(&self.game, position, self.images.clone());
            if image.is_none() {
                continue
            }

            let image_position: Rect = rendering::calculate_image_position(position as u8, self.window_scale);
            
            canvas.draw(
                &image.unwrap(),
                graphics::DrawParam::new()
                    .dest_rect(image_position)
                    .z(1)
            );
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
        
        if self.computer_white && self.game.get_player_turn() == 0b11 || 
            self.computer_black && self.game.get_player_turn() == 0b10 {
            return Ok(())
        }

        
        let position = 
            match calculate_position(x, y, self.window_scale) {
                Ok(val) => val,
                Err(e) => {
                    self.game.undo_carry();
                    return Ok(println!("{}", e.message));
                }
            };
        
        if compute_button_down(position, &mut self.game).is_err() {
            return Ok(println!("Invalid move to {}.x {}.y from player {}", x, y, self.game.get_player_turn()));
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

        if self.computer_white && self.game.get_player_turn() == 0b11 || 
            self.computer_black && self.game.get_player_turn() == 0b10 {
            return Ok(())
        }

        
        let position = 
            match calculate_position(x, y, self.window_scale) {
                Ok(val) => val,
                Err(e) => {
                    self.game.undo_carry();
                    return Ok(println!("{}", e.message));
                }
            };
        
        if compute_button_up(position, &mut self.game).is_err() {
            return Ok(println!("Invalid move to {}.x {}.y from player {}", x, y, self.game.get_player_turn()));
        }

        self.force_draw = true;

        Ok(())
    }
}