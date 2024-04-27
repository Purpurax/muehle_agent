use std::path::{self, Path};

use crate::engine;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image};
use ggez::event::{self, EventHandler};

pub fn render() {
    let (mut ctx, event_loop) = ContextBuilder::new("Mühle Agent", "Max Warkentin")
        .add_resource_path("resources")
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    image: Image
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let image = Image::from_path(_ctx, Path::new("/assets/muehle_board.png")).unwrap();
        MyGame {
            image: image
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        
        canvas.draw(&self.image, graphics::DrawParam::new());

        canvas.finish(ctx)
    }
}