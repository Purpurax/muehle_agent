use crate::engine;

use iced::{
    advanced::graphics::gradient::Linear, color, widget::{image, Button, Column, Container, Image, Text}, Alignment, Element, Length, Sandbox, Settings, Size
};

struct Counter {
    count: i32
}

#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    Increment,
    Decrement
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Counter{ count: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count += 1
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let background_image = Image::<image::Handle>::new("assets/muehle_board.png")
            .width(Length::Fill)
            .height(Length::Fill);

        let background: Container<'_, CounterMessage, iced::Theme, iced::Renderer> = Container::new(background_image)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let black_piece_image = Image::<image::Handle>::new("assets/muehle_black_piece.png")
            .width(Length::Fill)
            .height(Length::Fill);

        let black_piece: Container<'_, CounterMessage, iced::Theme, iced::Renderer> = Container::new(black_piece_image)
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0));

        // let label = Text::new(format!("Count: {}", self.count));
        // let incr = Button::new("Increment").on_press(CounterMessage::Increment);
        // let decr = Button::new("Decrement").on_press(CounterMessage::Decrement);
        // let col = Column::new().push(incr).push(label).push(decr);
        // Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()

        Container::new(background).into()
    }

}

pub fn render(game: engine::Game)  -> Result<(), iced::Error> {
    // TODO use game
    
    let mut settings = Settings::default();
    settings.window.size = Size::new(512.0, 512.0);
    Counter::run(settings)
}
// inspiration from https://nikolish.in/gs-with-iced-1