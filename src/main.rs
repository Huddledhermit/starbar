use iced::widget::{button, column, text};
use::iced::time;
use iced::Subscription;
mod configreader;
mod modules;

fn main() -> iced::Result {

    iced::run("", Bar::update, Bar::view)
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    Update
}

#[derive(Default)]
struct Bar {
    counter: usize,
}

impl Bar {
    fn update(&mut self, message: Message) {
        match Message {
            Message::ButtonPressed => self.counter += 1,
        }
    }

    fn view(&self) -> iced::Element<Message> {

        column![
            text(&self.counter),
            button("Increase").on_press(Message::ButtonPressed),
        ]
        .into()
    }

    fn Subscription(stat::){}

    }
}
