use iced::widget::{button, column, text};
mod configreader;
mod modules;

fn main() -> iced::Result {
    iced::run("", Bar::update, Bar::view)
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

#[derive(Default)]
struct Bar {
    counter: usize,
}

impl Bar {
    fn update(&mut self, message: Message) {
        match message {
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
}
