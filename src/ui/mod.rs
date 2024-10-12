pub mod rendering;

use good_web_game::graphics::Color;
use good_web_game as ggez;
use ggez::{event, graphics, GameError, GameResult, Context};
use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::cgmath::{Point2, Vector2};

use miniquad::GraphicsContext;
use std::collections::HashMap;
use Option;

use crate::core::enums::{State, Difficulty};
use crate::core::game::Game;
use crate::core::logic::{coords_to_board_position, coords_to_bottom_panel_position, is_restart_clicked, compute_bottom_panel, compute_button_down, compute_button_up, compute_computer_step};
use crate::ai;

pub struct Engine {
    game: Game,

    computer_white: Difficulty,
    computer_black: Difficulty,
    
    images: HashMap<String, Image>,
    offsets: Point2<f32>,
    scales: Vector2<f32>,
    force_draw: bool,
}

impl Engine {
    pub fn new(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult<Engine> {
        let game = Game::new();
        
        let images = Engine::load_images(ctx, quad_ctx);

        let (window_width, window_height) = graphics::drawable_size(quad_ctx);
        let offsets = Engine::calculate_offsets(window_width, window_height);
        let scales = Engine::calculate_scale(window_width, window_height);

        Ok(Engine {
            game,
            computer_white: Difficulty::Off,
            computer_black: Difficulty::Off,
            images,
            offsets,
            scales,
            force_draw: false,
        })
    }

    fn load_images(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> HashMap<String, Image> {
        let board: Image = Image::new(ctx, quad_ctx, "/assets/muehle_board.png").unwrap();
        let piece_white: Image = Image::new(ctx, quad_ctx, "/assets/muehle_white_piece.png").unwrap();
        let piece_white_outlined: Image = Image::new(ctx, quad_ctx, "/assets/muehle_white_piece_outlined.png").unwrap();
        let piece_black: Image = Image::new(ctx, quad_ctx, "/assets/muehle_black_piece.png").unwrap();
        let piece_black_outlined: Image = Image::new(ctx, quad_ctx, "/assets/muehle_black_piece_outlined.png").unwrap();
        let take_white: Image = Image::new(ctx, quad_ctx, "/assets/muehle_white_piece_take.png").unwrap();
        let take_black: Image = Image::new(ctx, quad_ctx, "/assets/muehle_black_piece_take.png").unwrap();
        let empty_white_outlined: Image = Image::new(ctx, quad_ctx, "/assets/muehle_no_white_piece_outlined.png").unwrap();
        let empty_black_outlined: Image = Image::new(ctx, quad_ctx, "/assets/muehle_no_black_piece_outlined.png").unwrap();
        let empty_outlined: Image = Image::new(ctx, quad_ctx, "/assets/muehle_no_piece_outlined.png").unwrap();
        
        let mut images = HashMap::from([
            ("board".to_string(), board),
            ("white".to_string(), piece_white),
            ("white outlined".to_string(), piece_white_outlined),
            ("black".to_string(), piece_black),
            ("black outlined".to_string(), piece_black_outlined),
            ("take white".to_string(), take_white),
            ("take black".to_string(), take_black),
            ("empty white outlined".to_string(), empty_white_outlined),
            ("empty black outlined".to_string(), empty_black_outlined),
            ("outline".to_string(), empty_outlined),
        ]);

        for i in 0..19 {
            let key = format!("bottom panel setup {}", i);
            let value = Image::new(ctx, quad_ctx, &format!("/assets/muehle_bottom_panel/muehle_bottom_panel_setup_{}.png", i)).unwrap();
            images.insert(key, value);
        }
        for i in 0..16 {
            let key = format!("bottom panel {}", i);
            let value = Image::new(ctx, quad_ctx, &format!("/assets/muehle_bottom_panel/muehle_bottom_panel_{}.png", i)).unwrap();
            images.insert(key, value);
        }

        images
    }
    
    fn calculate_offsets(window_width: f32, window_height: f32) -> Point2<f32> {
        const GAME_IMAGES_WIDTH: f32 = 1280.0;
        const GAME_IMAGES_HEIGHT: f32= 1280.0 + 80.0 + 240.0;

        let scale: Vector2<f32> = Engine::calculate_scale(window_width, window_height);

        let offset_x: f32 = (window_width - GAME_IMAGES_WIDTH * scale.x) / 2.0;
        let offset_y: f32 = (window_height - GAME_IMAGES_HEIGHT * scale.y) / 2.0;

        Point2::new(offset_x, offset_y)
    }

    fn calculate_scale(window_width: f32, window_height: f32) -> Vector2<f32> {
        const GAME_IMAGES_WIDTH: f32 = 1280.0;
        const GAME_IMAGES_HEIGHT: f32= 1280.0 + 80.0 + 240.0;

        let window_ratio: f32 = window_width / window_height;
        let game_images_ratio: f32 = GAME_IMAGES_WIDTH / GAME_IMAGES_HEIGHT;

        let scale: f32 =
            if window_ratio > game_images_ratio {
                window_height / GAME_IMAGES_HEIGHT
            } else {
                window_width / GAME_IMAGES_WIDTH
            };

        Vector2::new(scale, scale)
    }
}

impl EventHandler<GameError> for Engine {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult {
        if self.force_draw {
            self.force_draw = false;
            return Ok(())
        }
        if self.game.get_state() == State::Win {
            return Ok(())
        }

        let action =
            if self.computer_white != Difficulty::Off && self.game.get_player_turn() == 0b11 {
                ai::compute_step(&self.game, self.computer_white)
            } else if self.computer_black != Difficulty::Off && self.game.get_player_turn() == 0b10 {
                ai::compute_step(&self.game, self.computer_black)
            } else {
                Option::None
            };
        
        if action.is_some() {
            match compute_computer_step(action.unwrap(), &mut self.game) {
                Ok(()) => {},
                Err(_) => {
                    println!("Invalid move from computer");
                    self.game.update_state(Option::None);
                    self.force_draw = true;
                }
            }
        }

        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult {
        /* Background */
        graphics::clear(ctx, quad_ctx, Color::from_rgb_u32(0x3F2832));
        
        let param: DrawParam = DrawParam::new().dest(self.offsets).scale(self.scales);
        graphics::draw(ctx, quad_ctx, &self.images["board"], param)?;

        /* Bottom Panel Setup */
        let image: Image = rendering::calculate_bottom_panel_setup_image(&self.images, self.game.get_setup_pieces_left());
        let dest: Point2<f32> = Point2::new(
            self.offsets.x,
            self.offsets.y + self.images["board"].dimensions().h * self.scales.y);
        
        let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
        graphics::draw( ctx, quad_ctx, &image, param)?;

        /* Bottom Panel */
        let image: Image = rendering::calculate_bottom_panel_image(&self.images, self.computer_white, self.computer_black);
        let dest: Point2<f32> = Point2::new(
            self.offsets.x,
            self.offsets.y + (self.images["board"].dimensions().h + self.images["bottom panel setup 0"].dimensions().h) * self.scales.y);
        
        let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
        graphics::draw(ctx, quad_ctx, &image, param)?;
    
        /* Drawing on each field a piece or a marker */
        for position in 0..24 {
            let comp_white: bool = self.computer_white != Difficulty::Off;
            let comp_black: bool = self.computer_black != Difficulty::Off;
            let image = rendering::calculate_image(&self.game, position, &self.images, comp_white, comp_black);
            if image.is_none() {
                continue
            }

            let dest: Point2<f32> = rendering::calculate_image_position(position, self.offsets, self.scales);
            
            let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
            graphics::draw(ctx, quad_ctx, &image.unwrap(), param)?;
        }
        
        /* Drawing Piece which is Grabbed */
        let carry_piece = self.game.get_carry_piece();
        if carry_piece.is_some() {
            let image: Image;
            match carry_piece.unwrap().color {
                0b11 => image = self.images["white"].clone(),
                0b10 => image = self.images["black"].clone(),
                _ => panic!("Carry piece has no color"),
            }
            let dest: Point2<f32> = ctx.mouse_context.mouse_position() - (80.0*self.scales);

            let param: DrawParam = DrawParam::new().dest(dest).scale(self.scales);
            graphics::draw(ctx, quad_ctx, &image, param)?;
        }


        graphics::present(ctx, quad_ctx)
    }

    fn resize_event(
            &mut self,
            ctx: &mut Context,
            _quad_ctx: &mut GraphicsContext,
            width: f32,
            height: f32,
        ) {
        self.offsets = Engine::calculate_offsets(width, height);
        self.scales = Engine::calculate_scale(width, height);
        ctx.gfx_context.set_screen_coordinates(Rect::new(0.0, 0.0, width, height));
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _quad_ctx: &mut GraphicsContext,
            _button: event::MouseButton,
            x: f32,
            y: f32) {
        let logical_x: f32 = (x - self.offsets.x) / self.scales.x;
        let logical_y: f32 = (y - self.offsets.y) / self.scales.y;

        /* bottom panel */
        match coords_to_bottom_panel_position(logical_x, logical_y) {
            Ok(index) => {
                (self.computer_white, self.computer_black) = compute_bottom_panel(self.computer_white, self.computer_black, index);
                self.force_draw = true;
                return
            },
            Err(_) => {}
        }

        /* restart button */
        if is_restart_clicked(logical_x, logical_y) {
            self.game = Game::new();
            self.force_draw = true;
            println!("Game restarted");
            return
        }

        /* game board */
        if self.computer_white != Difficulty::Off && self.game.get_player_turn() == 0b11 || 
            self.computer_black != Difficulty::Off && self.game.get_player_turn() == 0b10 {
            return
        }

        match coords_to_board_position(logical_x, logical_y) {
            Ok(position) =>
                if compute_button_down(position, &mut self.game).is_err() {
                    println!("Invalid move to {}.logical_x {}.logical_y from player {}", logical_x, logical_y, self.game.get_player_turn());
                },
            Err(e) => {
                self.game.undo_carry();
                println!("{}", e.message);
            }
        };
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            _quad_ctx: &mut GraphicsContext,
            _button: event::MouseButton,
            x: f32,
            y: f32) {
        let logical_x: f32 = (x - self.offsets.x) / self.scales.x;
        let logical_y: f32 = (y - self.offsets.y) / self.scales.y;

        /* game board */
        if self.computer_white != Difficulty::Off && self.game.get_player_turn() == 0b11 || 
            self.computer_black != Difficulty::Off && self.game.get_player_turn() == 0b10 || self.force_draw {
            return
        }

        let position = 
            match coords_to_board_position(logical_x, logical_y) {
                Ok(val) => val,
                Err(e) => {
                    self.game.undo_carry();
                    return println!("{}", e.message);
                }
            };
        
        if compute_button_up(position, &mut self.game).is_err() {
            return println!("Invalid move to {}.logical_x {}.logical_y from player {}", logical_x, logical_y, self.game.get_player_turn());
        }

        self.force_draw = true;
    }
}