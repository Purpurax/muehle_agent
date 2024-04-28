pub mod logic;
pub mod game;
pub mod rendering;
pub mod enums;

use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::mint::Vector2;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Rect};
use ggez::event::{self, EventHandler};

use self::game::Game;


impl EventHandler for Game {
    fn update(&mut self, _gtx: &mut Context) -> GameResult {
        Ok(())
    }
    
    fn draw(&mut self, gtx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(gtx, Color::WHITE);
        
        /* Drawing Background */
        canvas.draw(
            &self.images["background"],
            DrawParam::new()
            .scale(Vector2::from_slice(&[self.window_scale; 2]))
            .z(0)
        );
        
        /* Drawing Piece which is Grabbed (optional) */
        if self.get_carry_piece().is_some() {
            canvas.draw(
                &self.get_carry_piece().unwrap().image,
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
        for (x, diagonal_row) in self.get_board().iter().enumerate() {
            for (ring, element) in diagonal_row.iter().enumerate() {
                let player_color = self.get_player_turn();

                let image = rendering::calculate_image(self.get_state(), player_color, *element, (x, ring), self.get_board(), self.get_carry_piece(), self.get_piece_count(player_color), self.images.clone());
                if image.is_none() {
                    continue
                }

                let image_position: Rect = rendering::calculate_image_position(x, ring, self.window_scale);
                
                canvas.draw(
                    &image.unwrap(),
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
        
        
        match logic::compute_step(true, x, y, self) {
            Ok(()) => {},
            Err(_e) => println!("Invalid move to {}.x {}.y from player {}", x, y, self.get_player_turn().to_str())
        };

        Ok(())
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {

        match logic::compute_step(false, x, y, self) {
            Ok(()) => {},
            Err(_e) => println!("Invalid move to {}.x {}.y from player {}", x, y, self.get_player_turn().to_str())
        };
        
        self.update_state(Option::None);

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
    let window_setup = WindowSetup {
        title: "Mühle Player vs Player ©Purpurax".to_string(),
        icon: "/muehle_board_icon.png".to_string(),
        ..Default::default()
    };

    let (mut gtx, event_loop) = ContextBuilder::new("Mühle Agent", "Max Warkentin")
        .add_resource_path("assets")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("Could not create ggez context!");


    let game = Game::new(&mut gtx, window_scale);

    // Uncomment for debugging:
    // let game = Game::new(&mut gtx, window_scale).game.set_testing_board();

    event::run(gtx, event_loop, game);
}